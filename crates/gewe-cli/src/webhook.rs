//! Webhook 服务器命令模块
//!
//! 提供 `serve-webhook` 命令，启动 HTTP 服务器接收 Gewe 平台推送的消息事件。

use crate::config::CliConfig;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Args;
use gewe_core::{AppId, BotContext};
use gewe_session::{InMemorySessionStore, SessionStore};
use gewe_webhook::{router_with_channel_and_state, WebhookBuilderOptions, WebhookEvent};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

/// serve-webhook 命令参数
#[derive(Args)]
pub struct ServeWebhookArgs {
    /// 监听地址（如 0.0.0.0:3000）
    #[arg(long, short = 'l', default_value = "0.0.0.0:3000")]
    pub listen: String,

    /// 事件队列大小
    #[arg(long, default_value = "1024")]
    pub queue_size: usize,

    /// 打印事件到控制台
    #[arg(long, default_value = "true")]
    pub print: bool,

    /// 保存事件到文件（JSONL 格式）
    #[arg(long, short = 'o')]
    pub output_file: Option<PathBuf>,

    /// 转发事件到指定 URL（可多次指定）
    #[arg(long, short = 'f')]
    pub forward_url: Vec<String>,

    /// 转发请求超时时间（秒）
    #[arg(long, default_value = "30")]
    pub forward_timeout: u64,

    /// 要求签名验证
    #[arg(long)]
    pub require_signature: bool,
}

/// 输出处理器 trait
#[async_trait]
pub trait OutputHandler: Send + Sync {
    async fn handle(&self, event: &WebhookEvent) -> Result<()>;
}

/// 控制台输出处理器
pub struct ConsoleOutput;

impl ConsoleOutput {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OutputHandler for ConsoleOutput {
    async fn handle(&self, event: &WebhookEvent) -> Result<()> {
        let json = serde_json::to_string_pretty(&serde_json::json!({
            "Appid": event.app_id.0,
            "TypeName": event.type_name,
            "Data": event.data,
        }))?;
        println!("{}", json);
        Ok(())
    }
}

/// 文件输出处理器（JSONL 格式）
pub struct FileOutput {
    path: PathBuf,
    file: Arc<Mutex<Option<tokio::fs::File>>>,
}

impl FileOutput {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            path,
            file: Arc::new(Mutex::new(None)),
        })
    }

    async fn ensure_file(&self) -> Result<tokio::fs::File> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await?;
        Ok(file)
    }
}

#[async_trait]
impl OutputHandler for FileOutput {
    async fn handle(&self, event: &WebhookEvent) -> Result<()> {
        let mut guard = self.file.lock().await;
        if guard.is_none() {
            *guard = Some(self.ensure_file().await?);
        }

        let line = serde_json::to_string(&serde_json::json!({
            "Timestamp": chrono::Utc::now().to_rfc3339(),
            "Appid": event.app_id.0,
            "TypeName": event.type_name,
            "Data": event.data,
        }))?;

        if let Some(ref mut file) = *guard {
            file.write_all(line.as_bytes()).await?;
            file.write_all(b"\n").await?;
            file.flush().await?;
        }

        Ok(())
    }
}

/// HTTP 转发输出处理器
pub struct ForwardOutput {
    url: String,
    client: reqwest::Client,
}

impl ForwardOutput {
    pub fn new(url: String, timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to build HTTP client");

        Self { url, client }
    }
}

#[async_trait]
impl OutputHandler for ForwardOutput {
    async fn handle(&self, event: &WebhookEvent) -> Result<()> {
        let payload = serde_json::json!({
            "Appid": event.app_id.0,
            "TypeName": event.type_name,
            "Data": event.data,
        });

        let response = self.client.post(&self.url).json(&payload).send().await?;

        if !response.status().is_success() {
            tracing::warn!(
                url = %self.url,
                status = %response.status(),
                "转发请求失败"
            );
        } else {
            tracing::debug!(url = %self.url, "事件已转发");
        }

        Ok(())
    }
}

/// 事件处理器：将事件分发到所有输出处理器
pub struct EventProcessor {
    outputs: Vec<Box<dyn OutputHandler>>,
}

impl EventProcessor {
    pub fn new(outputs: Vec<Box<dyn OutputHandler>>) -> Self {
        Self { outputs }
    }

    pub async fn process(&self, event: WebhookEvent) -> Result<()> {
        let futures: Vec<_> = self
            .outputs
            .iter()
            .map(|output| output.handle(&event))
            .collect();

        let results = futures::future::join_all(futures).await;

        for result in results {
            if let Err(err) = result {
                tracing::error!(?err, "输出处理失败");
            }
        }

        Ok(())
    }
}

/// 处理 serve-webhook 命令
pub async fn handle_serve_webhook(
    args: ServeWebhookArgs,
    _config_path: &Path,
    config: &CliConfig,
) -> Result<()> {
    // 1. 构建输出处理器
    let mut outputs: Vec<Box<dyn OutputHandler>> = Vec::new();

    if args.print {
        outputs.push(Box::new(ConsoleOutput::new()));
    }

    if let Some(ref path) = args.output_file {
        outputs.push(Box::new(FileOutput::new(path.clone())?));
    }

    for url in &args.forward_url {
        outputs.push(Box::new(ForwardOutput::new(
            url.clone(),
            args.forward_timeout,
        )));
    }

    if outputs.is_empty() {
        outputs.push(Box::new(ConsoleOutput::new()));
    }

    let processor = Arc::new(EventProcessor::new(outputs));

    // 2. 创建 webhook router
    let (router, rx, store) =
        router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
            queue_size: args.queue_size,
        });

    // 3. 注册机器人
    let global_token = config.token.clone();
    let mut registered_count = 0;

    for bot in &config.bots {
        let token = bot
            .token
            .clone()
            .or_else(|| global_token.clone())
            .ok_or_else(|| anyhow!("Bot {} 缺少 token 配置", bot.app_id))?;

        store
            .put_session(BotContext {
                app_id: AppId(bot.app_id.clone()),
                token,
                webhook_secret: bot.webhook_secret.clone(),
                description: bot.alias.clone(),
            })
            .await;
        tracing::info!(app_id = %bot.app_id, alias = ?bot.alias, "已注册机器人");
        registered_count += 1;
    }

    if registered_count == 0 {
        tracing::warn!("未配置任何机器人，请使用 `gewe config` 或编辑配置文件添加");
    }

    // 4. 设置签名验证环境变量
    if args.require_signature {
        std::env::set_var("GEWE_WEBHOOK_REQUIRE_SIGNATURE", "1");
    }

    // 5. 启动事件处理任务
    let mut event_rx = rx;
    let processor_clone = processor.clone();
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            if let Err(err) = processor_clone.process(event).await {
                tracing::error!(?err, "事件处理失败");
            }
        }
    });

    // 6. 启动 HTTP 服务器
    let listener = TcpListener::bind(&args.listen).await?;
    tracing::info!(
        listen = %args.listen,
        bots = registered_count,
        "Webhook 服务器已启动"
    );

    axum::serve(listener, router).await?;

    Ok(())
}
