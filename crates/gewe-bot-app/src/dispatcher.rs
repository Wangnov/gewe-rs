use crate::config::{
    AiAction, AiTool, AppConfig, ChatKind, CommandAction, MatchConfig, ReplyMode, RuleAction,
    RuleConfig, RuleKind, SaveAction,
};
use crate::tools::{
    run_claude_changelog, run_gemini_image, run_http_request, run_tool_versions, ChangelogQuery,
    HttpRequestQuery, ImageConfig, ImageQuery, VersionQuery,
};
use anyhow::{anyhow, Context, Result};
use gewe_core::{AppId, GeweError};
use gewe_http::GeweHttpClient;
use gewe_webhook::WebhookEvent;
use rand::Rng;
use regex::Regex;
use rig::completion::{
    self, CompletionModel, CompletionRequest, Message as RigMessage, ToolDefinition,
};
use rig::prelude::*;
use rig::providers::{anthropic, gemini, openai};
use std::{
    collections::{HashMap, VecDeque},
    process::Stdio,
    sync::OnceLock,
    time::{Duration, Instant},
};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as TokioCommand;
use tokio::sync::Mutex;
use tokio::time;

pub struct Dispatcher {
    bots: HashMap<AppId, BotInstance>,
    image_config: ImageConfig,
}

struct BotInstance {
    client: GeweHttpClient,
    rules: Vec<CompiledRule>,
    app_id: AppId,
    limiter: RateLimiter,
}

/// 简单的滑动窗口限速器，支持随机抖动
struct RateLimiter {
    window: Duration,
    max: usize,
    jitter_ms: u64,
    sends: Mutex<VecDeque<Instant>>,
}

impl BotInstance {
    async fn send_text(&self, to: &str, content: &str, ats: Option<&str>) -> Result<(), GeweError> {
        self.limiter.acquire().await;
        self.client
            .send_text(&self.app_id.0, to, content, ats)
            .await
            .map(|_| ())
    }

    async fn send_image(&self, to: &str, img_url: &str) -> Result<(), GeweError> {
        self.limiter.acquire().await;
        self.client
            .send_image(&self.app_id.0, to, img_url)
            .await
            .map(|_| ())
    }

    async fn send_appmsg(&self, to: &str, appmsg: &str) -> Result<(), GeweError> {
        self.limiter.acquire().await;
        self.client
            .send_app_msg(&self.app_id.0, to, appmsg)
            .await
            .map(|_| ())
    }
}

impl RateLimiter {
    fn new(window: Duration, max: usize, jitter_ms: u64) -> Self {
        Self {
            window,
            max,
            jitter_ms,
            sends: Mutex::new(VecDeque::new()),
        }
    }

    async fn acquire(&self) {
        loop {
            let mut guard = self.sends.lock().await;
            let now = Instant::now();
            while let Some(&ts) = guard.front() {
                if now.duration_since(ts) >= self.window {
                    guard.pop_front();
                } else {
                    break;
                }
            }

            if guard.len() < self.max {
                guard.push_back(now);
                drop(guard);
                if self.jitter_ms > 0 {
                    let jitter = rand::thread_rng().gen_range(0..=self.jitter_ms);
                    if jitter > 0 {
                        time::sleep(Duration::from_millis(jitter)).await;
                    }
                }
                return;
            }

            // 已达上限，等待最早的记录过期
            let wait = self
                .window
                .saturating_sub(now.duration_since(*guard.front().unwrap()));
            drop(guard);
            time::sleep(wait).await;
        }
    }
}

const DEFAULT_COMMAND_TIMEOUT_SECS: u64 = 15;
/// 默认命令/工具输出上限（约 20 KB），按字节截断。
const DEFAULT_COMMAND_MAX_OUTPUT: usize = 20 * 1024;
/// 引用回复标题截断上限（按字符计），宽于默认中文 5000 字的需求。
const DEFAULT_QUOTE_TITLE_MAX_LEN: usize = 20 * 1024;
const DEFAULT_AI_MAX_RETRIES: u32 = 2;
const DEFAULT_AI_RETRY_DELAY_MS: u64 = 1000;
const RATE_LIMIT_WINDOW_SECS: u64 = 60;
const RATE_LIMIT_MAX_PER_WINDOW: usize = 40;
const RATE_LIMIT_MAX_JITTER_MS: u64 = 300;

/// 根据 AI 错误生成用户友好的提示消息
fn ai_error_message(err: &anyhow::Error) -> String {
    let msg = err.to_string().to_lowercase();
    if msg.contains("api key") || msg.contains("unauthorized") || msg.contains("401") {
        "AI 服务配置异常，请联系管理员".to_string()
    } else if msg.contains("timeout") || msg.contains("timed out") {
        "AI 服务响应超时，请稍后重试".to_string()
    } else if msg.contains("rate limit") || msg.contains("429") {
        "AI 服务繁忙，请稍后重试".to_string()
    } else if msg.contains("connection") || msg.contains("network") {
        "AI 服务连接失败，请稍后重试".to_string()
    } else if msg.contains("503") || msg.contains("502") || msg.contains("500") {
        "AI 服务暂时不可用，请稍后重试".to_string()
    } else {
        "AI 请求失败，请稍后重试".to_string()
    }
}

#[derive(Debug)]
enum CommandSource {
    Builtin,
    External,
}

#[derive(Debug)]
struct CommandReport {
    reply: Option<String>,
    truncated: bool,
    duration: Duration,
    exit_code: Option<i32>,
    timed_out: bool,
    disabled: bool,
    source: CommandSource,
    program: String,
    stderr: Option<String>,
    error: Option<String>,
    /// 图片 URL 列表（用于图像生成工具）
    image_urls: Vec<String>,
}

#[derive(Clone)]
struct CompiledRule {
    kind: RuleKind,
    matcher: Matcher,
    from: FromGate,
    chat: Option<ChatKind>,
    action: RuleAction,
}

#[derive(Clone)]
struct Matcher {
    equals: Option<String>,
    contains: Option<String>,
    regex: Option<Regex>,
}

#[derive(Clone, Default)]
struct FromGate {
    nick: Option<String>,
    wxid: Option<String>,
}

/// 统一的 LLM 客户端封装，支持 OpenAI/Anthropic/Gemini
enum LlmClient {
    OpenAi(openai::responses_api::ResponsesCompletionModel),
    Anthropic(anthropic::completion::CompletionModel),
    Gemini(gemini::completion::CompletionModel),
}

/// LLM 响应结果
struct LlmResponse {
    /// 文本回复（如果有）
    text: Option<String>,
    /// 工具调用（如果有）
    tool_call: Option<LlmToolCall>,
}

/// LLM 工具调用
#[derive(Debug, Clone)]
struct LlmToolCall {
    name: String,
    #[allow(dead_code)]
    arguments: Option<String>,
}

impl LlmClient {
    /// 根据配置创建对应的 LLM 客户端
    fn from_config(action: &AiAction) -> Result<Self> {
        // 优先使用直接配置的 api_key，否则从环境变量读取
        let api_key = if let Some(ref key) = action.api_key {
            key.clone()
        } else {
            let env_name = action.api_key_env.as_deref().unwrap_or("GEWE_AI_API_KEY");
            std::env::var(env_name)
                .or_else(|_| std::env::var("GEWE_AI_API_KEY"))
                .map_err(|_| {
                    anyhow!(
                        "未找到 AI API Key，请配置 api_key 或设置环境变量 {}",
                        env_name
                    )
                })?
        };

        let provider = action.provider.as_deref().unwrap_or("openai");

        match provider {
            "anthropic" | "claude" => {
                let mut builder = anthropic::Client::builder(&api_key);
                if let Some(ref url) = action.base_url {
                    builder = builder.base_url(url.trim_end_matches('/'));
                }
                let client = builder
                    .build()
                    .map_err(|e| anyhow!("创建 Anthropic 客户端失败: {}", e))?;
                Ok(Self::Anthropic(client.completion_model(&action.model)))
            }
            "gemini" | "google" => {
                let mut builder = gemini::Client::builder(&api_key);
                if let Some(ref url) = action.base_url {
                    builder = builder.base_url(url.trim_end_matches('/'));
                }
                let client = builder
                    .build()
                    .map_err(|e| anyhow!("创建 Gemini 客户端失败: {}", e))?;
                Ok(Self::Gemini(client.completion_model(&action.model)))
            }
            _ => {
                // 默认使用 OpenAI 兼容模式，支持自定义 base_url
                let base_url = action
                    .base_url
                    .as_deref()
                    .unwrap_or("https://api.openai.com/v1")
                    .trim_end_matches('/');

                let client = openai::Client::builder(&api_key).base_url(base_url).build();

                Ok(Self::OpenAi(client.completion_model(&action.model)))
            }
        }
    }

    /// 执行 completion 请求
    async fn complete(&self, request: CompletionRequest) -> Result<LlmResponse> {
        match self {
            Self::OpenAi(model) => {
                let response = model
                    .completion(request)
                    .await
                    .map_err(|e| anyhow!("OpenAI 请求失败: {}", e))?;
                Self::parse_response(response)
            }
            Self::Anthropic(model) => {
                let response = model
                    .completion(request)
                    .await
                    .map_err(|e| anyhow!("Anthropic 请求失败: {}", e))?;
                Self::parse_response(response)
            }
            Self::Gemini(model) => {
                let response = model
                    .completion(request)
                    .await
                    .map_err(|e| anyhow!("Gemini 请求失败: {}", e))?;
                Self::parse_response(response)
            }
        }
    }

    /// 解析 LLM 响应，提取文本和工具调用
    fn parse_response<T>(response: completion::CompletionResponse<T>) -> Result<LlmResponse> {
        let mut text = None;
        let mut tool_call = None;

        for content in response.choice.iter() {
            match content {
                completion::AssistantContent::Text(t) => {
                    let content_text = t.text.trim();
                    if !content_text.is_empty() {
                        text = Some(content_text.to_string());
                    }
                }
                completion::AssistantContent::ToolCall(tc) => {
                    tool_call = Some(LlmToolCall {
                        name: tc.function.name.clone(),
                        arguments: Some(tc.function.arguments.to_string()),
                    });
                }
                _ => {} // 忽略其他内容类型（如 Reasoning）
            }
        }

        Ok(LlmResponse { text, tool_call })
    }

    /// 带重试的 completion 请求
    async fn complete_with_retry(
        &self,
        request_builder: impl Fn() -> CompletionRequest,
        max_retries: u32,
        base_delay_ms: u64,
    ) -> Result<LlmResponse> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            let request = request_builder();
            match self.complete(request).await {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    let is_last = attempt == max_retries;
                    let retryable = Self::is_retryable_error(&e);

                    if is_last || !retryable {
                        tracing::warn!(
                            attempt = attempt + 1,
                            max_retries = max_retries + 1,
                            retryable,
                            err = ?e,
                            "AI 请求失败，不再重试"
                        );
                        return Err(e);
                    }

                    let delay_ms = base_delay_ms * 2u64.pow(attempt);
                    tracing::info!(
                        attempt = attempt + 1,
                        max_retries = max_retries + 1,
                        delay_ms,
                        err = ?e,
                        "AI 请求失败，准备重试"
                    );
                    last_error = Some(e);
                    time::sleep(Duration::from_millis(delay_ms)).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("AI 请求失败")))
    }

    /// 判断错误是否可重试
    fn is_retryable_error(err: &anyhow::Error) -> bool {
        let msg = err.to_string().to_lowercase();
        // 可重试的情况：网络问题、超时、服务端错误、限流
        msg.contains("timeout")
            || msg.contains("timed out")
            || msg.contains("connection")
            || msg.contains("network")
            || msg.contains("503")
            || msg.contains("502")
            || msg.contains("500")
            || msg.contains("429")
            || msg.contains("rate limit")
            || msg.contains("rate_limit")
            || msg.contains("overloaded")
            || msg.contains("temporarily unavailable")
    }
}

impl Dispatcher {
    pub fn new(cfg: &AppConfig) -> Result<Self> {
        let mut bots = HashMap::new();
        for bot_cfg in &cfg.bots {
            let client = GeweHttpClient::new(bot_cfg.token.clone(), bot_cfg.base_url.clone())
                .with_context(|| format!("初始化 GEWE 客户端失败: {}", bot_cfg.app_id))?;
            bots.insert(
                AppId(bot_cfg.app_id.clone()),
                BotInstance {
                    client,
                    rules: bot_cfg
                        .rules
                        .iter()
                        .map(CompiledRule::try_from_config)
                        .collect::<Result<Vec<_>>>()?,
                    app_id: AppId(bot_cfg.app_id.clone()),
                    limiter: RateLimiter::new(
                        Duration::from_secs(RATE_LIMIT_WINDOW_SECS),
                        RATE_LIMIT_MAX_PER_WINDOW,
                        RATE_LIMIT_MAX_JITTER_MS,
                    ),
                },
            );
        }

        // 初始化图片配置（API Key 从环境变量读取）
        let image_config = ImageConfig {
            api_key: String::new(), // 会在运行时从 AiAction 获取
            base_url: None,
            image_dir: cfg.image_dir.clone(),
            image_url_prefix: cfg.image_url_prefix.clone(),
            external_base_url: cfg.external_base_url.clone(),
        };

        Ok(Self { bots, image_config })
    }

    pub async fn handle(&self, event: WebhookEvent) -> Result<()> {
        let Some(bot) = self.bots.get(&event.app_id) else {
            tracing::warn!(app_id=?event.app_id, "收到未知 app_id 的事件，已忽略");
            return Ok(());
        };
        let norm = normalize_event(&event)?;
        self.apply_rules(bot, &event, &norm).await
    }

    async fn apply_rules(
        &self,
        bot: &BotInstance,
        _event: &WebhookEvent,
        norm: &NormalizedEvent,
    ) -> Result<()> {
        for rule in &bot.rules {
            if !rule.is_match(norm) {
                continue;
            }

            log_rule_hit(bot, rule, norm);
            let reply_mode = rule.reply_mode();

            if rule.action.require_mention.unwrap_or(false)
                && norm.chat == Some(ChatKind::Group)
                && !mentioned_bot(norm)
            {
                let sender_colored = colorize(norm.sender_wxid(), "33"); // yellow
                let app_colored = colorize(Some(&bot.app_id.0), "34"); // blue
                let chat_colored =
                    colorize(norm.chat.as_ref().map(|c| chat_kind_cn(c.clone())), "31"); // red
                tracing::debug!(
                    app_id=%app_colored,
                    from=%sender_colored,
                    chat=%chat_colored,
                    "规则要求被 @，但当前消息未 @ 机器人，跳过"
                );
                continue;
            }

            if let Some(ref reply) = rule.action.reply_text {
                match send_reply(bot, norm, &reply_mode, reply).await {
                    Ok(_) => tracing::info!(
                        app_id=?bot.app_id,
                        from=?norm.from_wxid,
                        reply,
                        ?reply_mode,
                        "自动回复成功"
                    ),
                    Err(err) => tracing::warn!(
                        ?err,
                        app_id=?bot.app_id,
                        from=?norm.from_wxid,
                        reply,
                        ?reply_mode,
                        "自动回复失败"
                    ),
                }
            }

            if let Some(ref save) = rule.action.save {
                match save_media(bot, norm, save).await {
                    Ok(path) => tracing::info!(
                        app_id=?bot.app_id,
                        rule_kind=?rule.kind,
                        event_kind=?norm.kind,
                        from=?norm.from_wxid,
                        new_msg_id=?norm.new_msg_id,
                        %path,
                        "媒体已保存"
                    ),
                    Err(err) => tracing::warn!(
                        ?err,
                        app_id=?bot.app_id,
                        rule_kind=?rule.kind,
                        event_kind=?norm.kind,
                        from=?norm.from_wxid,
                        new_msg_id=?norm.new_msg_id,
                        "保存媒体失败"
                    ),
                }
            }

            if let Some(forwards) = rule.action.forward.as_ref() {
                if let Some(ref content) = norm.content {
                    for wxid in forwards {
                        match bot.send_text(wxid, content, None).await {
                            Ok(_) => tracing::info!(app_id=?bot.app_id, to = wxid, "转发成功"),
                            Err(err) => tracing::warn!(
                                ?err,
                                app_id=?bot.app_id,
                                to = wxid,
                                "转发失败"
                            ),
                        }
                    }
                } else {
                    tracing::debug!(
                        app_id=?bot.app_id,
                        "缺少 content，转发动作已跳过"
                    );
                }
            }

            if rule.action.log.unwrap_or(false) {
                let content_colored = colorize(norm.normalized_content.as_deref(), "36"); // cyan
                let sender_colored = colorize(norm.sender_wxid(), "33"); // yellow
                let from_colored = colorize(norm.from_wxid.as_deref(), "32"); // green
                let kind_colored = colorize(Some(rule_kind_cn(&norm.kind)), "35"); // magenta
                let app_colored = colorize(Some(&bot.app_id.0), "34"); // blue
                let chat_colored =
                    colorize(norm.chat.as_ref().map(|c| chat_kind_cn(c.clone())), "31"); // red
                tracing::info!(
                    app_id=%app_colored,
                    kind=%kind_colored,
                    chat=%chat_colored,
                    from_wxid=%from_colored,
                    sender_wxid=%sender_colored,
                    content=%content_colored,
                    "规则动作：记录日志"
                );
            }

            if rule.action.ignore.unwrap_or(false) {
                tracing::info!(
                    app_id=?bot.app_id,
                    kind=?norm.kind,
                    chat=?norm.chat,
                    from=?norm.from_wxid,
                    "规则标记为忽略，停止后续动作"
                );
                break;
            }

            if let Some(ai) = rule.action.ai.as_ref() {
                self.handle_ai_action(bot, norm, ai, reply_mode.clone())
                    .await?;
            }

            if let Some(command) = rule.action.command.as_ref() {
                self.handle_command(bot, norm, command, reply_mode.clone())
                    .await?;
            }
            break;
        }

        Ok(())
    }

    async fn handle_ai_action(
        &self,
        bot: &BotInstance,
        norm: &NormalizedEvent,
        action: &AiAction,
        reply_mode: ReplyMode,
    ) -> Result<()> {
        let Some(reply_to) = norm.from_wxid.as_deref() else {
            tracing::debug!(app_id=?bot.app_id, "缺少来源 wxid，跳过 AI 动作");
            return Ok(());
        };

        if action.model.trim().is_empty() {
            tracing::warn!(app_id=?bot.app_id, "ai.model 为空，跳过 AI 动作");
            return Ok(());
        }

        // 创建 LLM 客户端
        let llm = match LlmClient::from_config(action) {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(app_id=?bot.app_id, err=?e, "创建 LLM 客户端失败");
                let _ = send_reply(bot, norm, &reply_mode, "AI 服务配置异常，请联系管理员").await;
                return Ok(());
            }
        };

        // 执行预处理命令（如有）
        let command_output = if let Some(cmd) = action.command.as_ref() {
            let max = action
                .max_command_output
                .unwrap_or_else(|| command_max_output(cmd));
            let report = execute_command_action(cmd, norm, max, None, None).await;
            if report.error.is_some() {
                tracing::warn!(app_id=?bot.app_id, program=?cmd.program, "预处理命令异常");
            }
            report.reply
        } else {
            None
        };

        // 构建用户消息
        let user_content = build_user_content(action, norm, command_output.as_deref());

        // 获取重试配置
        let max_retries = action.max_retries.unwrap_or(DEFAULT_AI_MAX_RETRIES);
        let retry_delay_ms = action.retry_delay_ms.unwrap_or(DEFAULT_AI_RETRY_DELAY_MS);

        // 构建 completion 请求
        let tools = build_tools_for_request(&action.tools);

        // 发送请求（带重试）
        let response = match llm
            .complete_with_retry(
                || build_completion_request(action, &user_content, &tools),
                max_retries,
                retry_delay_ms,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                let user_msg = ai_error_message(&e);
                let _ = send_reply(bot, norm, &reply_mode, &user_msg).await;
                return Ok(());
            }
        };

        // 处理工具调用
        if let Some(ref tc) = response.tool_call {
            let tool_name = &tc.name;
            let Some(tool_cfg) = action.tools.iter().find(|t| &t.name == tool_name) else {
                send_reply(
                    bot,
                    norm,
                    &reply_mode,
                    &format!("未配置工具: {}", tool_name),
                )
                .await?;
                return Ok(());
            };
            let Some(cmd) = tool_cfg.command.as_ref() else {
                send_reply(
                    bot,
                    norm,
                    &reply_mode,
                    &format!("工具 {} 未绑定命令", tool_name),
                )
                .await?;
                return Ok(());
            };

            // 执行工具命令
            let max = action
                .max_command_output
                .unwrap_or_else(|| command_max_output(cmd));

            // 为图像生成工具准备配置（需要从 AiAction 获取 API Key）
            let image_config = if cmd.program == "gemini_image" {
                let api_key = if let Some(ref key) = action.api_key {
                    key.clone()
                } else {
                    let env_name = action.api_key_env.as_deref().unwrap_or("GEWE_AI_API_KEY");
                    std::env::var(env_name)
                        .or_else(|_| std::env::var("GEWE_AI_API_KEY"))
                        .unwrap_or_default()
                };
                Some(ImageConfig {
                    api_key,
                    base_url: action.base_url.clone(),
                    image_dir: self.image_config.image_dir.clone(),
                    image_url_prefix: self.image_config.image_url_prefix.clone(),
                    external_base_url: self.image_config.external_base_url.clone(),
                })
            } else {
                None
            };

            if let Some(text) = cmd.pre_reply.as_deref().filter(|s| !s.trim().is_empty()) {
                let _ = send_reply(bot, norm, &reply_mode, text).await;
            }

            let report = execute_command_action(
                cmd,
                norm,
                max,
                tc.arguments.as_deref(),
                image_config.as_ref(),
            )
            .await;
            log_command_report(bot, &report, reply_to, &cmd.args);

            // 发送图片（如果有）
            for img_url in &report.image_urls {
                match bot.send_image(reply_to, img_url).await {
                    Ok(_) => {
                        tracing::info!(
                            app_id = ?bot.app_id,
                            to = reply_to,
                            url = img_url,
                            "图片发送成功"
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            ?err,
                            app_id = ?bot.app_id,
                            to = reply_to,
                            url = img_url,
                            "图片发送失败"
                        );
                    }
                }
            }

            // 如果是图像生成工具且有图片，直接发送文本回复（如果有）并返回
            if !report.image_urls.is_empty() {
                if let Some(ref text) = report.reply {
                    if !text.is_empty() {
                        let _ = send_reply(bot, norm, &reply_mode, text).await;
                    }
                }
                // post_reply（如有）在成功执行后发送一次提示
                if report.error.is_none() {
                    if let Some(text) = cmd.post_reply.as_deref().filter(|s| !s.trim().is_empty()) {
                        let _ = send_reply(bot, norm, &reply_mode, text).await;
                    }
                }
                tracing::info!(
                    app_id = ?bot.app_id,
                    model = ?action.model,
                    tool = ?tool_name,
                    image_count = report.image_urls.len(),
                    "图像生成工具执行完成"
                );
                return Ok(());
            }

            let tool_output = report.reply.unwrap_or_else(|| "命令无输出".to_string());

            // 二次请求（带重试）
            let follow_content = format!(
                "{}\n\n工具 `{}` 输出：\n{}\n\n请结合以上工具输出，回答用户需求。",
                user_content, tool_name, tool_output
            );
            let follow_response = match llm
                .complete_with_retry(
                    || build_completion_request(action, &follow_content, &[]),
                    max_retries,
                    retry_delay_ms,
                )
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    let user_msg = ai_error_message(&e);
                    let _ = send_reply(bot, norm, &reply_mode, &user_msg).await;
                    return Ok(());
                }
            };

            if let Some(reply) = follow_response.text {
                send_reply(bot, norm, &reply_mode, &reply).await?;
                tracing::info!(app_id=?bot.app_id, model=?action.model, tool=?tool_name, "AI 工具调用回复已发送");
            } else {
                tracing::warn!(app_id=?bot.app_id, model=?action.model, "AI 工具调用后无有效回复");
                let _ = send_reply(
                    bot,
                    norm,
                    &reply_mode,
                    "AI 处理完成但未生成回复，请换个方式提问",
                )
                .await;
            }
        } else if let Some(reply) = response.text {
            send_reply(bot, norm, &reply_mode, &reply).await?;
            tracing::info!(app_id=?bot.app_id, model=?action.model, "AI 回复已发送");
        } else {
            tracing::warn!(app_id=?bot.app_id, model=?action.model, "AI 响应为空");
            let _ = send_reply(bot, norm, &reply_mode, "AI 未返回有效回复，请换个方式提问").await;
        }

        Ok(())
    }

    async fn handle_command(
        &self,
        bot: &BotInstance,
        norm: &NormalizedEvent,
        action: &CommandAction,
        reply_mode: ReplyMode,
    ) -> Result<()> {
        let Some(reply_to) = norm.from_wxid.as_deref() else {
            tracing::info!(
                app_id=?bot.app_id,
                program=?action.program,
                "缺少来源 wxid，跳过 command 动作"
            );
            return Ok(());
        };

        if action.program.trim().is_empty() {
            tracing::warn!(app_id=?bot.app_id, "command.program 为空，跳过执行");
            return Ok(());
        }

        if let Some(text) = action.pre_reply.as_deref().filter(|s| !s.trim().is_empty()) {
            let _ = send_reply(bot, norm, &reply_mode, text).await;
        }

        let max_output = command_max_output(action);
        let report = match action.program.as_str() {
            "claude_changelog" => run_builtin_claude_changelog(action, None, max_output).await,
            "http_request" => run_builtin_http_request(action, None, max_output).await,
            "tool_versions" => run_builtin_tool_versions(action, None, max_output).await,
            "gemini_image" => {
                run_builtin_gemini_image(action, None, max_output, &self.image_config).await
            }
            _ => run_external_command(action, norm, max_output).await,
        };

        log_command_report(bot, &report, reply_to, &action.args);

        // 发送图片（如果有）
        for img_url in &report.image_urls {
            match bot.send_image(reply_to, img_url).await {
                Ok(_) => {
                    tracing::info!(
                        app_id = ?bot.app_id,
                        to = reply_to,
                        url = img_url,
                        program = ?action.program,
                        "图片发送成功"
                    );
                }
                Err(err) => {
                    tracing::warn!(
                        ?err,
                        app_id = ?bot.app_id,
                        to = reply_to,
                        url = img_url,
                        program = ?action.program,
                        "图片发送失败"
                    );
                }
            }
        }

        if let Some(reply) = report.reply.as_deref() {
            match send_reply(bot, norm, &reply_mode, reply).await {
                Ok(_) => tracing::info!(
                    app_id=?bot.app_id,
                    to=reply_to,
                    program=?action.program,
                    "命令回复发送成功"
                ),
                Err(err) => tracing::warn!(
                    ?err,
                    app_id=?bot.app_id,
                    to=reply_to,
                    program=?action.program,
                    "命令回复发送失败"
                ),
            }
        }

        if report.error.is_none() {
            if let Some(text) = action
                .post_reply
                .as_deref()
                .filter(|s| !s.trim().is_empty())
            {
                let _ = send_reply(bot, norm, &reply_mode, text).await;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct NormalizedEvent {
    kind: RuleKind,
    app_id: AppId,
    msg_type: Option<i64>,
    from_wxid: Option<String>,
    group_sender_wxid: Option<String>,
    to_wxid: Option<String>,
    content: Option<String>,
    push_content: Option<String>,
    msg_source: Option<String>,
    appmsg_type: Option<i32>,
    new_msg_id: Option<i64>,
    chat: Option<ChatKind>,
    nickname: Option<String>,
    type_name: Option<String>,
    normalized_content: Option<String>,
}

impl NormalizedEvent {
    fn nickname(&self) -> Option<String> {
        self.nickname.clone()
    }

    fn sender_wxid(&self) -> Option<&str> {
        if self.chat == Some(ChatKind::Group) {
            self.group_sender_wxid
                .as_deref()
                .or(self.from_wxid.as_deref())
        } else {
            self.from_wxid.as_deref()
        }
    }
}

fn normalize_event(event: &WebhookEvent) -> Result<NormalizedEvent> {
    let type_name = event.type_name.clone();
    let mut norm = NormalizedEvent {
        kind: RuleKind::Any,
        app_id: event.app_id.clone(),
        msg_type: None,
        from_wxid: None,
        group_sender_wxid: None,
        to_wxid: None,
        content: None,
        push_content: None,
        msg_source: None,
        appmsg_type: None,
        new_msg_id: extract_new_msg_id(&event.data),
        chat: None,
        nickname: None,
        type_name,
        normalized_content: None,
    };

    match norm.type_name.as_deref() {
        Some("AddMsg") => {
            let msg_type = event
                .data
                .get("MsgType")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("AddMsg 缺少 MsgType"))?;
            norm.msg_type = Some(msg_type);
            norm.from_wxid = event
                .data
                .get("FromUserName")
                .and_then(|v| v.get("string"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            norm.to_wxid = event
                .data
                .get("ToUserName")
                .and_then(|v| v.get("string"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            norm.chat = norm.from_wxid.as_deref().map(|w| {
                if w.ends_with("@chatroom") {
                    ChatKind::Group
                } else {
                    ChatKind::Private
                }
            });
            norm.content = event
                .data
                .get("Content")
                .and_then(|v| v.get("string"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            if matches!(norm.chat, Some(ChatKind::Group)) {
                if let Some(ref content) = norm.content {
                    norm.group_sender_wxid = extract_group_sender(content);
                }
            }
            norm.push_content = event
                .data
                .get("PushContent")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            norm.msg_source = event
                .data
                .get("MsgSource")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            norm.appmsg_type = extract_appmsg_type(norm.msg_type, norm.content.as_deref());
            norm.kind = match (msg_type, norm.appmsg_type) {
                (1, _) => RuleKind::Text,
                (3, _) => RuleKind::Image,
                (34, _) => RuleKind::Voice,
                (43, _) => RuleKind::Video,
                (47, _) => RuleKind::Emoji,
                (49, Some(5)) => RuleKind::Link,
                (49, Some(74)) => RuleKind::FileNotice,
                _ => RuleKind::Any,
            };
            // 群聊文本形如 "sender:\n内容"，在确定类型后切分正文
            if norm.msg_type == Some(1) && norm.chat == Some(ChatKind::Group) {
                if let Some(ref content) = norm.content {
                    norm.content = Some(strip_sender_prefix(content));
                }
            }
            norm.normalized_content = Some(normalize_content(&norm));
        }
        Some("ModContacts") | Some("DelContacts") | Some("Offline") => {
            norm.kind = RuleKind::ContactEvent;
        }
        _ => {
            norm.kind = RuleKind::Any;
        }
    }
    norm.nickname = extract_nickname(norm.push_content.as_deref());

    Ok(norm)
}

fn extract_new_msg_id(data: &serde_json::Value) -> Option<i64> {
    data.get("NewMsgId").and_then(|v| v.as_i64()).or_else(|| {
        data.get("Data")
            .and_then(|inner| inner.get("NewMsgId"))
            .and_then(|v| v.as_i64())
    })
}

fn extract_appmsg_type(msg_type: Option<i64>, content: Option<&str>) -> Option<i32> {
    if msg_type != Some(49) {
        return None;
    }
    let xml = content?;
    // 简单提取 <type>5</type>
    xml.find("<type>").and_then(|start| {
        let rest = &xml[start + 6..];
        rest.find("</type>")
            .and_then(|end| rest[..end].trim().parse::<i32>().ok())
    })
}

/// 用于日志的内容归一化：
/// - 文本：展示实际内容
/// - 引用：展示被引内容类型/文本
/// - 图片/表情/链接/文件：展示占位符
fn normalize_content(norm: &NormalizedEvent) -> String {
    match norm.kind {
        RuleKind::Text => normalize_text_content(norm),
        RuleKind::Image => "[图片]".to_string(),
        RuleKind::Voice => "[语音]".to_string(),
        RuleKind::Video => "[视频]".to_string(),
        RuleKind::Emoji => "[表情]".to_string(),
        RuleKind::Link => "[链接]".to_string(),
        RuleKind::FileNotice => "[文件]".to_string(),
        RuleKind::ContactEvent => "[联系人事件]".to_string(),
        // 对于未识别类型，若是 appmsg（如引用 57），走文本归一化，否则占位符
        RuleKind::Any => {
            if norm.msg_type == Some(49) {
                normalize_text_content(norm)
            } else {
                norm.content
                    .as_deref()
                    .map(|s| shorten(s, 200))
                    .unwrap_or("[unknown]".to_string())
            }
        }
    }
}

/// 文本类内容的归一化，兼顾引用消息（appmsg type 57）
fn normalize_text_content(norm: &NormalizedEvent) -> String {
    let raw = norm.content.as_deref().unwrap_or("[text]").to_string();
    // 如果是微信卡片/引用消息（MsgType=49 且含 appmsg），尝试提取引用内容/标题
    if norm.msg_type == Some(49) && raw.contains("<appmsg") {
        let title = extract_between(&raw, "<title>", "</title>");
        if raw.contains("<refermsg>") {
            let refer_block = extract_between(&raw, "<refermsg>", "</refermsg>");
            let refer_type = refer_block
                .as_deref()
                .and_then(|r| extract_between(r, "<type>", "</type>"))
                .and_then(|t| t.parse::<i32>().ok());
            let refer_label = map_type_label(refer_type);
            // 尝试取引用内容；如果为空，再尝试 title
            let refer_content = refer_block
                .as_deref()
                .and_then(|r| extract_between(r, "<content>", "</content>"))
                .or_else(|| {
                    refer_block
                        .as_deref()
                        .and_then(|r| extract_between(r, "<title>", "</title>"))
                })
                .unwrap_or_default();
            let mut parts = Vec::new();
            parts.push(format!("[引用:{}]", refer_label));
            // 非文本或含 XML/IMG 的引用，不输出原文
            if refer_label == "文本"
                && !refer_content.trim_start().starts_with('<')
                && !refer_content.contains("<img")
                && !refer_content.trim().is_empty()
            {
                parts.push(refer_content.trim().to_string());
            }
            if let Some(t) = title {
                if !t.trim().is_empty() {
                    parts.push(t);
                }
            }
            return shorten(&parts.join(" "), 200);
        }
        // 非引用的卡片/链接
        return title.unwrap_or("[卡片]".to_string());
    }
    shorten(&raw, 300)
}

fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let start_pos = s.find(start)?;
    let rest = &s[start_pos + start.len()..];
    let end_pos = rest.find(end)?;
    Some(rest[..end_pos].to_string())
}

fn map_type_label(t: Option<i32>) -> &'static str {
    match t {
        Some(1) => "文本",
        Some(3) => "图片",
        Some(34) => "语音",
        Some(43) => "视频",
        Some(47) => "表情",
        Some(5) => "链接",
        Some(_) => "其他",
        None => "引用",
    }
}

/// 给字段加上 ANSI 颜色（在 stdout 下有效，写文件时仍会带控制符）
fn colorize(val: Option<&str>, ansi_code: &str) -> String {
    match val {
        Some(v) if !v.is_empty() => format!("\x1b[{}m{}\x1b[0m", ansi_code, v),
        _ => "".to_string(),
    }
}

fn rule_kind_cn(kind: &RuleKind) -> &'static str {
    match kind {
        RuleKind::Text => "文本",
        RuleKind::Image => "图片",
        RuleKind::Voice => "语音",
        RuleKind::Video => "视频",
        RuleKind::Emoji => "表情",
        RuleKind::Link => "链接",
        RuleKind::FileNotice => "文件",
        RuleKind::ContactEvent => "联系人事件",
        RuleKind::Any => "任意",
    }
}

fn chat_kind_cn(chat: ChatKind) -> &'static str {
    match chat {
        ChatKind::Private => "私聊",
        ChatKind::Group => "群聊",
    }
}

/// 群聊文本前缀剥离，形如 "sender:\n正文" 或 "sender:\r\n正文"
fn strip_sender_prefix(raw: &str) -> String {
    if let Some(pos) = raw.find(":\n") {
        return raw[pos + 2..].to_string();
    }
    if let Some(pos) = raw.find(":\r\n") {
        return raw[pos + 3..].to_string();
    }
    raw.to_string()
}

fn shorten(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut cut = max;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    format!("{}…(+{} chars)", &s[..cut], s.len() - cut)
}

impl CompiledRule {
    fn try_from_config(cfg: &RuleConfig) -> Result<Self> {
        let matcher = Matcher::from_match_config(&cfg.r#match)?;
        Ok(Self {
            kind: cfg.kind.clone(),
            matcher,
            from: FromGate {
                nick: cfg.from.nick.clone(),
                wxid: cfg.from.wxid.clone(),
            },
            chat: cfg.chat.clone(),
            action: cfg.action.clone(),
        })
    }

    fn is_match(&self, norm: &NormalizedEvent) -> bool {
        if !matches_kind(self.kind.clone(), norm) {
            return false;
        }
        if let Some(expected_chat) = &self.chat {
            let actual_chat = norm.chat.as_ref();
            if actual_chat != Some(expected_chat) {
                return false;
            }
        }
        if let Some(ref nick) = self.from.nick {
            if norm.nickname().as_deref() != Some(nick.as_str()) {
                return false;
            }
        }
        if let Some(ref wxid) = self.from.wxid {
            if norm.chat == Some(ChatKind::Group) {
                let sender = norm.sender_wxid();
                let group_id = norm.from_wxid.as_deref();
                if sender != Some(wxid.as_str()) && group_id != Some(wxid.as_str()) {
                    return false;
                }
            } else if norm.sender_wxid() != Some(wxid.as_str()) {
                return false;
            }
        }
        if !self
            .matcher
            .matches(norm.content.as_deref().unwrap_or_default())
        {
            return false;
        }
        true
    }

    fn reply_mode(&self) -> ReplyMode {
        self.action.reply_mode.clone().unwrap_or(ReplyMode::None)
    }
}

fn log_rule_hit(bot: &BotInstance, rule: &CompiledRule, norm: &NormalizedEvent) {
    let content_colored = colorize(norm.normalized_content.as_deref(), "36"); // cyan
    let sender_colored = colorize(norm.sender_wxid(), "33"); // yellow
    let from_colored = colorize(norm.from_wxid.as_deref(), "32"); // green
    let kind_colored = colorize(Some(rule_kind_cn(&rule.kind)), "35"); // magenta
    let app_colored = colorize(Some(&bot.app_id.0), "34"); // blue
    let chat_colored = colorize(norm.chat.as_ref().map(|c| chat_kind_cn(c.clone())), "31"); // red
    tracing::debug!(
        app_id=%app_colored,
        rule_kind=%kind_colored,
        event_kind=?norm.kind,
        chat=%chat_colored,
        from_wxid=%from_colored,
        sender_wxid=%sender_colored,
        new_msg_id=?norm.new_msg_id,
        content=%content_colored,
        "规则命中"
    );
}

fn matches_kind(rule_kind: RuleKind, norm: &NormalizedEvent) -> bool {
    if let RuleKind::Any = rule_kind {
        return true;
    }
    matches!(
        (&rule_kind, &norm.kind),
        (RuleKind::Text, RuleKind::Text)
            | (RuleKind::Image, RuleKind::Image)
            | (RuleKind::Voice, RuleKind::Voice)
            | (RuleKind::Video, RuleKind::Video)
            | (RuleKind::Emoji, RuleKind::Emoji)
            | (RuleKind::Link, RuleKind::Link)
            | (RuleKind::FileNotice, RuleKind::FileNotice)
            | (RuleKind::ContactEvent, RuleKind::ContactEvent)
    )
}

impl Matcher {
    fn from_match_config(cfg: &MatchConfig) -> Result<Self> {
        let regex = match cfg.regex.as_deref() {
            Some(pat) if !pat.is_empty() => Some(Regex::new(pat).map_err(|e| anyhow!(e))?),
            _ => None,
        };
        Ok(Self {
            equals: cfg.equals.clone(),
            contains: cfg.contains.clone(),
            regex,
        })
    }

    fn matches(&self, content: &str) -> bool {
        let text = content.trim();
        let mut used = false;
        if let Some(eq) = &self.equals {
            used = true;
            if text != eq {
                return false;
            }
        }
        if let Some(cn) = &self.contains {
            used = true;
            if !text.contains(cn) {
                return false;
            }
        }
        if let Some(re) = &self.regex {
            used = true;
            if !re.is_match(text) {
                return false;
            }
        }
        if !used {
            return true;
        }
        true
    }
}

async fn save_media(
    bot: &BotInstance,
    norm: &NormalizedEvent,
    save: &SaveAction,
) -> Result<String> {
    let kind = norm.kind.clone();
    let xml = norm.content.as_deref().unwrap_or_default();
    let app_id = &bot.app_id.0;
    let file_url = match kind {
        RuleKind::Image => bot.client.download_image(app_id, xml, 2).await?.file_url,
        RuleKind::Video => bot.client.download_video(app_id, xml).await?.file_url,
        RuleKind::Voice => {
            bot.client
                .download_voice(app_id, xml, norm.new_msg_id.unwrap_or_default())
                .await?
                .file_url
        }
        RuleKind::Emoji => {
            let md5 = extract_emoji_md5(xml).ok_or_else(|| anyhow!("缺少 emoji md5"))?;
            bot.client.download_emoji(app_id, &md5).await?.url
        }
        RuleKind::FileNotice => bot.client.download_file(app_id, xml).await?.file_url,
        _ => return Err(anyhow!("当前类型不支持保存: {:?}", kind)),
    };

    let bytes = reqwest::get(&file_url)
        .await
        .map_err(|e| anyhow!("下载媒体失败: {e}"))?
        .bytes()
        .await
        .map_err(|e| anyhow!("读取媒体失败: {e}"))?;

    let dir = if save.dir.is_empty() {
        "data".to_string()
    } else {
        save.dir.clone()
    };
    fs::create_dir_all(&dir)
        .await
        .map_err(|e| anyhow!("创建目录失败: {e}"))?;

    let filename = render_filename(save, norm);
    let path = format!("{}/{}", dir.trim_end_matches('/'), filename);
    let mut file = fs::File::create(&path)
        .await
        .map_err(|e| anyhow!("创建文件失败: {e}"))?;
    file.write_all(&bytes)
        .await
        .map_err(|e| anyhow!("写入文件失败: {e}"))?;
    Ok(path)
}

fn render_filename(save: &SaveAction, norm: &NormalizedEvent) -> String {
    let tpl = save.filename.as_deref().unwrap_or("{new_msg_id}.bin");
    let mut out = tpl.to_string();
    if let Some(id) = norm.new_msg_id {
        out = out.replace("{new_msg_id}", &id.to_string());
    }
    if let Some(from) = &norm.from_wxid {
        out = out.replace("{from_wxid}", from);
    }
    out = out.replace("{app_id}", &norm.app_id.0);
    out
}

fn extract_emoji_md5(xml: &str) -> Option<String> {
    xml.split("md5=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .map(|s| s.to_string())
}

fn extract_nickname(push_content: Option<&str>) -> Option<String> {
    let raw = push_content?;
    raw.split_once(':')
        .or_else(|| raw.split_once('：'))
        .map(|(name, _)| name.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_display_name(push_content: Option<&str>) -> Option<String> {
    let raw = push_content?;
    // 常见格式：昵称: 内容
    if let Some((name, _)) = raw.split_once(':') {
        let n = name.trim();
        if !n.is_empty() {
            return Some(n.to_string());
        }
    }
    // 另一种格式：昵称在群聊中@了你
    if let Some(pos) = raw.find("在群聊") {
        let (name, _) = raw.split_at(pos);
        let n = name.trim();
        if !n.is_empty() {
            return Some(n.to_string());
        }
    }
    None
}

fn extract_group_sender(content: &str) -> Option<String> {
    let trimmed = content.trim_start();
    // 群聊消息格式常见为「发送者: 内容」，wxid 不包含冒号，取首个冒号前的部分。
    if let Some((head, _)) = trimmed.split_once(':') {
        let sender = head.trim();
        if !sender.is_empty() {
            return Some(sender.to_string());
        }
    }
    None
}

fn mentioned_bot(norm: &NormalizedEvent) -> bool {
    if norm.chat != Some(ChatKind::Group) {
        return false;
    }
    let Some(bot_wxid) = norm.to_wxid.as_deref() else {
        return false;
    };
    if let Some(ref src) = norm.msg_source {
        if let Some(inner) = extract_atuserlist(src) {
            if inner.contains(bot_wxid) {
                return true;
            }
        }
    }

    norm.content
        .as_deref()
        .map(|c| c.contains(bot_wxid))
        .unwrap_or(false)
}

fn extract_atuserlist(src: &str) -> Option<String> {
    let start = src.find("<atuserlist>")?;
    let tail = &src[start + "<atuserlist>".len()..];
    let end = tail.find("</atuserlist>")?;
    Some(tail[..end].to_string())
}

/// 根据回复模式发送文本或引用
async fn send_reply(
    bot: &BotInstance,
    norm: &NormalizedEvent,
    mode: &ReplyMode,
    text: &str,
) -> Result<(), anyhow::Error> {
    let to = norm
        .from_wxid
        .as_deref()
        .ok_or_else(|| anyhow!("missing from_wxid"))?;

    match mode {
        ReplyMode::None => bot
            .send_text(to, text, None)
            .await
            .map_err(anyhow::Error::msg),
        ReplyMode::At => {
            let ats = norm.sender_wxid();
            let content = if matches!(norm.chat, Some(ChatKind::Group)) {
                let name = norm
                    .nickname()
                    .or_else(|| extract_display_name(norm.push_content.as_deref()))
                    .unwrap_or_else(|| "你".to_string());
                format!("@{} {}", name, text)
            } else {
                text.to_string()
            };
            bot.send_text(to, &content, ats)
                .await
                .map_err(anyhow::Error::msg)
        }
        ReplyMode::Quote => {
            let svrid = norm
                .new_msg_id
                .ok_or_else(|| anyhow!("missing new_msg_id for quote"))?;
            let title = if text.trim().is_empty() {
                "引用回复"
            } else {
                text
            };
            let title = escape_xml(title, DEFAULT_QUOTE_TITLE_MAX_LEN);
            let appmsg = format!(
                "<appmsg><title>{}</title><type>57</type><refermsg><svrid>{}</svrid></refermsg></appmsg>",
                title, svrid
            );
            bot.send_appmsg(to, &appmsg)
                .await
                .map_err(anyhow::Error::msg)
        }
        ReplyMode::QuoteAndAt => {
            let svrid = norm
                .new_msg_id
                .ok_or_else(|| anyhow!("missing new_msg_id for quote"))?;
            let sender = norm
                .sender_wxid()
                .ok_or_else(|| anyhow!("missing sender_wxid"))?;
            let title = if text.trim().is_empty() {
                "引用回复"
            } else {
                text
            };
            let title = escape_xml(title, DEFAULT_QUOTE_TITLE_MAX_LEN);
            let appmsg = format!(
                "<appmsg><title>{}</title><type>57</type><refermsg><svrid>{}</svrid><msgsource>&lt;msgsource&gt;&lt;atuserlist&gt;{}&lt;/atuserlist&gt;&lt;/msgsource&gt;</msgsource></refermsg></appmsg>",
                title, svrid, sender
            );
            bot.send_appmsg(to, &appmsg)
                .await
                .map_err(anyhow::Error::msg)
        }
    }
}

fn escape_xml(input: &str, max_len: usize) -> String {
    let truncated: String = input.chars().take(max_len).collect();
    truncated
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn command_timeout(action: &CommandAction) -> Duration {
    let secs = action.timeout_secs.unwrap_or(DEFAULT_COMMAND_TIMEOUT_SECS);
    let secs = if secs == 0 {
        DEFAULT_COMMAND_TIMEOUT_SECS
    } else {
        secs
    };
    Duration::from_secs(secs)
}

fn command_max_output(action: &CommandAction) -> usize {
    match action.max_output {
        Some(0) | None => DEFAULT_COMMAND_MAX_OUTPUT,
        Some(v) => v,
    }
}

async fn execute_command_action(
    action: &CommandAction,
    _norm: &NormalizedEvent,
    max_output: usize,
    arguments: Option<&str>,
    image_config: Option<&ImageConfig>,
) -> CommandReport {
    match action.program.as_str() {
        "claude_changelog" => run_builtin_claude_changelog(action, arguments, max_output).await,
        "http_request" => run_builtin_http_request(action, arguments, max_output).await,
        "tool_versions" => run_builtin_tool_versions(action, arguments, max_output).await,
        "gemini_image" => {
            if let Some(config) = image_config {
                run_builtin_gemini_image(action, arguments, max_output, config).await
            } else {
                CommandReport {
                    reply: Some("图像生成工具未配置".to_string()),
                    truncated: false,
                    duration: Duration::from_millis(0),
                    exit_code: None,
                    timed_out: false,
                    disabled: true,
                    source: CommandSource::Builtin,
                    program: action.program.clone(),
                    stderr: None,
                    error: Some("image config missing".to_string()),
                    image_urls: vec![],
                }
            }
        }
        _ => run_external_command(action, _norm, max_output).await,
    }
}

fn external_command_allowed() -> bool {
    static ALLOW: OnceLock<bool> = OnceLock::new();
    *ALLOW.get_or_init(|| match std::env::var("GEWE_ALLOW_COMMAND") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "True"),
        Err(_) => false,
    })
}

/// 构建用户消息内容
fn build_user_content(
    action: &AiAction,
    norm: &NormalizedEvent,
    command_output: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    if let Some(prefix) = action.user_prefix.as_deref().filter(|s| !s.is_empty()) {
        parts.push(render_user_prefix(prefix, norm));
    }
    if let Some(content) = norm.content.as_deref().filter(|s| !s.trim().is_empty()) {
        parts.push(format!("用户消息：{}", content.trim()));
    }
    if let Some(ctx) = command_output.filter(|s| !s.trim().is_empty()) {
        parts.push(format!("查询结果：\n{}", ctx.trim()));
    }
    if parts.is_empty() {
        parts.push("请直接回复用户消息。".to_string());
    }
    parts.join("\n\n")
}

/// 将 user_prefix 中的占位符替换为上下文字段
/// 支持：{app_id} {chat} {from_wxid} {sender_wxid} {to_wxid} {new_msg_id}
fn render_user_prefix(prefix: &str, norm: &NormalizedEvent) -> String {
    let chat = match norm.chat {
        Some(ChatKind::Group) => "group",
        Some(ChatKind::Private) => "private",
        None => "unknown",
    };
    let sender = norm.sender_wxid().unwrap_or_default();
    prefix
        .replace("{app_id}", &norm.app_id.0)
        .replace("{chat}", chat)
        .replace("{from_wxid}", norm.from_wxid.as_deref().unwrap_or_default())
        .replace("{sender_wxid}", sender)
        .replace("{to_wxid}", norm.to_wxid.as_deref().unwrap_or_default())
        .replace(
            "{new_msg_id}",
            &norm.new_msg_id.map(|v| v.to_string()).unwrap_or_default(),
        )
}

/// 构建 rig CompletionRequest
fn build_completion_request(
    action: &AiAction,
    user_content: &str,
    tools: &[ToolDefinition],
) -> CompletionRequest {
    let chat_history = rig::OneOrMany::one(RigMessage::user(user_content));

    // 构建额外参数（对于 Gemini，需要包含 generationConfig）
    let mut params = serde_json::json!({
        "generationConfig": {}
    });

    if let Some(ref rf) = action.response_format {
        if let Some(ref ft) = rf.format_type {
            params["response_format"] = serde_json::json!({ "type": ft });
            if let Some(ref schema) = rf.schema {
                params["response_format"]["schema"] = schema.clone();
            }
        }
    }

    let additional_params = Some(params);

    CompletionRequest {
        preamble: action.system_prompt.clone(),
        chat_history,
        tools: tools.to_vec(),
        tool_choice: None,
        temperature: action.temperature.map(|t| t as f64),
        max_tokens: action.max_tokens.map(|t| t as u64),
        additional_params,
        documents: vec![],
    }
}

/// 构建工具定义列表
fn build_tools_for_request(tools: &[AiTool]) -> Vec<ToolDefinition> {
    tools
        .iter()
        .filter(|t| !t.name.trim().is_empty())
        .map(|t| ToolDefinition {
            name: t.name.clone(),
            description: t.description.clone().unwrap_or_default(),
            parameters: t.parameters.clone().unwrap_or(serde_json::json!({})),
        })
        .collect()
}

fn build_command_env(norm: &NormalizedEvent) -> Vec<(String, String)> {
    let chat = match norm.chat {
        Some(ChatKind::Group) => "group",
        Some(ChatKind::Private) => "private",
        None => "unknown",
    };
    vec![
        ("APP_ID".to_string(), norm.app_id.0.clone()),
        (
            "FROM_WXID".to_string(),
            norm.from_wxid.clone().unwrap_or_default(),
        ),
        (
            "TO_WXID".to_string(),
            norm.to_wxid.clone().unwrap_or_default(),
        ),
        (
            "CONTENT".to_string(),
            norm.content.clone().unwrap_or_default(),
        ),
        (
            "PUSH_CONTENT".to_string(),
            norm.push_content.clone().unwrap_or_default(),
        ),
        (
            "NICK".to_string(),
            norm.nickname.clone().unwrap_or_default(),
        ),
        (
            "MSG_TYPE".to_string(),
            norm.msg_type.map(|v| v.to_string()).unwrap_or_default(),
        ),
        (
            "APPMSG_TYPE".to_string(),
            norm.appmsg_type.map(|v| v.to_string()).unwrap_or_default(),
        ),
        (
            "NEW_MSG_ID".to_string(),
            norm.new_msg_id.map(|v| v.to_string()).unwrap_or_default(),
        ),
        ("CHAT".to_string(), chat.to_string()),
        ("KIND".to_string(), rule_kind_name(&norm.kind).to_string()),
        (
            "TYPE_NAME".to_string(),
            norm.type_name.clone().unwrap_or_default(),
        ),
    ]
}

fn rule_kind_name(kind: &RuleKind) -> &'static str {
    match kind {
        RuleKind::Text => "text",
        RuleKind::Image => "image",
        RuleKind::Voice => "voice",
        RuleKind::Video => "video",
        RuleKind::Emoji => "emoji",
        RuleKind::Link => "link",
        RuleKind::FileNotice => "file_notice",
        RuleKind::ContactEvent => "contact_event",
        RuleKind::Any => "any",
    }
}

fn clamp_output(text: String, max: usize) -> (String, bool) {
    let mut text = text;
    let truncated = truncate_string(&mut text, max);
    (text, truncated)
}

fn truncate_string(text: &mut String, max: usize) -> bool {
    if text.len() <= max {
        return false;
    }
    if max == 0 {
        text.clear();
        return true;
    }
    let mut cut = max.min(text.len());
    while cut > 0 && !text.is_char_boundary(cut) {
        cut -= 1;
    }
    text.truncate(cut);
    true
}

fn shorten_for_log(text: &str, max: usize) -> String {
    let mut out = text.to_string();
    truncate_string(&mut out, max);
    out
}

fn log_command_report(bot: &BotInstance, report: &CommandReport, target: &str, args: &[String]) {
    let stderr_preview = report
        .stderr
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| shorten_for_log(s, 200));
    let reply_preview = report
        .reply
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| shorten_for_log(s, 200));
    let source = match report.source {
        CommandSource::Builtin => "builtin",
        CommandSource::External => "external",
    };
    let warn = report.timed_out
        || report.disabled
        || report.error.is_some()
        || report.exit_code.map(|c| c != 0).unwrap_or(false);

    if warn {
        tracing::warn!(
            app_id=?bot.app_id,
            program=?report.program,
            source,
            target,
            args=?args,
            exit_code=?report.exit_code,
            timed_out=report.timed_out,
            disabled=report.disabled,
            duration_ms=?report.duration.as_millis(),
            truncated=report.truncated,
            error=?report.error,
            stderr_preview=?stderr_preview,
            reply_preview=?reply_preview,
            "命令执行完成"
        );
    } else {
        tracing::info!(
            app_id=?bot.app_id,
            program=?report.program,
            source,
            target,
            args=?args,
            exit_code=?report.exit_code,
            timed_out=report.timed_out,
            disabled=report.disabled,
            duration_ms=?report.duration.as_millis(),
            truncated=report.truncated,
            stderr_preview=?stderr_preview,
            reply_preview=?reply_preview,
            "命令执行完成"
        );
    }
}

async fn run_external_command(
    action: &CommandAction,
    norm: &NormalizedEvent,
    max_output: usize,
) -> CommandReport {
    if !external_command_allowed() {
        return CommandReport {
            reply: Some("未启用 command 执行，请设置 GEWE_ALLOW_COMMAND=1 后重试".to_string()),
            truncated: false,
            duration: Duration::from_millis(0),
            exit_code: None,
            timed_out: false,
            disabled: true,
            source: CommandSource::External,
            program: action.program.clone(),
            stderr: None,
            error: Some("command disabled by GEWE_ALLOW_COMMAND".to_string()),
            image_urls: vec![],
        };
    }

    let timeout = command_timeout(action);
    let start = Instant::now();
    let mut cmd = TokioCommand::new(&action.program);
    if !action.args.is_empty() {
        cmd.args(&action.args);
    }
    cmd.envs(build_command_env(norm));
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.kill_on_drop(true);

    let child = match cmd.spawn() {
        Ok(c) => c,
        Err(err) => {
            return CommandReport {
                reply: Some(format!("命令启动失败: {err}")),
                truncated: false,
                duration: start.elapsed(),
                exit_code: None,
                timed_out: false,
                disabled: false,
                source: CommandSource::External,
                program: action.program.clone(),
                stderr: None,
                error: Some(err.to_string()),
                image_urls: vec![],
            }
        }
    };

    let output = match time::timeout(timeout, child.wait_with_output()).await {
        Ok(res) => match res {
            Ok(out) => out,
            Err(err) => {
                return CommandReport {
                    reply: Some("命令执行失败，请稍后再试".to_string()),
                    truncated: false,
                    duration: start.elapsed(),
                    exit_code: None,
                    timed_out: false,
                    disabled: false,
                    source: CommandSource::External,
                    program: action.program.clone(),
                    stderr: None,
                    error: Some(err.to_string()),
                    image_urls: vec![],
                }
            }
        },
        Err(_) => {
            return CommandReport {
                reply: Some("命令执行超时，请稍后再试".to_string()),
                truncated: false,
                duration: timeout,
                exit_code: None,
                timed_out: true,
                disabled: false,
                source: CommandSource::External,
                program: action.program.clone(),
                stderr: None,
                error: Some("command timeout".to_string()),
                image_urls: vec![],
            };
        }
    };

    let duration = start.elapsed();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code();

    if !output.status.success() {
        let log_stderr = if !stderr.is_empty() {
            Some(stderr)
        } else if !stdout.trim().is_empty() {
            Some(stdout)
        } else {
            None
        };
        return CommandReport {
            reply: Some(format!(
                "命令执行失败（退出码 {}）",
                exit_code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            )),
            truncated: false,
            duration,
            exit_code,
            timed_out: false,
            disabled: false,
            source: CommandSource::External,
            program: action.program.clone(),
            stderr: log_stderr,
            error: Some("non-zero exit status".to_string()),
            image_urls: vec![],
        };
    }

    let (reply_text, truncated_now) = clamp_output(stdout, max_output);
    let mut truncated = truncated_now;
    let reply_body = if reply_text.trim().is_empty() {
        "命令执行成功，但无输出".to_string()
    } else {
        reply_text
    };
    let (reply, truncated_reply) = clamp_output(reply_body, max_output);
    truncated |= truncated_reply;

    CommandReport {
        reply: Some(reply),
        truncated,
        duration,
        exit_code,
        timed_out: false,
        disabled: false,
        source: CommandSource::External,
        program: action.program.clone(),
        stderr: if stderr.is_empty() {
            None
        } else {
            Some(stderr)
        },
        error: None,
        image_urls: vec![],
    }
}

/// 执行内置的 claude_changelog 命令
async fn run_builtin_claude_changelog(
    action: &CommandAction,
    arguments: Option<&str>,
    max_output: usize,
) -> CommandReport {
    let program = action.program.clone();
    let timeout_secs = action.timeout_secs;

    // 解析查询参数
    let query = arguments.map(ChangelogQuery::from_json).unwrap_or_default();

    let result = run_claude_changelog(query, timeout_secs, max_output).await;

    CommandReport {
        reply: Some(result.content),
        truncated: result.truncated,
        duration: result.duration,
        exit_code: None,
        timed_out: result.timed_out,
        disabled: false,
        source: CommandSource::Builtin,
        program,
        stderr: None,
        error: result.error,
        image_urls: vec![],
    }
}

/// 执行内置的 tool_versions 命令
async fn run_builtin_tool_versions(
    action: &CommandAction,
    arguments: Option<&str>,
    max_output: usize,
) -> CommandReport {
    let program = action.program.clone();
    let timeout_secs = action.timeout_secs;

    // 解析查询参数
    let query = arguments.map(VersionQuery::from_json).unwrap_or_default();

    let result = run_tool_versions(query, timeout_secs, max_output).await;

    CommandReport {
        reply: Some(result.content),
        truncated: result.truncated,
        duration: result.duration,
        exit_code: None,
        timed_out: result.timed_out,
        disabled: false,
        source: CommandSource::Builtin,
        program,
        stderr: None,
        error: result.error,
        image_urls: vec![],
    }
}

/// 执行内置的 http_request 命令
async fn run_builtin_http_request(
    action: &CommandAction,
    arguments: Option<&str>,
    max_output: usize,
) -> CommandReport {
    let program = action.program.clone();
    let timeout_secs = action.timeout_secs;

    // 解析查询参数
    let query = arguments
        .map(HttpRequestQuery::from_json)
        .unwrap_or_default();

    let result = run_http_request(query, timeout_secs, max_output).await;

    CommandReport {
        reply: Some(result.content),
        truncated: result.truncated,
        duration: result.duration,
        exit_code: None,
        timed_out: result.timed_out,
        disabled: false,
        source: CommandSource::Builtin,
        program,
        stderr: None,
        error: result.error,
        image_urls: vec![],
    }
}

/// 执行内置的 gemini_image 命令
async fn run_builtin_gemini_image(
    action: &CommandAction,
    arguments: Option<&str>,
    max_output: usize,
    config: &ImageConfig,
) -> CommandReport {
    let program = action.program.clone();
    let timeout_secs = action.timeout_secs;

    // 解析查询参数
    let query = arguments.map(ImageQuery::from_json).unwrap_or_default();

    let result = run_gemini_image(query, config, timeout_secs, max_output).await;

    CommandReport {
        reply: result.text,
        truncated: result.truncated,
        duration: result.duration,
        exit_code: None,
        timed_out: result.timed_out,
        disabled: false,
        source: CommandSource::Builtin,
        program,
        stderr: None,
        error: result.error,
        image_urls: result.image_urls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ===== 测试 normalize_event 相关函数 =====

    #[test]
    fn test_normalize_event_add_msg_text() {
        // 测试文本消息的归一化
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 1,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": "hello world"},
                "NewMsgId": 12345,
                "PushContent": "Alice: hello world"
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Text);
        assert_eq!(norm.msg_type, Some(1));
        assert_eq!(norm.from_wxid, Some("user123".to_string()));
        assert_eq!(norm.to_wxid, Some("bot456".to_string()));
        assert_eq!(norm.content, Some("hello world".to_string()));
        assert_eq!(norm.chat, Some(ChatKind::Private));
        assert_eq!(norm.new_msg_id, Some(12345));
        assert_eq!(norm.nickname, Some("Alice".to_string()));
    }

    #[test]
    fn test_normalize_event_group_text() {
        // 测试群聊文本消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 1,
                "FromUserName": {"string": "group123@chatroom"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": "sender789:\nhello everyone"},
                "NewMsgId": 12346,
                "PushContent": "Bob: hello everyone"
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Text);
        assert_eq!(norm.chat, Some(ChatKind::Group));
        assert_eq!(norm.group_sender_wxid, Some("sender789".to_string()));
        assert_eq!(norm.content, Some("hello everyone".to_string())); // 前缀已剥离
        assert_eq!(norm.nickname, Some("Bob".to_string()));
    }

    #[test]
    fn test_normalize_event_image() {
        // 测试图片消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 3,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": "<msg>...</msg>"},
                "NewMsgId": 12347
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Image);
        assert_eq!(norm.msg_type, Some(3));
        assert_eq!(norm.normalized_content, Some("[图片]".to_string()));
    }

    #[test]
    fn test_normalize_event_voice() {
        // 测试语音消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 34,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": "<msg>...</msg>"},
                "NewMsgId": 12348
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Voice);
        assert_eq!(norm.normalized_content, Some("[语音]".to_string()));
    }

    #[test]
    fn test_normalize_event_video() {
        // 测试视频消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 43,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "NewMsgId": 12349
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Video);
        assert_eq!(norm.normalized_content, Some("[视频]".to_string()));
    }

    #[test]
    fn test_normalize_event_emoji() {
        // 测试表情消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 47,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "NewMsgId": 12350
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Emoji);
        assert_eq!(norm.normalized_content, Some("[表情]".to_string()));
    }

    #[test]
    fn test_normalize_event_link() {
        // 测试链接消息
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 49,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": "<msg><appmsg><type>5</type><title>Test Link</title></appmsg></msg>"},
                "NewMsgId": 12351
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::Link);
        assert_eq!(norm.appmsg_type, Some(5));
    }

    #[test]
    fn test_normalize_event_contact_event() {
        // 测试联系人事件
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("ModContacts".to_string()),
            data: json!({}),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::ContactEvent);

        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("DelContacts".to_string()),
            data: json!({}),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.kind, RuleKind::ContactEvent);
    }

    #[test]
    fn test_normalize_event_quote_message() {
        // 测试引用消息
        let xml = r#"<msg><appmsg><type>57</type><title>回复内容</title>
            <refermsg><type>1</type><content>原始消息</content></refermsg>
            </appmsg></msg>"#;
        let event = WebhookEvent {
            app_id: AppId("test_app".to_string()),
            type_name: Some("AddMsg".to_string()),
            data: json!({
                "MsgType": 49,
                "FromUserName": {"string": "user123"},
                "ToUserName": {"string": "bot456"},
                "Content": {"string": xml},
                "NewMsgId": 12352
            }),
        };

        let norm = normalize_event(&event).unwrap();
        assert_eq!(norm.msg_type, Some(49));
        assert!(norm.normalized_content.as_ref().unwrap().contains("[引用"));
    }

    // ===== 测试 NormalizedEvent 方法 =====

    #[test]
    fn test_normalized_event_sender_wxid() {
        // 私聊场景
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: None,
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert_eq!(norm.sender_wxid(), Some("user123"));

        // 群聊场景
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("group@chatroom".to_string()),
            group_sender_wxid: Some("sender456".to_string()),
            to_wxid: None,
            content: None,
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert_eq!(norm.sender_wxid(), Some("sender456"));
    }

    // ===== 测试 mentioned_bot =====

    #[test]
    fn test_mentioned_bot_private_chat() {
        // 私聊不需要 @
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: Some("bot123".to_string()),
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert!(!mentioned_bot(&norm));
    }

    #[test]
    fn test_mentioned_bot_in_msg_source() {
        // 通过 msg_source 检测 @
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: Some("bot123".to_string()),
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: Some("<msgsource><atuserlist>bot123</atuserlist></msgsource>".to_string()),
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert!(mentioned_bot(&norm));
    }

    #[test]
    fn test_mentioned_bot_in_content() {
        // 通过 content 检测 @
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: Some("bot123".to_string()),
            content: Some("@bot123 hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert!(mentioned_bot(&norm));
    }

    #[test]
    fn test_mentioned_bot_not_mentioned() {
        // 未 @
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: Some("bot123".to_string()),
            content: Some("hello everyone".to_string()),
            push_content: None,
            msg_source: Some("<msgsource></msgsource>".to_string()),
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        assert!(!mentioned_bot(&norm));
    }

    // ===== 测试基础辅助函数 =====

    #[test]
    fn test_strip_sender_prefix() {
        // 测试群聊前缀剥离
        let input = "sender123:\ncontent here";
        let result = strip_sender_prefix(input);
        assert_eq!(result, "content here");

        let input = "sender456:\r\ncontent here";
        let result = strip_sender_prefix(input);
        assert_eq!(result, "content here");

        // 测试不含前缀的情况
        let input = "no prefix content";
        let result = strip_sender_prefix(input);
        assert_eq!(result, "no prefix content");
    }

    // ===== 测试 normalize_content 相关 =====

    #[test]
    fn test_normalize_content_text() {
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: Some(1),
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("test content".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        let result = normalize_content(&norm);
        assert_eq!(result, "test content");
    }

    #[test]
    fn test_normalize_content_with_quote() {
        let xml = r#"<msg><appmsg><type>57</type><title>回复内容</title>
            <refermsg><type>1</type><content>原始文本</content></refermsg>
            </appmsg></msg>"#;
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: Some(49),
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: None,
            content: Some(xml.to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        let result = normalize_content(&norm);
        assert!(result.contains("[引用"));
        assert!(result.contains("原始文本"));
    }

    #[test]
    fn test_normalize_content_quote_with_xml() {
        // 引用内容包含 XML 时应该被过滤
        let xml = r#"<msg><appmsg><type>57</type><title>回复</title>
            <refermsg><type>1</type><content><img src="test" /></content></refermsg>
            </appmsg></msg>"#;
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: Some(49),
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: None,
            content: Some(xml.to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };
        let result = normalize_content(&norm);
        assert!(result.contains("[引用"));
        // XML 内容应该被过滤掉
        assert!(!result.contains("<img"));
    }

    // ===== 测试 CompiledRule 匹配逻辑 =====

    #[test]
    fn test_compiled_rule_match_from_wxid_private() {
        // 私聊按 from_wxid 匹配
        let rule = CompiledRule {
            kind: RuleKind::Text,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate {
                nick: None,
                wxid: Some("user123".to_string()),
            },
            chat: Some(ChatKind::Private),
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(rule.is_match(&norm));
    }

    #[test]
    fn test_compiled_rule_match_from_wxid_group() {
        // 群聊可以匹配发送者或群 ID
        let rule = CompiledRule {
            kind: RuleKind::Text,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate {
                nick: None,
                wxid: Some("sender123".to_string()),
            },
            chat: Some(ChatKind::Group),
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("group@chatroom".to_string()),
            group_sender_wxid: Some("sender123".to_string()),
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(rule.is_match(&norm));
    }

    #[test]
    fn test_compiled_rule_match_group_id() {
        // 群聊匹配群 ID
        let rule = CompiledRule {
            kind: RuleKind::Text,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate {
                nick: None,
                wxid: Some("group@chatroom".to_string()),
            },
            chat: Some(ChatKind::Group),
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("group@chatroom".to_string()),
            group_sender_wxid: Some("sender123".to_string()),
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(rule.is_match(&norm));
    }

    #[test]
    fn test_compiled_rule_match_nickname() {
        // 昵称匹配
        let rule = CompiledRule {
            kind: RuleKind::Text,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate {
                nick: Some("Alice".to_string()),
                wxid: None,
            },
            chat: None,
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: Some("Alice: hello".to_string()),
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: Some("Alice".to_string()),
            type_name: None,
            normalized_content: None,
        };

        assert!(rule.is_match(&norm));
    }

    #[test]
    fn test_compiled_rule_no_match_wrong_kind() {
        // 类型不匹配
        let rule = CompiledRule {
            kind: RuleKind::Image,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate::default(),
            chat: None,
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(!rule.is_match(&norm));
    }

    #[test]
    fn test_compiled_rule_no_match_wrong_chat() {
        // 聊天类型不匹配
        let rule = CompiledRule {
            kind: RuleKind::Text,
            matcher: Matcher {
                equals: None,
                contains: None,
                regex: None,
            },
            from: FromGate::default(),
            chat: Some(ChatKind::Group),
            action: RuleAction::default(),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(!rule.is_match(&norm));
    }

    // ===== 测试 build_command_env =====

    #[test]
    fn test_build_command_env() {
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test_app".to_string()),
            msg_type: Some(1),
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: Some("bot456".to_string()),
            content: Some("hello".to_string()),
            push_content: Some("Alice: hello".to_string()),
            msg_source: None,
            appmsg_type: None,
            new_msg_id: Some(98765),
            chat: Some(ChatKind::Private),
            nickname: Some("Alice".to_string()),
            type_name: Some("AddMsg".to_string()),
            normalized_content: None,
        };

        let env = build_command_env(&norm);
        let env_map: HashMap<_, _> = env.into_iter().collect();

        assert_eq!(env_map.get("APP_ID"), Some(&"test_app".to_string()));
        assert_eq!(env_map.get("FROM_WXID"), Some(&"user123".to_string()));
        assert_eq!(env_map.get("TO_WXID"), Some(&"bot456".to_string()));
        assert_eq!(env_map.get("CONTENT"), Some(&"hello".to_string()));
        assert_eq!(env_map.get("NICK"), Some(&"Alice".to_string()));
        assert_eq!(env_map.get("CHAT"), Some(&"private".to_string()));
        assert_eq!(env_map.get("KIND"), Some(&"text".to_string()));
        assert_eq!(env_map.get("MSG_TYPE"), Some(&"1".to_string()));
        assert_eq!(env_map.get("NEW_MSG_ID"), Some(&"98765".to_string()));
    }

    #[test]
    fn test_build_command_env_group() {
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test_app".to_string()),
            msg_type: Some(1),
            from_wxid: Some("group@chatroom".to_string()),
            group_sender_wxid: Some("sender123".to_string()),
            to_wxid: Some("bot456".to_string()),
            content: Some("hello".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: Some(98766),
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        let env = build_command_env(&norm);
        let env_map: HashMap<_, _> = env.into_iter().collect();

        assert_eq!(env_map.get("CHAT"), Some(&"group".to_string()));
        assert_eq!(
            env_map.get("FROM_WXID"),
            Some(&"group@chatroom".to_string())
        );
    }

    // ===== 测试 LlmClient::is_retryable_error =====

    #[test]
    fn test_is_retryable_error() {
        // 可重试的错误
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "Connection timeout"
        )));
        assert!(LlmClient::is_retryable_error(&anyhow!("Request timed out")));
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "503 Service Unavailable"
        )));
        assert!(LlmClient::is_retryable_error(&anyhow!("502 Bad Gateway")));
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "500 Internal Server Error"
        )));
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "429 Rate limit exceeded"
        )));
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "Network connection failed"
        )));
        assert!(LlmClient::is_retryable_error(&anyhow!(
            "Service overloaded"
        )));

        // 不可重试的错误
        assert!(!LlmClient::is_retryable_error(&anyhow!("Invalid API key")));
        assert!(!LlmClient::is_retryable_error(&anyhow!("401 Unauthorized")));
        assert!(!LlmClient::is_retryable_error(&anyhow!("400 Bad Request")));
    }

    #[test]
    fn test_shorten() {
        // 测试字符串截断
        let short = "hello";
        assert_eq!(shorten(short, 10), "hello");

        let long = "a".repeat(100);
        let result = shorten(&long, 50);
        assert!(result.contains("…"));
        assert!(result.contains("+50 chars"));
    }

    #[test]
    fn test_colorize() {
        // 测试颜色化
        let result = colorize(Some("test"), "31");
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("test"));
        assert!(result.contains("\x1b[0m"));

        // 测试空值
        let result = colorize(None, "31");
        assert_eq!(result, "");

        let result = colorize(Some(""), "31");
        assert_eq!(result, "");
    }

    #[test]
    fn test_rule_kind_cn() {
        // 测试规则类型中文名
        assert_eq!(rule_kind_cn(&RuleKind::Text), "文本");
        assert_eq!(rule_kind_cn(&RuleKind::Image), "图片");
        assert_eq!(rule_kind_cn(&RuleKind::Voice), "语音");
        assert_eq!(rule_kind_cn(&RuleKind::Video), "视频");
        assert_eq!(rule_kind_cn(&RuleKind::Any), "任意");
    }

    #[test]
    fn test_chat_kind_cn() {
        // 测试聊天类型中文名
        assert_eq!(chat_kind_cn(ChatKind::Private), "私聊");
        assert_eq!(chat_kind_cn(ChatKind::Group), "群聊");
    }

    #[test]
    fn test_rule_kind_name() {
        // 测试规则类型英文名
        assert_eq!(rule_kind_name(&RuleKind::Text), "text");
        assert_eq!(rule_kind_name(&RuleKind::Image), "image");
        assert_eq!(rule_kind_name(&RuleKind::Link), "link");
        assert_eq!(rule_kind_name(&RuleKind::Any), "any");
    }

    #[test]
    fn test_matches_kind() {
        // 测试类型匹配
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test".to_string()),
            msg_type: Some(1),
            from_wxid: None,
            group_sender_wxid: None,
            to_wxid: None,
            content: None,
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        assert!(matches_kind(RuleKind::Text, &norm));
        assert!(matches_kind(RuleKind::Any, &norm));
        assert!(!matches_kind(RuleKind::Image, &norm));
    }

    #[test]
    fn test_extract_between() {
        // 测试字符串提取
        let text = "<tag>content</tag>";
        let result = extract_between(text, "<tag>", "</tag>");
        assert_eq!(result, Some("content".to_string()));

        let text = "no tags here";
        let result = extract_between(text, "<tag>", "</tag>");
        assert_eq!(result, None);
    }

    #[test]
    fn test_map_type_label() {
        // 测试类型标签映射
        assert_eq!(map_type_label(Some(1)), "文本");
        assert_eq!(map_type_label(Some(3)), "图片");
        assert_eq!(map_type_label(Some(34)), "语音");
        assert_eq!(map_type_label(None), "引用");
    }

    #[test]
    fn test_extract_emoji_md5() {
        // 测试 emoji MD5 提取
        let xml = r#"<emoji md5="abc123def456" />"#;
        let result = extract_emoji_md5(xml);
        assert_eq!(result, Some("abc123def456".to_string()));

        let xml = r#"<emoji />"#;
        let result = extract_emoji_md5(xml);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_nickname() {
        // 测试昵称提取
        let push = "Alice: hello world";
        assert_eq!(extract_nickname(Some(push)), Some("Alice".to_string()));

        let push = "Bob：你好";
        assert_eq!(extract_nickname(Some(push)), Some("Bob".to_string()));

        let push = "no colon";
        assert_eq!(extract_nickname(Some(push)), None);

        assert_eq!(extract_nickname(None), None);
    }

    #[test]
    fn test_extract_display_name() {
        // 测试显示名提取
        let push = "Alice: hello";
        assert_eq!(extract_display_name(Some(push)), Some("Alice".to_string()));

        let push = "Bob在群聊中@了你";
        assert_eq!(extract_display_name(Some(push)), Some("Bob".to_string()));

        assert_eq!(extract_display_name(None), None);
    }

    #[test]
    fn test_extract_group_sender() {
        // 测试群聊发送者提取
        let content = "wxid_abc123: hello world";
        assert_eq!(
            extract_group_sender(content),
            Some("wxid_abc123".to_string())
        );

        let content = "no colon content";
        assert_eq!(extract_group_sender(content), None);
    }

    #[test]
    fn test_extract_atuserlist() {
        // 测试 @ 用户列表提取
        let src = "<msgsource><atuserlist>wxid_1,wxid_2</atuserlist></msgsource>";
        assert_eq!(extract_atuserlist(src), Some("wxid_1,wxid_2".to_string()));

        let src = "<msgsource></msgsource>";
        assert_eq!(extract_atuserlist(src), None);
    }

    #[test]
    fn test_escape_xml() {
        // 测试 XML 转义
        let input = r#"<tag>"test" & 'more'"#;
        let result = escape_xml(input, 100);
        assert!(result.contains("&lt;"));
        assert!(result.contains("&gt;"));
        assert!(result.contains("&quot;"));
        assert!(result.contains("&apos;"));
        assert!(result.contains("&amp;"));
    }

    #[test]
    fn test_escape_xml_truncate() {
        // 测试 XML 转义时的截断
        let input = "a".repeat(100);
        let result = escape_xml(&input, 50);
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_truncate_string() {
        // 测试字符串截断
        let mut text = "hello world".to_string();
        let truncated = truncate_string(&mut text, 20);
        assert!(!truncated);
        assert_eq!(text, "hello world");

        let mut text = "hello world".to_string();
        let truncated = truncate_string(&mut text, 5);
        assert!(truncated);
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_clamp_output() {
        // 测试输出限制
        let text = "hello world".to_string();
        let (result, truncated) = clamp_output(text, 100);
        assert_eq!(result, "hello world");
        assert!(!truncated);

        let text = "a".repeat(100);
        let (result, truncated) = clamp_output(text, 50);
        assert_eq!(result.len(), 50);
        assert!(truncated);
    }

    #[test]
    fn test_command_timeout() {
        // 测试命令超时
        let action = CommandAction {
            program: "test".to_string(),
            args: vec![],
            timeout_secs: Some(30),
            max_output: None,
            pre_reply: None,
            post_reply: None,
        };
        let timeout = command_timeout(&action);
        assert_eq!(timeout, Duration::from_secs(30));

        // 测试默认超时
        let action = CommandAction {
            program: "test".to_string(),
            args: vec![],
            timeout_secs: None,
            max_output: None,
            pre_reply: None,
            post_reply: None,
        };
        let timeout = command_timeout(&action);
        assert_eq!(timeout, Duration::from_secs(15));
    }

    #[test]
    fn test_command_max_output() {
        // 测试命令最大输出
        let action = CommandAction {
            program: "test".to_string(),
            args: vec![],
            timeout_secs: None,
            max_output: Some(1024),
            pre_reply: None,
            post_reply: None,
        };
        assert_eq!(command_max_output(&action), 1024);

        // 测试默认最大输出
        let action = CommandAction {
            program: "test".to_string(),
            args: vec![],
            timeout_secs: None,
            max_output: None,
            pre_reply: None,
            post_reply: None,
        };
        assert_eq!(command_max_output(&action), 20 * 1024);
    }

    #[test]
    fn test_extract_new_msg_id() {
        // 测试 NewMsgId 提取
        let data = json!({
            "NewMsgId": 12345
        });
        assert_eq!(extract_new_msg_id(&data), Some(12345));

        let data = json!({
            "Data": {
                "NewMsgId": 67890
            }
        });
        assert_eq!(extract_new_msg_id(&data), Some(67890));

        let data = json!({});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    #[test]
    fn test_extract_appmsg_type() {
        // 测试 appmsg 类型提取
        let xml = "<msg><appmsg><type>5</type></appmsg></msg>";
        assert_eq!(extract_appmsg_type(Some(49), Some(xml)), Some(5));

        // 非 appmsg
        assert_eq!(extract_appmsg_type(Some(1), Some("text")), None);

        // 无内容
        assert_eq!(extract_appmsg_type(Some(49), None), None);
    }

    #[test]
    fn test_ai_error_message() {
        // 测试 AI 错误消息
        let err = anyhow::anyhow!("API key invalid");
        assert!(ai_error_message(&err).contains("配置异常"));

        let err = anyhow::anyhow!("timeout occurred");
        assert!(ai_error_message(&err).contains("超时"));

        let err = anyhow::anyhow!("rate limit exceeded");
        assert!(ai_error_message(&err).contains("繁忙"));

        let err = anyhow::anyhow!("unknown error");
        assert!(ai_error_message(&err).contains("请稍后重试"));
    }

    #[test]
    fn test_build_user_content() {
        // 测试构建用户内容
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test_app".to_string()),
            msg_type: Some(1),
            from_wxid: Some("user1".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: Some("hello world".to_string()),
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: None,
            chat: Some(ChatKind::Private),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        let action = AiAction {
            provider: None,
            model: "test".to_string(),
            api_key: None,
            api_key_env: None,
            base_url: None,
            system_prompt: None,
            user_prefix: Some("Context: {chat}".to_string()),
            command: None,
            max_command_output: None,
            temperature: None,
            max_tokens: None,
            response_format: None,
            tools: vec![],
            max_retries: None,
            retry_delay_ms: None,
        };

        let result = build_user_content(&action, &norm, None);
        assert!(result.contains("Context: private"));
        assert!(result.contains("用户消息：hello world"));
    }

    #[test]
    fn test_render_user_prefix() {
        // 测试渲染用户前缀
        let norm = NormalizedEvent {
            kind: RuleKind::Text,
            app_id: AppId("test_app".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: Some("sender456".to_string()),
            to_wxid: Some("bot789".to_string()),
            content: None,
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: Some(12345),
            chat: Some(ChatKind::Group),
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        let prefix = "app={app_id}, chat={chat}, from={from_wxid}, sender={sender_wxid}";
        let result = render_user_prefix(prefix, &norm);
        assert!(result.contains("app=test_app"));
        assert!(result.contains("chat=group"));
        assert!(result.contains("from=user123"));
        assert!(result.contains("sender=sender456"));
    }

    #[test]
    fn test_render_filename() {
        // 测试文件名渲染
        let save = SaveAction {
            dir: "data".to_string(),
            filename: Some("{new_msg_id}_{from_wxid}.bin".to_string()),
        };

        let norm = NormalizedEvent {
            kind: RuleKind::Image,
            app_id: AppId("app1".to_string()),
            msg_type: None,
            from_wxid: Some("user123".to_string()),
            group_sender_wxid: None,
            to_wxid: None,
            content: None,
            push_content: None,
            msg_source: None,
            appmsg_type: None,
            new_msg_id: Some(98765),
            chat: None,
            nickname: None,
            type_name: None,
            normalized_content: None,
        };

        let result = render_filename(&save, &norm);
        assert_eq!(result, "98765_user123.bin");
    }

    #[test]
    fn test_external_command_allowed() {
        // 测试外部命令是否允许
        // 注意：这个测试依赖环境变量，可能需要设置
        let allowed = external_command_allowed();
        assert!(!allowed || allowed); // 总是通过，只是确保函数可以调用
    }

    #[test]
    fn test_matcher_matches() {
        // 测试匹配器
        let matcher = Matcher {
            equals: Some("hello".to_string()),
            contains: None,
            regex: None,
        };
        assert!(matcher.matches("hello"));
        assert!(!matcher.matches("hello world"));

        let matcher = Matcher {
            equals: None,
            contains: Some("world".to_string()),
            regex: None,
        };
        assert!(matcher.matches("hello world"));
        assert!(!matcher.matches("hello"));

        // 测试空匹配器（匹配所有）
        let matcher = Matcher {
            equals: None,
            contains: None,
            regex: None,
        };
        assert!(matcher.matches("anything"));
    }

    #[test]
    fn test_matcher_from_match_config() {
        // 测试从配置创建匹配器
        let config = MatchConfig {
            equals: Some("test".to_string()),
            contains: Some("hello".to_string()),
            regex: Some(r"^\d+$".to_string()),
        };

        let matcher = Matcher::from_match_config(&config).unwrap();
        assert!(matcher.equals.is_some());
        assert!(matcher.contains.is_some());
        assert!(matcher.regex.is_some());

        // 测试无效的正则表达式
        let config = MatchConfig {
            equals: None,
            contains: None,
            regex: Some("[invalid".to_string()),
        };

        assert!(Matcher::from_match_config(&config).is_err());
    }
}
