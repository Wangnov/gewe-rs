//! wait-reply 命令模块
//!
//! 发送消息后等待特定用户的回复，支持私聊和群聊场景。

use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::Args;
use gewe_core::{AppId, BotContext};
use gewe_http::GeweHttpClient;
use gewe_session::{InMemorySessionStore, SessionStore};
use gewe_webhook::{router_with_channel_and_state, WebhookBuilderOptions, WebhookEvent};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::net::TcpListener as StdTcpListener;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, UnixListener, UnixStream};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::time;
use tracing::{debug, error, info, warn};

/// wait-reply 命令参数
#[derive(Args, Clone)]
pub struct WaitReplyArgs {
    /// 发送目标用户 wxid
    #[arg(long)]
    pub to_wxid: String,

    /// webhook 监听地址（如 0.0.0.0:4399）
    #[arg(long, short = 'l')]
    pub listen: String,

    /// 群 ID（群消息场景，以 @chatroom 结尾）
    #[arg(long)]
    pub group_wxid: Option<String>,

    /// 过滤发送者 wxid（默认同 --to-wxid）
    #[arg(long)]
    pub filter_wxid: Option<String>,

    /// 正则匹配回复内容
    #[arg(long, short = 'm')]
    pub r#match: Option<String>,

    /// 超时秒数（默认无限等待）
    #[arg(long, short = 't')]
    pub timeout: Option<u64>,

    /// 输出格式：text / json
    #[arg(long, short = 'o', default_value = "text")]
    pub output_format: OutputFormat,

    /// 发送的消息（可多次指定），格式：TYPE:CONTENT
    #[arg(long, short = 'M')]
    pub message: Vec<String>,

    /// API Token
    #[arg(long)]
    pub token: Option<String>,

    /// App ID
    #[arg(long)]
    pub app_id: Option<String>,

    /// Bot App ID
    #[arg(long)]
    pub bot_app_id: Option<String>,

    /// Bot 别名
    #[arg(long)]
    pub bot_alias: Option<String>,

    /// API Base URL
    #[arg(long)]
    pub base_url: Option<String>,
}

/// 输出格式
#[derive(Clone, Debug, Default, PartialEq)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("无效的输出格式: {}，支持 text 或 json", s)),
        }
    }
}

/// 退出码
#[derive(Debug, Clone, Copy)]
pub enum ExitStatus {
    /// 成功收到匹配的回复
    Success = 0,
    /// 超时未收到回复
    Timeout = 1,
    /// 发送消息失败
    SendFailed = 2,
    /// webhook 启动失败
    WebhookFailed = 3,
}

/// 要发送的消息
#[derive(Debug, Clone)]
pub enum WaitReplyMessage {
    Text(String),
    Image(String),
    Voice(String),
    Video(String),
    Link {
        title: String,
        desc: String,
        url: String,
        thumb_url: String,
    },
}

impl WaitReplyMessage {
    /// 从 TYPE:CONTENT 格式解析消息
    pub fn parse(s: &str) -> Result<Self> {
        let (msg_type, content) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("消息格式错误，应为 TYPE:CONTENT"))?;

        match msg_type.to_lowercase().as_str() {
            "text" => Ok(Self::Text(content.to_string())),
            "image" => Ok(Self::Image(content.to_string())),
            "voice" => Ok(Self::Voice(content.to_string())),
            "video" => Ok(Self::Video(content.to_string())),
            "link" => {
                // 格式：标题|描述|URL|缩略图URL
                let parts: Vec<&str> = content.split('|').collect();
                if parts.len() != 4 {
                    return Err(anyhow!(
                        "链接消息格式错误，应为 link:标题|描述|URL|缩略图URL"
                    ));
                }
                Ok(Self::Link {
                    title: parts[0].to_string(),
                    desc: parts[1].to_string(),
                    url: parts[2].to_string(),
                    thumb_url: parts[3].to_string(),
                })
            }
            _ => Err(anyhow!(
                "未知消息类型: {}，支持 text/image/voice/video/link",
                msg_type
            )),
        }
    }
}

/// 收到的回复消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedReply {
    pub from_wxid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_wxid: Option<String>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

/// 广播消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastMessage {
    #[serde(rename = "message")]
    Message { data: ReceivedReply },
    #[serde(rename = "shutdown")]
    Shutdown { reason: String },
}

/// wait-reply 运行时状态
struct WaitReplyState {
    /// 过滤条件
    filter_wxid: String,
    group_wxid: Option<String>,
    match_regex: Option<Regex>,
    /// 已收到的消息
    received: Vec<ReceivedReply>,
    /// 是否已匹配成功
    matched: bool,
}

impl WaitReplyState {
    fn new(
        filter_wxid: String,
        group_wxid: Option<String>,
        match_pattern: Option<&str>,
    ) -> Result<Self> {
        let match_regex = match match_pattern {
            Some(pat) if !pat.is_empty() => Some(Regex::new(pat)?),
            _ => None,
        };
        Ok(Self {
            filter_wxid,
            group_wxid,
            match_regex,
            received: Vec::new(),
            matched: false,
        })
    }

    /// 处理收到的消息，返回是否匹配成功
    fn handle_message(&mut self, reply: ReceivedReply) -> bool {
        // 检查是否匹配
        let is_match = match &self.match_regex {
            Some(re) => re.is_match(&reply.content),
            None => true, // 未指定 --match 时，收到第一条即返回
        };

        self.received.push(reply);

        if is_match {
            self.matched = true;
        }

        is_match
    }

    /// 检查消息是否满足过滤条件
    fn should_accept(&self, event: &WebhookEvent) -> Option<ReceivedReply> {
        // 只处理 AddMsg 类型的消息
        if event.type_name.as_deref() != Some("AddMsg") {
            return None;
        }

        // 只处理文本消息（MsgType=1）
        let msg_type = event.data.get("MsgType").and_then(|v| v.as_i64())?;
        if msg_type != 1 {
            return None;
        }

        // 提取 FromUserName
        let from_wxid = event
            .data
            .get("FromUserName")
            .and_then(|v| v.get("string"))
            .and_then(|v| v.as_str())?
            .to_string();

        // 提取 Content
        let raw_content = event
            .data
            .get("Content")
            .and_then(|v| v.get("string"))
            .and_then(|v| v.as_str())?
            .to_string();

        // 群聊/私聊场景判断
        let is_group = from_wxid.ends_with("@chatroom");

        if let Some(ref expected_group) = self.group_wxid {
            // 群聊模式
            if !is_group || &from_wxid != expected_group {
                return None;
            }

            // 从 content 提取发送者
            let (sender_wxid, content) = extract_group_sender_and_content(&raw_content)?;
            if sender_wxid != self.filter_wxid {
                return None;
            }

            Some(ReceivedReply {
                from_wxid: sender_wxid,
                group_wxid: Some(from_wxid),
                content,
                timestamp: Utc::now(),
            })
        } else {
            // 私聊模式
            if is_group {
                return None;
            }
            if from_wxid != self.filter_wxid {
                return None;
            }

            Some(ReceivedReply {
                from_wxid,
                group_wxid: None,
                content: raw_content,
                timestamp: Utc::now(),
            })
        }
    }

    /// 检查回复是否满足过滤条件（用于订阅者模式）
    fn should_accept_reply(&self, reply: &ReceivedReply) -> bool {
        // 检查 group_wxid
        if self.group_wxid != reply.group_wxid {
            return false;
        }
        // 检查 filter_wxid
        reply.from_wxid == self.filter_wxid
    }
}

/// 从群消息 content 提取发送者和实际内容
fn extract_group_sender_and_content(content: &str) -> Option<(String, String)> {
    let trimmed = content.trim_start();
    // 群聊消息格式：sender_wxid:\n实际消息内容
    if let Some(pos) = trimmed.find(":\n") {
        let sender = trimmed[..pos].trim().to_string();
        let actual = trimmed[pos + 2..].to_string();
        if !sender.is_empty() {
            return Some((sender, actual));
        }
    }
    // 另一种格式：sender_wxid:\r\n实际消息内容
    if let Some(pos) = trimmed.find(":\r\n") {
        let sender = trimmed[..pos].trim().to_string();
        let actual = trimmed[pos + 3..].to_string();
        if !sender.is_empty() {
            return Some((sender, actual));
        }
    }
    None
}

/// 获取广播 socket 路径
fn socket_path(port: u16) -> String {
    format!("/tmp/gewe-wait-reply-{}.sock", port)
}

/// 从监听地址解析端口
fn parse_port(listen: &str) -> Result<u16> {
    listen
        .rsplit(':')
        .next()
        .ok_or_else(|| anyhow!("无法解析端口号"))?
        .parse()
        .map_err(|_| anyhow!("无效的端口号"))
}

/// 处理 wait-reply 命令
pub async fn handle_wait_reply(
    args: WaitReplyArgs,
    _config_path: &Path,
    config: &CliConfig,
) -> Result<()> {
    // 解析参数
    let token = resolve_value(args.token.clone(), config.token.clone(), "token")?;
    let base_url = args
        .base_url
        .clone()
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(
        args.bot_alias.clone(),
        args.bot_app_id.clone().or(args.app_id.clone()),
        config,
    )?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;

    let filter_wxid = args
        .filter_wxid
        .clone()
        .unwrap_or_else(|| args.to_wxid.clone());

    // 解析消息
    let messages: Vec<WaitReplyMessage> = args
        .message
        .iter()
        .map(|s| WaitReplyMessage::parse(s))
        .collect::<Result<Vec<_>>>()?;

    // 解析正则
    let state = Arc::new(Mutex::new(WaitReplyState::new(
        filter_wxid,
        args.group_wxid.clone(),
        args.r#match.as_deref(),
    )?));

    let port = parse_port(&args.listen)?;
    let sock_path = socket_path(port);

    // 尝试绑定端口
    match try_bind_port(&args.listen) {
        Ok(listener) => {
            // 清理残留的 socket 文件
            cleanup_stale_socket(&sock_path).await?;
            run_as_primary(
                listener, &sock_path, args, state, &token, &base_url, &app_id, messages, config,
            )
            .await
        }
        Err(_) => {
            // 端口被占用，检查是否有广播 socket
            if tokio::fs::metadata(&sock_path).await.is_ok() {
                run_as_subscriber(&sock_path, args, state).await
            } else {
                error!(listen = %args.listen, "端口被其他进程占用");
                std::process::exit(ExitStatus::WebhookFailed as i32);
            }
        }
    }
}

/// 尝试绑定端口
fn try_bind_port(listen: &str) -> Result<StdTcpListener> {
    StdTcpListener::bind(listen).map_err(|e| anyhow!("绑定端口失败: {}", e))
}

/// 清理残留的 socket 文件
async fn cleanup_stale_socket(path: &str) -> Result<()> {
    if tokio::fs::metadata(path).await.is_ok() {
        // 尝试连接，如果失败说明是残留文件
        match UnixStream::connect(path).await {
            Ok(_) => {
                // socket 仍在使用
                return Err(anyhow!("socket 文件已被其他进程使用"));
            }
            Err(_) => {
                // 残留文件，删除
                tokio::fs::remove_file(path).await?;
                debug!(path, "已清理残留 socket 文件");
            }
        }
    }
    Ok(())
}

/// 作为主进程运行
#[allow(clippy::too_many_arguments)]
async fn run_as_primary(
    std_listener: StdTcpListener,
    sock_path: &str,
    args: WaitReplyArgs,
    state: Arc<Mutex<WaitReplyState>>,
    token: &str,
    base_url: &str,
    app_id: &str,
    messages: Vec<WaitReplyMessage>,
    config: &CliConfig,
) -> Result<()> {
    info!(listen = %args.listen, "作为主进程启动");

    // 转换为 tokio TcpListener
    std_listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(std_listener)?;

    // 创建广播 socket
    let _ = tokio::fs::remove_file(sock_path).await;
    let unix_listener = UnixListener::bind(sock_path)?;
    info!(path = sock_path, "广播 socket 已创建");

    // 创建广播通道
    let (broadcast_tx, _) = broadcast::channel::<BroadcastMessage>(1024);
    let broadcast_tx = Arc::new(broadcast_tx);

    // 启动订阅者连接处理
    let broadcast_tx_clone = broadcast_tx.clone();
    tokio::spawn(async move {
        loop {
            match unix_listener.accept().await {
                Ok((stream, _)) => {
                    let rx = broadcast_tx_clone.subscribe();
                    tokio::spawn(handle_subscriber_connection(stream, rx));
                }
                Err(e) => {
                    warn!(error = ?e, "接受订阅者连接失败");
                }
            }
        }
    });

    // 创建 webhook router
    let (router, mut rx, store) =
        router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
            queue_size: 1024,
        });

    // 注册机器人
    register_bot(&store, app_id, token, config).await?;

    // 启动 HTTP 服务器
    let server = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            error!(error = ?e, "HTTP 服务器错误");
        }
    });

    // 发送消息
    if !messages.is_empty() {
        let client = GeweHttpClient::new(token.to_string(), base_url.to_string())?;
        let to = if args.group_wxid.is_some() {
            args.group_wxid.as_ref().unwrap()
        } else {
            &args.to_wxid
        };

        for msg in &messages {
            if let Err(e) = send_message(&client, app_id, to, msg).await {
                error!(error = ?e, "发送消息失败");
                std::process::exit(ExitStatus::SendFailed as i32);
            }
        }
        info!(count = messages.len(), "消息发送完成");
    }

    // 等待回复
    let timeout_duration = args.timeout.map(Duration::from_secs);
    let exit_status = wait_for_reply(
        &mut rx,
        state.clone(),
        broadcast_tx.clone(),
        timeout_duration,
    )
    .await;

    // 发送 shutdown 消息
    let _ = broadcast_tx.send(BroadcastMessage::Shutdown {
        reason: "primary_exit".to_string(),
    });

    // 清理 socket 文件
    let sock_path_owned = sock_path.to_string();
    let _ = tokio::fs::remove_file(&sock_path_owned).await;

    // 停止服务器
    server.abort();

    // 输出结果
    let guard = state.lock().await;
    output_result(&args.output_format, &guard.received);

    std::process::exit(exit_status as i32);
}

/// 作为订阅者运行
async fn run_as_subscriber(
    sock_path: &str,
    args: WaitReplyArgs,
    state: Arc<Mutex<WaitReplyState>>,
) -> Result<()> {
    info!(path = sock_path, "作为订阅者启动");

    let timeout_duration = args.timeout.map(Duration::from_secs);
    let start = std::time::Instant::now();
    let sock_path_owned = sock_path.to_string();

    loop {
        // 连接到主进程
        let stream = match UnixStream::connect(&sock_path_owned).await {
            Ok(s) => s,
            Err(e) => {
                warn!(error = ?e, "连接主进程失败");
                // 短暂等待后重试或退出
                let jitter = rand::random::<u64>() % 200 + 100;
                time::sleep(Duration::from_millis(jitter)).await;

                // 检查超时
                if let Some(timeout) = timeout_duration {
                    if start.elapsed() >= timeout {
                        let guard = state.lock().await;
                        output_result(&args.output_format, &guard.received);
                        std::process::exit(ExitStatus::Timeout as i32);
                    }
                }
                continue;
            }
        };

        let (reader, _) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            // 检查超时
            if let Some(timeout) = timeout_duration {
                if start.elapsed() >= timeout {
                    let guard = state.lock().await;
                    output_result(&args.output_format, &guard.received);
                    std::process::exit(ExitStatus::Timeout as i32);
                }
            }

            // 设置读取超时
            let read_timeout = timeout_duration
                .map(|t| t.saturating_sub(start.elapsed()))
                .unwrap_or(Duration::from_secs(60));

            line.clear();
            match time::timeout(read_timeout, reader.read_line(&mut line)).await {
                Ok(Ok(0)) => {
                    // EOF，主进程断开
                    warn!("主进程断开，等待重连");
                    break;
                }
                Ok(Ok(_)) => {
                    // 解析消息
                    if let Ok(msg) = serde_json::from_str::<BroadcastMessage>(&line) {
                        match msg {
                            BroadcastMessage::Message { data } => {
                                let mut guard = state.lock().await;
                                if guard.should_accept_reply(&data) && guard.handle_message(data) {
                                    output_result(&args.output_format, &guard.received);
                                    std::process::exit(ExitStatus::Success as i32);
                                }
                            }
                            BroadcastMessage::Shutdown { .. } => {
                                warn!("收到主进程 shutdown 消息");
                                break;
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    warn!(error = ?e, "读取消息失败");
                    break;
                }
                Err(_) => {
                    // 超时
                    let guard = state.lock().await;
                    output_result(&args.output_format, &guard.received);
                    std::process::exit(ExitStatus::Timeout as i32);
                }
            }
        }

        // 短暂等待后重试
        let jitter = rand::random::<u64>() % 100 + 50;
        time::sleep(Duration::from_millis(jitter)).await;
    }
}

/// 处理订阅者连接
async fn handle_subscriber_connection(
    stream: UnixStream,
    mut rx: broadcast::Receiver<BroadcastMessage>,
) {
    let (_, mut writer) = stream.into_split();

    loop {
        match rx.recv().await {
            Ok(msg) => {
                let line = match serde_json::to_string(&msg) {
                    Ok(s) => s + "\n",
                    Err(_) => continue,
                };
                if writer.write_all(line.as_bytes()).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Closed) => break,
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
        }
    }
}

/// 等待回复
async fn wait_for_reply(
    rx: &mut mpsc::Receiver<WebhookEvent>,
    state: Arc<Mutex<WaitReplyState>>,
    broadcast_tx: Arc<broadcast::Sender<BroadcastMessage>>,
    timeout: Option<Duration>,
) -> ExitStatus {
    let wait_future = async {
        loop {
            if let Some(event) = rx.recv().await {
                let mut guard = state.lock().await;
                if let Some(reply) = guard.should_accept(&event) {
                    // 广播给订阅者
                    let _ = broadcast_tx.send(BroadcastMessage::Message {
                        data: reply.clone(),
                    });

                    if guard.handle_message(reply) {
                        return ExitStatus::Success;
                    }
                }
            } else {
                return ExitStatus::WebhookFailed;
            }
        }
    };

    match timeout {
        Some(duration) => match time::timeout(duration, wait_future).await {
            Ok(status) => status,
            Err(_) => ExitStatus::Timeout,
        },
        None => wait_future.await,
    }
}

/// 注册机器人到 SessionStore
async fn register_bot<S: SessionStore>(
    store: &Arc<S>,
    app_id: &str,
    token: &str,
    config: &CliConfig,
) -> Result<()> {
    // 查找机器人配置
    let bot_cfg = config.bots.iter().find(|b| b.app_id == app_id);

    store
        .put_session(BotContext {
            app_id: AppId(app_id.to_string()),
            token: token.to_string(),
            webhook_secret: bot_cfg.and_then(|b| b.webhook_secret.clone()),
            description: bot_cfg.and_then(|b| b.alias.clone()),
        })
        .await;

    info!(app_id, "机器人已注册");
    Ok(())
}

/// 发送消息
async fn send_message(
    client: &GeweHttpClient,
    app_id: &str,
    to: &str,
    msg: &WaitReplyMessage,
) -> Result<()> {
    match msg {
        WaitReplyMessage::Text(content) => {
            client.send_text(app_id, to, content, None).await?;
        }
        WaitReplyMessage::Image(url) => {
            client.send_image(app_id, to, url).await?;
        }
        WaitReplyMessage::Voice(url) => {
            // 默认语音时长 0
            client.send_voice(app_id, to, url, 0).await?;
        }
        WaitReplyMessage::Video(url) => {
            // 默认无缩略图，时长 0
            client.send_video(app_id, to, url, "", 0).await?;
        }
        WaitReplyMessage::Link {
            title,
            desc,
            url,
            thumb_url,
        } => {
            client
                .send_link(app_id, to, title, desc, url, thumb_url)
                .await?;
        }
    }
    Ok(())
}

/// 输出结果
fn output_result(format: &OutputFormat, received: &[ReceivedReply]) {
    match format {
        OutputFormat::Text => {
            for reply in received {
                println!("{}", reply.content);
            }
        }
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(received) {
                println!("{}", json);
            }
        }
    }
}

fn resolve_bot(
    alias: Option<String>,
    explicit: Option<String>,
    config: &CliConfig,
) -> Result<Option<String>> {
    if let Some(alias) = alias {
        Ok(Some(lookup_bot(config, &alias).ok_or_else(|| {
            anyhow!("bot alias not found: {}", alias)
        })?))
    } else {
        Ok(explicit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!("text".parse::<OutputFormat>().unwrap(), OutputFormat::Text);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("TEXT".parse::<OutputFormat>().unwrap(), OutputFormat::Text);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_wait_reply_message_parse_text() {
        let msg = WaitReplyMessage::parse("text:Hello World").unwrap();
        match msg {
            WaitReplyMessage::Text(content) => assert_eq!(content, "Hello World"),
            _ => panic!("Expected Text message"),
        }
    }

    #[test]
    fn test_wait_reply_message_parse_image() {
        let msg = WaitReplyMessage::parse("image:http://example.com/img.png").unwrap();
        match msg {
            WaitReplyMessage::Image(url) => assert_eq!(url, "http://example.com/img.png"),
            _ => panic!("Expected Image message"),
        }
    }

    #[test]
    fn test_wait_reply_message_parse_link() {
        let msg =
            WaitReplyMessage::parse("link:Title|Description|http://example.com|http://thumb.png")
                .unwrap();
        match msg {
            WaitReplyMessage::Link {
                title,
                desc,
                url,
                thumb_url,
            } => {
                assert_eq!(title, "Title");
                assert_eq!(desc, "Description");
                assert_eq!(url, "http://example.com");
                assert_eq!(thumb_url, "http://thumb.png");
            }
            _ => panic!("Expected Link message"),
        }
    }

    #[test]
    fn test_wait_reply_message_parse_invalid() {
        assert!(WaitReplyMessage::parse("invalid").is_err());
        assert!(WaitReplyMessage::parse("unknown:content").is_err());
        assert!(WaitReplyMessage::parse("link:only|three|parts").is_err());
    }

    #[test]
    fn test_extract_group_sender_and_content() {
        let content = "wxid_abc123:\nHello World";
        let (sender, actual) = extract_group_sender_and_content(content).unwrap();
        assert_eq!(sender, "wxid_abc123");
        assert_eq!(actual, "Hello World");

        let content = "wxid_def456:\r\nAnother message";
        let (sender, actual) = extract_group_sender_and_content(content).unwrap();
        assert_eq!(sender, "wxid_def456");
        assert_eq!(actual, "Another message");

        // 无冒号
        assert!(extract_group_sender_and_content("no colon").is_none());
    }

    #[test]
    fn test_socket_path() {
        assert_eq!(socket_path(4399), "/tmp/gewe-wait-reply-4399.sock");
        assert_eq!(socket_path(8080), "/tmp/gewe-wait-reply-8080.sock");
    }

    #[test]
    fn test_parse_port() {
        assert_eq!(parse_port("0.0.0.0:4399").unwrap(), 4399);
        assert_eq!(parse_port("127.0.0.1:8080").unwrap(), 8080);
        assert_eq!(parse_port(":3000").unwrap(), 3000);
        assert!(parse_port("invalid").is_err());
    }

    #[test]
    fn test_wait_reply_state_new() {
        let state = WaitReplyState::new(
            "wxid_test".to_string(),
            Some("group@chatroom".to_string()),
            Some(r"确认|取消"),
        )
        .unwrap();

        assert_eq!(state.filter_wxid, "wxid_test");
        assert_eq!(state.group_wxid, Some("group@chatroom".to_string()));
        assert!(state.match_regex.is_some());
        assert!(!state.matched);
    }

    #[test]
    fn test_wait_reply_state_handle_message() {
        let mut state =
            WaitReplyState::new("wxid_test".to_string(), None, Some(r"确认|取消")).unwrap();

        // 不匹配的消息
        let reply1 = ReceivedReply {
            from_wxid: "wxid_test".to_string(),
            group_wxid: None,
            content: "hello".to_string(),
            timestamp: Utc::now(),
        };
        assert!(!state.handle_message(reply1));
        assert!(!state.matched);
        assert_eq!(state.received.len(), 1);

        // 匹配的消息
        let reply2 = ReceivedReply {
            from_wxid: "wxid_test".to_string(),
            group_wxid: None,
            content: "确认".to_string(),
            timestamp: Utc::now(),
        };
        assert!(state.handle_message(reply2));
        assert!(state.matched);
        assert_eq!(state.received.len(), 2);
    }

    #[test]
    fn test_wait_reply_state_should_accept_reply() {
        let state = WaitReplyState::new(
            "wxid_target".to_string(),
            Some("group@chatroom".to_string()),
            None,
        )
        .unwrap();

        // 匹配的回复
        let reply1 = ReceivedReply {
            from_wxid: "wxid_target".to_string(),
            group_wxid: Some("group@chatroom".to_string()),
            content: "test".to_string(),
            timestamp: Utc::now(),
        };
        assert!(state.should_accept_reply(&reply1));

        // 错误的发送者
        let reply2 = ReceivedReply {
            from_wxid: "wxid_other".to_string(),
            group_wxid: Some("group@chatroom".to_string()),
            content: "test".to_string(),
            timestamp: Utc::now(),
        };
        assert!(!state.should_accept_reply(&reply2));

        // 错误的群
        let reply3 = ReceivedReply {
            from_wxid: "wxid_target".to_string(),
            group_wxid: Some("other_group@chatroom".to_string()),
            content: "test".to_string(),
            timestamp: Utc::now(),
        };
        assert!(!state.should_accept_reply(&reply3));
    }

    #[test]
    fn test_broadcast_message_serialize() {
        let msg = BroadcastMessage::Message {
            data: ReceivedReply {
                from_wxid: "wxid_test".to_string(),
                group_wxid: None,
                content: "hello".to_string(),
                timestamp: Utc::now(),
            },
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"message\""));

        let shutdown = BroadcastMessage::Shutdown {
            reason: "test".to_string(),
        };
        let json = serde_json::to_string(&shutdown).unwrap();
        assert!(json.contains("\"type\":\"shutdown\""));
    }
}
