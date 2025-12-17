use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

fn default_listen_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_queue_size() -> usize {
    2048
}

fn default_image_dir() -> String {
    "data/images".to_string()
}

fn default_image_url_prefix() -> String {
    "/images".to_string()
}

fn default_max_concurrency() -> usize {
    8
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotConfig {
    pub app_id: String,
    pub token: String,
    pub base_url: String,
    #[serde(default)]
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub listen_addr: String,
    pub queue_size: usize,
    /// 图片存储目录
    pub image_dir: String,
    /// 图片 URL 前缀（用于构建可访问的图片 URL）
    pub image_url_prefix: String,
    /// 外部访问基础 URL（如 https://your-domain.com），用于构建完整图片 URL
    pub external_base_url: Option<String>,
    /// 全局最大并发（处理 webhook 事件），默认 8
    #[serde(default = "default_max_concurrency")]
    pub max_concurrency: usize,
    pub bots: Vec<BotConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RuleKind {
    Text,
    Image,
    Voice,
    Video,
    Emoji,
    Link,
    FileNotice,
    ContactEvent,
    #[default]
    Any,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MatchConfig {
    #[serde(default)]
    pub equals: Option<String>,
    #[serde(default)]
    pub contains: Option<String>,
    #[serde(default)]
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FromConfig {
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub wxid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatKind {
    Private,
    Group,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SaveAction {
    /// 保存目录
    pub dir: String,
    /// 文件名模板，可选。支持 {new_msg_id}/{app_id}/{from_wxid} 替换。
    #[serde(default)]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CommandAction {
    /// 程序名或可执行路径。内置命令使用预置名称（如 claude_changelog）。
    pub program: String,
    /// 传给外置命令的参数，可选。
    #[serde(default)]
    pub args: Vec<String>,
    /// 超时秒数，可选，默认内部设置。
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    /// 最大输出长度（字节），超出会截断。
    #[serde(default)]
    pub max_output: Option<usize>,
    /// 在执行命令前先回复的一段话（可选）。
    #[serde(default)]
    pub pre_reply: Option<String>,
    /// 命令执行完成后（成功）再回复的一段话（可选）。
    #[serde(default)]
    pub post_reply: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RuleAction {
    #[serde(default)]
    pub reply_text: Option<String>,
    #[serde(default)]
    pub save: Option<SaveAction>,
    #[serde(default)]
    pub forward: Option<Vec<String>>,
    #[serde(default)]
    pub log: Option<bool>,
    #[serde(default)]
    pub ignore: Option<bool>,
    #[serde(default)]
    pub command: Option<CommandAction>,
    /// AI 回复动作，使用 OpenAI 兼容接口（可指向 Gemini OpenAI 兼容端点）。
    #[serde(default)]
    pub ai: Option<AiAction>,
    /// 回复模式：none（默认，直接发消息）、quote（引用原消息）、at（在群聊中@发送者）。
    #[serde(default)]
    pub reply_mode: Option<ReplyMode>,
    /// 是否要求在群聊中被 @ 才触发该规则（仅群聊生效）。
    #[serde(default)]
    pub require_mention: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    #[serde(default)]
    pub kind: RuleKind,
    #[serde(default)]
    pub r#match: MatchConfig,
    #[serde(default)]
    pub from: FromConfig,
    #[serde(default)]
    pub chat: Option<ChatKind>,
    #[serde(default)]
    pub action: RuleAction,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ResponseFormatConfig {
    /// 目前支持 json_object。
    #[serde(rename = "type", default)]
    pub format_type: Option<String>,
    /// 可选 JSON Schema。
    #[serde(default)]
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AiAction {
    /// LLM Provider: openai, anthropic, gemini。默认 openai（支持 OpenAI 兼容接口）。
    #[serde(default)]
    pub provider: Option<String>,
    /// 模型名称，如 gpt-4o / claude-3-5-sonnet / gemini-2.0-flash 等。
    pub model: String,
    /// API Key，直接配置（优先级高于 api_key_env）。
    /// 注意：直接写在配置文件中有安全风险，建议使用 api_key_env。
    #[serde(default)]
    pub api_key: Option<String>,
    /// API Key 环境变量名，未配置则读取 GEWE_AI_API_KEY。
    #[serde(default)]
    pub api_key_env: Option<String>,
    /// OpenAI 兼容 base_url。默认为 https://api.openai.com/v1；
    /// 若调用 Gemini OpenAI 兼容层，设置为 https://generativelanguage.googleapis.com/v1beta/openai/。
    #[serde(default)]
    pub base_url: Option<String>,
    /// system prompt。
    #[serde(default)]
    pub system_prompt: Option<String>,
    /// user 内容前置提示词。
    #[serde(default)]
    pub user_prefix: Option<String>,
    /// 在调用模型前执行的命令（例如查询接口），输出会拼到提示词中。
    #[serde(default)]
    pub command: Option<CommandAction>,
    /// command 输出截断长度，未设置时沿用默认 command 最大输出。
    #[serde(default)]
    pub max_command_output: Option<usize>,
    /// 模型参数
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    /// 结构化输出配置
    #[serde(default)]
    pub response_format: Option<ResponseFormatConfig>,
    /// OpenAI 工具配置，模型可自主选择调用，内部映射到命令执行。
    #[serde(default)]
    pub tools: Vec<AiTool>,
    /// 最大重试次数，默认 2（即最多请求 3 次）。设为 0 禁用重试。
    #[serde(default)]
    pub max_retries: Option<u32>,
    /// 重试基础延迟（毫秒），默认 1000。采用指数退避：第 N 次重试等待 base_delay * 2^N。
    #[serde(default)]
    pub retry_delay_ms: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReplyMode {
    #[default]
    None,
    Quote,
    At,
    QuoteAndAt,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AiTool {
    /// 工具名称，需与模型 tool call 的 function.name 一致。
    pub name: String,
    /// 工具描述，帮助模型按需选择。
    #[serde(default)]
    pub description: Option<String>,
    /// OpenAI 风格的 parameters（JSON Schema），可选。
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
    /// 绑定的命令（内置或外置）。
    #[serde(default)]
    pub command: Option<CommandAction>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            listen_addr: default_listen_addr(),
            queue_size: default_queue_size(),
            image_dir: default_image_dir(),
            image_url_prefix: default_image_url_prefix(),
            external_base_url: None,
            max_concurrency: default_max_concurrency(),
            bots: Vec::new(),
        }
    }
}

impl AppConfig {
    pub fn load(path: Option<&str>) -> Result<Self> {
        let path = path
            .map(PathBuf::from)
            .or_else(|| std::env::var("GEWE_BOT_CONFIG").ok().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("config/bot-app.toml"));
        let body = std::fs::read_to_string(&path)
            .with_context(|| format!("读取配置失败: {}", path.display()))?;

        // 检测是否为 V2 配置（config_version = 2）
        if is_v2_config(&body) {
            let v2: AppConfigV2 = toml::from_str(&body)
                .with_context(|| format!("解析 V2 配置失败: {}", path.display()))?;
            let config = v2
                .into_v1(&path)
                .with_context(|| format!("转换 V2 配置失败: {}", path.display()))?;
            return Ok(config);
        }

        let mut config: AppConfig =
            toml::from_str(&body).with_context(|| format!("解析配置失败: {}", path.display()))?;
        if config.listen_addr.is_empty() {
            config.listen_addr = default_listen_addr();
        }
        if config.queue_size == 0 {
            config.queue_size = default_queue_size();
        }
        if config.max_concurrency == 0 {
            config.max_concurrency = default_max_concurrency();
        }
        Ok(config)
    }
}

/// 判定配置是否为 V2 结构
fn is_v2_config(body: &str) -> bool {
    toml::from_str::<toml::Value>(body)
        .ok()
        .and_then(|v| v.get("config_version").and_then(|x| x.as_integer()))
        == Some(2)
}

// -------------------- V2 配置定义与转换 --------------------

/// V2 配置根结构（公开用于 API）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigV2 {
    pub config_version: u8,
    #[serde(default)]
    pub server: ServerConfigV2,
    #[serde(default)]
    pub storage: StorageConfigV2,
    #[serde(default)]
    pub defaults: DefaultsV2,
    #[serde(default)]
    pub bots: Vec<BotConfigV2>,
    #[serde(default)]
    pub ai_profiles: Vec<AiProfileV2>,
    #[serde(default)]
    pub tools: Vec<ToolConfigV2>,
    #[serde(default)]
    pub rule_templates: Vec<RuleTemplateV2>,
    #[serde(default)]
    pub rule_instances: Vec<RuleInstanceV2>,
}

/// 服务器配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerConfigV2 {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    #[serde(default = "default_queue_size")]
    pub queue_size: usize,
}

/// 存储配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageConfigV2 {
    #[serde(default = "default_image_dir")]
    pub image_dir: String,
    #[serde(default = "default_image_url_prefix")]
    pub image_url_prefix: String,
    #[serde(default)]
    pub external_base_url: Option<String>,
}

/// 默认配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultsV2 {
    #[serde(default)]
    pub reply_mode: Option<ReplyMode>,
    #[serde(default)]
    pub log: Option<bool>,
    #[serde(default)]
    pub ai: Option<DefaultsAiV2>,
}

/// AI 默认配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultsAiV2 {
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub require_mention: Option<bool>,
}

/// Bot 配置（V2）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotConfigV2 {
    #[serde(default)]
    pub id: Option<String>,
    pub app_id: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub token_env: Option<String>,
    pub base_url: String,
    #[serde(default)]
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub webhook_secret_env: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// AI Profile 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiProfileV2 {
    pub id: String,
    #[serde(default)]
    pub provider: Option<String>,
    pub model: String,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub api_key_env: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub system_prompt_file: Option<String>,
    #[serde(default)]
    pub user_prefix: Option<String>,
    #[serde(default)]
    pub tool_ids: Vec<String>,
}

/// 工具配置（V2）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolConfigV2 {
    pub id: String,
    #[serde(default)]
    pub kind: Option<String>,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub max_output: Option<usize>,
    #[serde(default)]
    pub pre_reply: Option<String>,
    #[serde(default)]
    pub post_reply: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// 可选的 parameters（JSON Schema），未配置时会补全 {"type":"object"}
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
}

/// 规则模板（V2）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleTemplateV2 {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub kind: Option<RuleKind>,
    #[serde(default)]
    pub r#match: MatchConfigV2,
    #[serde(default)]
    pub action: TemplateActionV2,
    #[serde(default)]
    pub defaults: TemplateDefaultsV2,
}

/// 模板默认配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateDefaultsV2 {
    #[serde(default)]
    pub require_mention: Option<bool>,
}

/// 匹配配置（V2）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchConfigV2 {
    #[serde(default)]
    pub equals: Option<String>,
    #[serde(default)]
    pub contains: Option<String>,
    #[serde(default)]
    pub regex: Option<String>,
    #[serde(default)]
    pub any: Option<bool>,
}

/// 模板动作配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateActionV2 {
    #[serde(default)]
    pub ai_profile: Option<String>,
    #[serde(default)]
    pub reply_mode: Option<ReplyMode>,
    #[serde(default)]
    pub log: Option<bool>,
    #[serde(default)]
    pub require_mention: Option<bool>,
    #[serde(default)]
    pub reply_text: Option<String>,
}

/// 实例覆盖配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceOverridesV2 {
    #[serde(default)]
    pub require_mention: Option<bool>,
    #[serde(default)]
    pub reply_mode: Option<ReplyMode>,
    #[serde(default)]
    pub ai_profile: Option<String>,
    #[serde(default)]
    pub log: Option<bool>,
    #[serde(default)]
    pub reply_text: Option<String>,
}

/// 规则实例（V2）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleInstanceV2 {
    pub id: String,
    pub template: String,
    #[serde(default)]
    pub channel: Option<String>, // private/group/both
    #[serde(default)]
    pub from: FromConfig,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub overrides: Option<InstanceOverridesV2>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

impl AppConfigV2 {
    /// 从文件加载 V2 配置
    #[allow(dead_code)]
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("读取配置失败: {}", path.display()))?;
        Self::parse(&body)
    }

    /// 从 TOML 字符串解析
    pub fn parse(body: &str) -> Result<Self> {
        toml::from_str(body).with_context(|| "解析 V2 配置失败")
    }

    /// 从 JSON 字符串解析
    #[allow(dead_code)]
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).with_context(|| "解析 JSON 配置失败")
    }

    /// 序列化为 TOML 字符串
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).with_context(|| "序列化 TOML 失败")
    }

    /// 序列化为 JSON 字符串
    #[allow(dead_code)]
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).with_context(|| "序列化 JSON 失败")
    }

    /// 校验配置，返回错误列表
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // 检查 config_version
        if self.config_version != 2 {
            errors.push(format!(
                "config_version 应为 2，当前为 {}",
                self.config_version
            ));
        }

        // 检查 bots
        let mut bot_ids = std::collections::HashSet::new();
        for (i, bot) in self.bots.iter().enumerate() {
            if bot.app_id.trim().is_empty() {
                errors.push(format!("bots[{}]: app_id 不能为空", i));
            }
            if bot.base_url.trim().is_empty() {
                errors.push(format!("bots[{}]: base_url 不能为空", i));
            }
            if bot.token.is_none() && bot.token_env.is_none() {
                errors.push(format!("bots[{}]: token 或 token_env 必须设置其一", i));
            }
            let id = bot.id.clone().unwrap_or_else(|| bot.app_id.clone());
            if !bot_ids.insert(id.clone()) {
                errors.push(format!("bots[{}]: 重复的 id/app_id: {}", i, id));
            }
        }

        // 检查 ai_profiles
        let mut profile_ids = std::collections::HashSet::new();
        for (i, profile) in self.ai_profiles.iter().enumerate() {
            if profile.id.trim().is_empty() {
                errors.push(format!("ai_profiles[{}]: id 不能为空", i));
            }
            if profile.model.trim().is_empty() {
                errors.push(format!("ai_profiles[{}]: model 不能为空", i));
            }
            if !profile_ids.insert(profile.id.clone()) {
                errors.push(format!("ai_profiles[{}]: 重复的 id: {}", i, profile.id));
            }
            // 检查引用的 tool_ids 是否存在
            for tool_id in &profile.tool_ids {
                if !self.tools.iter().any(|t| &t.id == tool_id) {
                    errors.push(format!("ai_profiles[{}]: 引用的工具不存在: {}", i, tool_id));
                }
            }
        }

        // 检查 tools
        let mut tool_ids = std::collections::HashSet::new();
        for (i, tool) in self.tools.iter().enumerate() {
            if tool.id.trim().is_empty() {
                errors.push(format!("tools[{}]: id 不能为空", i));
            }
            if tool.program.trim().is_empty() {
                errors.push(format!("tools[{}]: program 不能为空", i));
            }
            if !tool_ids.insert(tool.id.clone()) {
                errors.push(format!("tools[{}]: 重复的 id: {}", i, tool.id));
            }
        }

        // 检查 rule_templates
        let mut template_ids = std::collections::HashSet::new();
        for (i, template) in self.rule_templates.iter().enumerate() {
            if template.id.trim().is_empty() {
                errors.push(format!("rule_templates[{}]: id 不能为空", i));
            }
            if !template_ids.insert(template.id.clone()) {
                errors.push(format!("rule_templates[{}]: 重复的 id: {}", i, template.id));
            }
            // 检查引用的 ai_profile 是否存在
            if let Some(ref profile_id) = template.action.ai_profile {
                if !profile_ids.contains(profile_id) {
                    errors.push(format!(
                        "rule_templates[{}]: 引用的 ai_profile 不存在: {}",
                        i, profile_id
                    ));
                }
            }
        }

        // 检查 rule_instances
        let mut instance_ids = std::collections::HashSet::new();
        for (i, instance) in self.rule_instances.iter().enumerate() {
            if instance.id.trim().is_empty() {
                errors.push(format!("rule_instances[{}]: id 不能为空", i));
            }
            if instance.template.trim().is_empty() {
                errors.push(format!("rule_instances[{}]: template 不能为空", i));
            }
            if !template_ids.contains(&instance.template) {
                errors.push(format!(
                    "rule_instances[{}]: 引用的模板不存在: {}",
                    i, instance.template
                ));
            }
            if !instance_ids.insert(instance.id.clone()) {
                errors.push(format!("rule_instances[{}]: 重复的 id: {}", i, instance.id));
            }
            // 检查 channel 值
            if let Some(ref channel) = instance.channel {
                if !["private", "group", "both"].contains(&channel.as_str()) {
                    errors.push(format!(
                        "rule_instances[{}]: channel 必须是 private/group/both，当前为: {}",
                        i, channel
                    ));
                }
            }
            // 检查 overrides 中引用的 ai_profile
            if let Some(ref overrides) = instance.overrides {
                if let Some(ref profile_id) = overrides.ai_profile {
                    if !profile_ids.contains(profile_id) {
                        errors.push(format!(
                            "rule_instances[{}].overrides: 引用的 ai_profile 不存在: {}",
                            i, profile_id
                        ));
                    }
                }
            }
        }

        errors
    }

    /// 将 V2 配置转换为 V1 运行时配置
    fn into_v1(self, base_path: &std::path::Path) -> Result<AppConfig> {
        // 构建映射
        let ai_map: HashMap<String, AiProfileV2> = self
            .ai_profiles
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect();
        let tool_map: HashMap<String, ToolConfigV2> =
            self.tools.into_iter().map(|t| (t.id.clone(), t)).collect();
        let tmpl_map: HashMap<String, RuleTemplateV2> = self
            .rule_templates
            .into_iter()
            .map(|t| (t.id.clone(), t))
            .collect();

        let mut bots = Vec::new();
        for bot in self.bots {
            let token = pick_credential("token", &bot.token, &bot.token_env)?;
            let webhook_secret = pick_optional_credential(
                "webhook_secret",
                &bot.webhook_secret,
                &bot.webhook_secret_env,
            )?;

            // 依据实例生成规则
            let mut instances = self.rule_instances.clone();
            // 优先级排序，小值在前
            instances.sort_by_key(|i| i.priority.unwrap_or(0));

            let mut rules = Vec::new();
            for inst in instances {
                if matches!(inst.enabled, Some(false)) {
                    continue;
                }
                let tmpl = tmpl_map
                    .get(&inst.template)
                    .ok_or_else(|| anyhow::anyhow!("未找到规则模板: {}", inst.template))?;

                let chat = match inst.channel.as_deref() {
                    Some("group") => Some(ChatKind::Group),
                    Some("private") => Some(ChatKind::Private),
                    Some("both") | None => None,
                    Some(other) => {
                        return Err(anyhow::anyhow!("不支持的 channel: {}", other));
                    }
                };

                let mut action = RuleAction::default();

                // reply_mode 组合：实例覆盖 > 模板 > 全局 defaults
                action.reply_mode = inst
                    .overrides
                    .as_ref()
                    .and_then(|o| o.reply_mode.clone())
                    .or_else(|| tmpl.action.reply_mode.clone())
                    .or_else(|| self.defaults.reply_mode.clone());

                // log 组合
                action.log = inst
                    .overrides
                    .as_ref()
                    .and_then(|o| o.log)
                    .or(tmpl.action.log)
                    .or(self.defaults.log);

                // require_mention 组合：实例覆盖 > 模板 action > 模板 defaults > 全局 defaults.ai
                action.require_mention = inst
                    .overrides
                    .as_ref()
                    .and_then(|o| o.require_mention)
                    .or(tmpl.action.require_mention)
                    .or(tmpl.defaults.require_mention)
                    .or_else(|| self.defaults.ai.as_ref().and_then(|a| a.require_mention));

                // reply_text 组合
                action.reply_text = inst
                    .overrides
                    .as_ref()
                    .and_then(|o| o.reply_text.clone())
                    .or_else(|| tmpl.action.reply_text.clone());

                // AI 配置：实例覆盖 > 模板 action > 全局 defaults.ai.profile
                if let Some(profile_id) = inst
                    .overrides
                    .as_ref()
                    .and_then(|o| o.ai_profile.clone())
                    .or_else(|| tmpl.action.ai_profile.clone())
                {
                    let ai_profile = ai_map
                        .get(&profile_id)
                        .ok_or_else(|| anyhow::anyhow!("未找到 AI Profile: {}", profile_id))?;
                    action.ai = Some(build_ai_action(ai_profile, &tool_map, base_path)?);
                }

                let rule = RuleConfig {
                    kind: tmpl.kind.clone().unwrap_or_default(),
                    r#match: tmpl.r#match.to_v1(),
                    from: inst.from.clone(),
                    chat,
                    action,
                };
                rules.push(rule);
            }

            let bot_cfg = BotConfig {
                app_id: bot.app_id,
                token,
                base_url: bot.base_url,
                webhook_secret,
                rules,
            };
            bots.push(bot_cfg);
        }

        Ok(AppConfig {
            listen_addr: self.server.listen_addr,
            queue_size: self.server.queue_size,
            image_dir: self.storage.image_dir,
            image_url_prefix: self.storage.image_url_prefix,
            external_base_url: self.storage.external_base_url,
            max_concurrency: default_max_concurrency(),
            bots,
        })
    }
}

impl MatchConfigV2 {
    fn to_v1(&self) -> MatchConfig {
        MatchConfig {
            equals: self.equals.clone(),
            contains: self.contains.clone(),
            regex: self.regex.clone(),
        }
    }
}

fn pick_credential(name: &str, value: &Option<String>, env_key: &Option<String>) -> Result<String> {
    if let Some(v) = value.clone() {
        if !v.trim().is_empty() {
            return Ok(v);
        }
    }
    if let Some(env) = env_key.as_deref() {
        if let Ok(v) = std::env::var(env) {
            if !v.trim().is_empty() {
                return Ok(v);
            }
        }
        return Err(anyhow::anyhow!("未找到 {}，请设置环境变量 {}", name, env));
    }
    Err(anyhow::anyhow!("未找到 {}，请在配置或环境变量中设置", name))
}

fn pick_optional_credential(
    _name: &str,
    value: &Option<String>,
    env_key: &Option<String>,
) -> Result<Option<String>> {
    if let Some(v) = value.clone() {
        if !v.trim().is_empty() {
            return Ok(Some(v));
        }
    }
    if let Some(env) = env_key.as_deref() {
        if let Ok(v) = std::env::var(env) {
            if !v.trim().is_empty() {
                return Ok(Some(v));
            }
        }
    }
    Ok(None)
}

fn build_ai_action(
    profile: &AiProfileV2,
    tool_map: &HashMap<String, ToolConfigV2>,
    base_path: &std::path::Path,
) -> Result<AiAction> {
    let mut system_prompt = profile.system_prompt.clone();
    if system_prompt.is_none() {
        if let Some(ref file) = profile.system_prompt_file {
            let abs = if PathBuf::from(file).is_absolute() {
                PathBuf::from(file)
            } else {
                base_path.parent().unwrap_or(Path::new(".")).join(file)
            };
            system_prompt = Some(
                std::fs::read_to_string(&abs)
                    .with_context(|| format!("读取 system_prompt_file 失败: {}", abs.display()))?,
            );
        }
    }

    let mut tools = Vec::new();
    for tool_id in &profile.tool_ids {
        let tool = tool_map
            .get(tool_id)
            .ok_or_else(|| anyhow::anyhow!("AI Profile 引用的工具不存在: {}", tool_id))?;
        let cmd = CommandAction {
            program: tool.program.clone(),
            args: tool.args.clone(),
            timeout_secs: tool.timeout_secs,
            max_output: tool.max_output,
            pre_reply: tool.pre_reply.clone(),
            post_reply: tool.post_reply.clone(),
        };
        tools.push(AiTool {
            name: tool.id.clone(),
            description: tool.description.clone(),
            parameters: Some(
                tool.parameters
                    .clone()
                    .unwrap_or_else(|| json!({"type": "object"})),
            ),
            command: Some(cmd),
        });
    }

    Ok(AiAction {
        provider: profile.provider.clone(),
        model: profile.model.clone(),
        api_key: profile.api_key.clone(),
        api_key_env: profile.api_key_env.clone(),
        base_url: profile.base_url.clone(),
        system_prompt,
        user_prefix: profile.user_prefix.clone(),
        command: None,
        max_command_output: None,
        temperature: None,
        max_tokens: None,
        response_format: None,
        tools,
        max_retries: None,
        retry_delay_ms: None,
    })
}
