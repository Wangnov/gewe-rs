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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_kind_serde() {
        // 测试 RuleKind 序列化和反序列化
        let kind = RuleKind::Text;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, r#""text""#);

        let deserialized: RuleKind = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, RuleKind::Text);

        // 测试默认值
        let default_kind = RuleKind::default();
        assert_eq!(default_kind, RuleKind::Any);
    }

    #[test]
    fn test_reply_mode_serde() {
        // 测试 ReplyMode 序列化和反序列化
        let mode = ReplyMode::Quote;
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, r#""quote""#);

        let deserialized: ReplyMode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ReplyMode::Quote);

        // 测试默认值
        let default_mode = ReplyMode::default();
        assert_eq!(default_mode, ReplyMode::None);
    }

    #[test]
    fn test_chat_kind_serde() {
        // 测试 ChatKind 序列化和反序列化
        let kind = ChatKind::Group;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, r#""group""#);

        let deserialized: ChatKind = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ChatKind::Group);
    }

    #[test]
    fn test_from_config_serde() {
        // 测试 FromConfig 序列化和反序列化
        let config = FromConfig {
            nick: Some("test_nick".to_string()),
            wxid: Some("test_wxid".to_string()),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: FromConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.nick, Some("test_nick".to_string()));
        assert_eq!(deserialized.wxid, Some("test_wxid".to_string()));
    }

    #[test]
    fn test_match_config_deserialize() {
        // 测试 MatchConfig 反序列化
        let toml_str = r#"
            equals = "hello"
            contains = "world"
            regex = "^test.*"
        "#;

        let config: MatchConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.equals, Some("hello".to_string()));
        assert_eq!(config.contains, Some("world".to_string()));
        assert_eq!(config.regex, Some("^test.*".to_string()));
    }

    #[test]
    fn test_command_action_deserialize() {
        // 测试 CommandAction 反序列化
        let toml_str = r#"
            program = "echo"
            args = ["hello", "world"]
            timeout_secs = 30
            max_output = 1024
            pre_reply = "Processing..."
            post_reply = "Done!"
        "#;

        let action: CommandAction = toml::from_str(toml_str).unwrap();
        assert_eq!(action.program, "echo");
        assert_eq!(action.args, vec!["hello", "world"]);
        assert_eq!(action.timeout_secs, Some(30));
        assert_eq!(action.max_output, Some(1024));
        assert_eq!(action.pre_reply, Some("Processing...".to_string()));
        assert_eq!(action.post_reply, Some("Done!".to_string()));
    }

    #[test]
    fn test_app_config_default() {
        // 测试 AppConfig 默认值
        let config = AppConfig::default();
        assert_eq!(config.listen_addr, "0.0.0.0:3000");
        assert_eq!(config.queue_size, 2048);
        assert_eq!(config.image_dir, "data/images");
        assert_eq!(config.image_url_prefix, "/images");
        assert_eq!(config.max_concurrency, 8);
        assert!(config.bots.is_empty());
    }

    #[test]
    fn test_is_v2_config() {
        // 测试 V2 配置检测
        let v2_toml = r#"
            config_version = 2

            [server]
            listen_addr = "0.0.0.0:3000"
        "#;

        assert!(is_v2_config(v2_toml));

        let v1_toml = r#"
            listen_addr = "0.0.0.0:3000"

            [[bots]]
            app_id = "test"
        "#;

        assert!(!is_v2_config(v1_toml));
    }

    #[test]
    fn test_app_config_v2_parse() {
        // 测试 V2 配置解析
        let toml_str = r#"
            config_version = 2

            [server]
            listen_addr = "0.0.0.0:3000"
            queue_size = 1024

            [storage]
            image_dir = "data/images"
            image_url_prefix = "/images"
        "#;

        let config = AppConfigV2::parse(toml_str).unwrap();
        assert_eq!(config.config_version, 2);
        assert_eq!(config.server.listen_addr, "0.0.0.0:3000");
        assert_eq!(config.server.queue_size, 1024);
        assert_eq!(config.storage.image_dir, "data/images");
    }

    #[test]
    fn test_app_config_v2_to_toml() {
        // 测试 V2 配置序列化为 TOML
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2 {
                listen_addr: "0.0.0.0:3000".to_string(),
                queue_size: 2048,
            },
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let toml = config.to_toml().unwrap();
        assert!(toml.contains("config_version = 2"));
        assert!(toml.contains("listen_addr"));
    }

    #[test]
    fn test_app_config_v2_validate_empty() {
        // 测试空配置验证
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_app_config_v2_validate_wrong_version() {
        // 测试错误的版本号
        let config = AppConfigV2 {
            config_version: 1,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("config_version"));
    }

    #[test]
    fn test_app_config_v2_validate_duplicate_bot_ids() {
        // 测试重复的 bot id
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![
                BotConfigV2 {
                    id: Some("bot1".to_string()),
                    app_id: "app1".to_string(),
                    token: Some("token1".to_string()),
                    base_url: "http://localhost".to_string(),
                    ..Default::default()
                },
                BotConfigV2 {
                    id: Some("bot1".to_string()),
                    app_id: "app2".to_string(),
                    token: Some("token2".to_string()),
                    base_url: "http://localhost".to_string(),
                    ..Default::default()
                },
            ],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("重复的 id")));
    }

    #[test]
    fn test_app_config_v2_validate_missing_token() {
        // 测试缺少 token
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![BotConfigV2 {
                app_id: "app1".to_string(),
                token: None,
                token_env: None,
                base_url: "http://localhost".to_string(),
                ..Default::default()
            }],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("token")));
    }

    #[test]
    fn test_pick_credential_direct() {
        // 测试直接配置凭证
        let result = pick_credential(
            "test",
            &Some("direct_value".to_string()),
            &Some("ENV_VAR".to_string()),
        )
        .unwrap();

        assert_eq!(result, "direct_value");
    }

    #[test]
    fn test_pick_credential_missing() {
        // 测试缺少凭证
        let result = pick_credential("test", &None, &None);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到"));
    }

    #[test]
    fn test_pick_optional_credential() {
        // 测试可选凭证
        let result = pick_optional_credential("test", &Some("value".to_string()), &None).unwrap();

        assert_eq!(result, Some("value".to_string()));

        let result = pick_optional_credential("test", &None, &None).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_match_config_v2_to_v1() {
        // 测试 MatchConfigV2 转换为 V1
        let v2 = MatchConfigV2 {
            equals: Some("test".to_string()),
            contains: Some("hello".to_string()),
            regex: Some("^world".to_string()),
            any: Some(true),
        };

        let v1 = v2.to_v1();
        assert_eq!(v1.equals, Some("test".to_string()));
        assert_eq!(v1.contains, Some("hello".to_string()));
        assert_eq!(v1.regex, Some("^world".to_string()));
    }

    #[test]
    fn test_default_functions() {
        // 测试默认值函数
        assert_eq!(default_listen_addr(), "0.0.0.0:3000");
        assert_eq!(default_queue_size(), 2048);
        assert_eq!(default_image_dir(), "data/images");
        assert_eq!(default_image_url_prefix(), "/images");
        assert_eq!(default_max_concurrency(), 8);
    }

    #[test]
    fn test_app_config_load_v1() {
        // 测试加载 V1 配置
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
listen_addr = "127.0.0.1:8080"
queue_size = 1024
image_dir = "test/images"
image_url_prefix = "/test-images"
external_base_url = "https://example.com"
max_concurrency = 4

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"
webhook_secret = "secret123"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let config = AppConfig::load(Some(tmpfile.path().to_str().unwrap())).unwrap();
        assert_eq!(config.listen_addr, "127.0.0.1:8080");
        assert_eq!(config.queue_size, 1024);
        assert_eq!(config.image_dir, "test/images");
        assert_eq!(config.image_url_prefix, "/test-images");
        assert_eq!(
            config.external_base_url,
            Some("https://example.com".to_string())
        );
        assert_eq!(config.max_concurrency, 4);
        assert_eq!(config.bots.len(), 1);
        assert_eq!(config.bots[0].app_id, "test_app");
        assert_eq!(config.bots[0].token, "test_token");
    }

    #[test]
    fn test_app_config_load_with_defaults() {
        // 测试加载配置时应用默认值
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
listen_addr = ""
queue_size = 0
max_concurrency = 0

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let config = AppConfig::load(Some(tmpfile.path().to_str().unwrap())).unwrap();
        assert_eq!(config.listen_addr, "0.0.0.0:3000");
        assert_eq!(config.queue_size, 2048);
        assert_eq!(config.max_concurrency, 8);
    }

    #[test]
    fn test_app_config_load_v2() {
        // 测试加载 V2 配置
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[server]
listen_addr = "0.0.0.0:8080"
queue_size = 512

[storage]
image_dir = "v2/images"
image_url_prefix = "/v2-images"

[[bots]]
app_id = "v2_app"
token = "v2_token"
base_url = "https://v2.example.com"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let config = AppConfig::load(Some(tmpfile.path().to_str().unwrap())).unwrap();
        assert_eq!(config.listen_addr, "0.0.0.0:8080");
        assert_eq!(config.queue_size, 512);
        assert_eq!(config.image_dir, "v2/images");
        assert_eq!(config.image_url_prefix, "/v2-images");
    }

    #[test]
    fn test_app_config_load_nonexistent() {
        // 测试加载不存在的配置文件
        let result = AppConfig::load(Some("/nonexistent/path/config.toml"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("读取配置失败"));
    }

    #[test]
    fn test_app_config_load_invalid_toml() {
        // 测试加载无效的 TOML 配置
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        tmpfile.write_all(b"invalid toml content {{{").unwrap();
        tmpfile.flush().unwrap();

        let result = AppConfig::load(Some(tmpfile.path().to_str().unwrap()));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("解析配置失败"));
    }

    #[test]
    fn test_app_config_v2_from_json() {
        // 测试从 JSON 解析 V2 配置
        let json = r#"{
            "config_version": 2,
            "server": {
                "listen_addr": "0.0.0.0:3000",
                "queue_size": 2048
            },
            "storage": {
                "image_dir": "data/images",
                "image_url_prefix": "/images",
                "external_base_url": null
            },
            "defaults": {},
            "bots": [],
            "ai_profiles": [],
            "tools": [],
            "rule_templates": [],
            "rule_instances": []
        }"#;

        let config = AppConfigV2::from_json(json).unwrap();
        assert_eq!(config.config_version, 2);
        assert_eq!(config.server.listen_addr, "0.0.0.0:3000");
    }

    #[test]
    fn test_app_config_v2_to_json() {
        // 测试 V2 配置序列化为 JSON
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let json = config.to_json().unwrap();
        assert!(json.contains("\"config_version\": 2"));
        assert!(json.contains("\"listen_addr\""));
    }

    #[test]
    fn test_app_config_v2_validate_empty_bot_fields() {
        // 测试验证空的 bot 字段
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![BotConfigV2 {
                app_id: "".to_string(),
                base_url: "".to_string(),
                token: Some("token".to_string()),
                ..Default::default()
            }],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("app_id 不能为空")));
        assert!(errors.iter().any(|e| e.contains("base_url 不能为空")));
    }

    #[test]
    fn test_app_config_v2_validate_empty_ai_profile_fields() {
        // 测试验证空的 AI profile 字段
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![AiProfileV2 {
                id: "".to_string(),
                model: "".to_string(),
                ..Default::default()
            }],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("id 不能为空")));
        assert!(errors.iter().any(|e| e.contains("model 不能为空")));
    }

    #[test]
    fn test_app_config_v2_validate_duplicate_profiles() {
        // 测试验证重复的 AI profile ID
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![
                AiProfileV2 {
                    id: "profile1".to_string(),
                    model: "gpt-4".to_string(),
                    ..Default::default()
                },
                AiProfileV2 {
                    id: "profile1".to_string(),
                    model: "gpt-3.5".to_string(),
                    ..Default::default()
                },
            ],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("重复的 id")));
    }

    #[test]
    fn test_app_config_v2_validate_missing_tool_reference() {
        // 测试验证 AI profile 引用不存在的工具
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![AiProfileV2 {
                id: "profile1".to_string(),
                model: "gpt-4".to_string(),
                tool_ids: vec!["nonexistent_tool".to_string()],
                ..Default::default()
            }],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("引用的工具不存在")));
    }

    #[test]
    fn test_app_config_v2_validate_empty_tool_fields() {
        // 测试验证空的工具字段
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![ToolConfigV2 {
                id: "".to_string(),
                program: "".to_string(),
                ..Default::default()
            }],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("id 不能为空")));
        assert!(errors.iter().any(|e| e.contains("program 不能为空")));
    }

    #[test]
    fn test_app_config_v2_validate_duplicate_tools() {
        // 测试验证重复的工具 ID
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![
                ToolConfigV2 {
                    id: "tool1".to_string(),
                    program: "echo".to_string(),
                    ..Default::default()
                },
                ToolConfigV2 {
                    id: "tool1".to_string(),
                    program: "cat".to_string(),
                    ..Default::default()
                },
            ],
            rule_templates: vec![],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("重复的 id")));
    }

    #[test]
    fn test_app_config_v2_validate_empty_template_fields() {
        // 测试验证空的规则模板字段
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![RuleTemplateV2 {
                id: "".to_string(),
                ..Default::default()
            }],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("id 不能为空")));
    }

    #[test]
    fn test_app_config_v2_validate_duplicate_templates() {
        // 测试验证重复的模板 ID
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![
                RuleTemplateV2 {
                    id: "template1".to_string(),
                    ..Default::default()
                },
                RuleTemplateV2 {
                    id: "template1".to_string(),
                    ..Default::default()
                },
            ],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("重复的 id")));
    }

    #[test]
    fn test_app_config_v2_validate_template_missing_profile() {
        // 测试验证模板引用不存在的 AI profile
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![RuleTemplateV2 {
                id: "template1".to_string(),
                action: TemplateActionV2 {
                    ai_profile: Some("nonexistent_profile".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            rule_instances: vec![],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.contains("引用的 ai_profile 不存在")));
    }

    #[test]
    fn test_app_config_v2_validate_empty_instance_fields() {
        // 测试验证空的实例字段
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![RuleInstanceV2 {
                id: "".to_string(),
                template: "".to_string(),
                ..Default::default()
            }],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("id 不能为空")));
        assert!(errors.iter().any(|e| e.contains("template 不能为空")));
    }

    #[test]
    fn test_app_config_v2_validate_instance_missing_template() {
        // 测试验证实例引用不存在的模板
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![],
            rule_instances: vec![RuleInstanceV2 {
                id: "instance1".to_string(),
                template: "nonexistent_template".to_string(),
                ..Default::default()
            }],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("引用的模板不存在")));
    }

    #[test]
    fn test_app_config_v2_validate_duplicate_instances() {
        // 测试验证重复的实例 ID
        let template = RuleTemplateV2 {
            id: "template1".to_string(),
            ..Default::default()
        };
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![template],
            rule_instances: vec![
                RuleInstanceV2 {
                    id: "instance1".to_string(),
                    template: "template1".to_string(),
                    ..Default::default()
                },
                RuleInstanceV2 {
                    id: "instance1".to_string(),
                    template: "template1".to_string(),
                    ..Default::default()
                },
            ],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("重复的 id")));
    }

    #[test]
    fn test_app_config_v2_validate_invalid_channel() {
        // 测试验证无效的 channel 值
        let template = RuleTemplateV2 {
            id: "template1".to_string(),
            ..Default::default()
        };
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![template],
            rule_instances: vec![RuleInstanceV2 {
                id: "instance1".to_string(),
                template: "template1".to_string(),
                channel: Some("invalid_channel".to_string()),
                ..Default::default()
            }],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.contains("channel 必须是 private/group/both")));
    }

    #[test]
    fn test_app_config_v2_validate_instance_override_missing_profile() {
        // 测试验证实例覆盖引用不存在的 AI profile
        let template = RuleTemplateV2 {
            id: "template1".to_string(),
            ..Default::default()
        };
        let config = AppConfigV2 {
            config_version: 2,
            server: ServerConfigV2::default(),
            storage: StorageConfigV2::default(),
            defaults: DefaultsV2::default(),
            bots: vec![],
            ai_profiles: vec![],
            tools: vec![],
            rule_templates: vec![template],
            rule_instances: vec![RuleInstanceV2 {
                id: "instance1".to_string(),
                template: "template1".to_string(),
                overrides: Some(InstanceOverridesV2 {
                    ai_profile: Some("nonexistent_profile".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }],
        };

        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| e.contains("引用的 ai_profile 不存在")));
    }

    #[test]
    fn test_pick_credential_from_env() {
        // 测试从环境变量获取凭证
        std::env::set_var("TEST_TOKEN_ENV", "env_token_value");

        let result = pick_credential("test", &None, &Some("TEST_TOKEN_ENV".to_string())).unwrap();

        assert_eq!(result, "env_token_value");
        std::env::remove_var("TEST_TOKEN_ENV");
    }

    #[test]
    fn test_pick_credential_env_not_found() {
        // 测试环境变量不存在
        let result = pick_credential("test", &None, &Some("NONEXISTENT_ENV_VAR".to_string()));

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到"));
    }

    #[test]
    fn test_pick_credential_empty_value() {
        // 测试空值应被忽略
        let result = pick_credential("test", &Some("   ".to_string()), &None);

        assert!(result.is_err());
    }

    #[test]
    fn test_pick_optional_credential_from_env() {
        // 测试从环境变量获取可选凭证
        std::env::set_var("TEST_OPTIONAL_ENV", "optional_value");

        let result =
            pick_optional_credential("test", &None, &Some("TEST_OPTIONAL_ENV".to_string()))
                .unwrap();

        assert_eq!(result, Some("optional_value".to_string()));
        std::env::remove_var("TEST_OPTIONAL_ENV");
    }

    #[test]
    fn test_pick_optional_credential_empty_value() {
        // 测试空值返回 None
        let result = pick_optional_credential("test", &Some("   ".to_string()), &None).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_app_config_v2_into_v1_basic() {
        // 测试 V2 配置转换为 V1（基础配置）
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[server]
listen_addr = "0.0.0.0:8080"
queue_size = 1024

[storage]
image_dir = "test/images"
image_url_prefix = "/test-images"

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.listen_addr, "0.0.0.0:8080");
        assert_eq!(v1.queue_size, 1024);
        assert_eq!(v1.image_dir, "test/images");
        assert_eq!(v1.bots.len(), 1);
        assert_eq!(v1.bots[0].app_id, "test_app");
    }

    #[test]
    fn test_app_config_v2_into_v1_with_rules() {
        // 测试 V2 配置转换为 V1（包含规则）
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"
kind = "text"

[rule_templates.match]
contains = "hello"

[rule_templates.action]
reply_text = "Hi there!"

[[rule_instances]]
id = "instance1"
template = "template1"
channel = "group"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots.len(), 1);
        assert_eq!(v1.bots[0].rules.len(), 1);
        assert_eq!(v1.bots[0].rules[0].kind, RuleKind::Text);
        assert_eq!(
            v1.bots[0].rules[0].r#match.contains,
            Some("hello".to_string())
        );
        assert_eq!(
            v1.bots[0].rules[0].action.reply_text,
            Some("Hi there!".to_string())
        );
        assert_eq!(v1.bots[0].rules[0].chat, Some(ChatKind::Group));
    }

    #[test]
    fn test_app_config_v2_into_v1_disabled_instance() {
        // 测试禁用的规则实例不会被转换
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"

[[rule_instances]]
id = "instance1"
template = "template1"
enabled = false
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots.len(), 1);
        assert_eq!(v1.bots[0].rules.len(), 0);
    }

    #[test]
    fn test_app_config_v2_into_v1_channel_private() {
        // 测试 channel = private 转换
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"

[[rule_instances]]
id = "instance1"
template = "template1"
channel = "private"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots[0].rules[0].chat, Some(ChatKind::Private));
    }

    #[test]
    fn test_app_config_v2_into_v1_channel_both() {
        // 测试 channel = both 转换（应为 None）
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"

[[rule_instances]]
id = "instance1"
template = "template1"
channel = "both"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots[0].rules[0].chat, None);
    }

    #[test]
    fn test_app_config_v2_into_v1_priority_sorting() {
        // 测试规则实例按优先级排序
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"
[rule_templates.action]
reply_text = "First"

[[rule_templates]]
id = "template2"
[rule_templates.action]
reply_text = "Second"

[[rule_instances]]
id = "instance1"
template = "template1"
priority = 10

[[rule_instances]]
id = "instance2"
template = "template2"
priority = 5
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots[0].rules.len(), 2);
        // 优先级 5 应该在前面
        assert_eq!(
            v1.bots[0].rules[0].action.reply_text,
            Some("Second".to_string())
        );
        assert_eq!(
            v1.bots[0].rules[1].action.reply_text,
            Some("First".to_string())
        );
    }

    #[test]
    fn test_app_config_v2_into_v1_overrides() {
        // 测试实例覆盖配置
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[defaults]
reply_mode = "quote"
log = true

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"
[rule_templates.action]
reply_text = "Template reply"
reply_mode = "at"

[[rule_instances]]
id = "instance1"
template = "template1"

[rule_instances.overrides]
reply_text = "Overridden reply"
reply_mode = "quote_and_at"
log = false
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        let rule = &v1.bots[0].rules[0];
        assert_eq!(rule.action.reply_text, Some("Overridden reply".to_string()));
        assert_eq!(rule.action.reply_mode, Some(ReplyMode::QuoteAndAt));
        assert_eq!(rule.action.log, Some(false));
    }

    #[test]
    fn test_app_config_v2_into_v1_with_ai_profile() {
        // 测试包含 AI profile 的转换
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[ai_profiles]]
id = "gpt4"
model = "gpt-4"
provider = "openai"
api_key = "test_key"
system_prompt = "You are a helpful assistant"
user_prefix = "User: "

[[rule_templates]]
id = "ai_template"
[rule_templates.action]
ai_profile = "gpt4"

[[rule_instances]]
id = "ai_instance"
template = "ai_template"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        let rule = &v1.bots[0].rules[0];
        assert!(rule.action.ai.is_some());
        let ai = rule.action.ai.as_ref().unwrap();
        assert_eq!(ai.model, "gpt-4");
        assert_eq!(ai.provider, Some("openai".to_string()));
        assert_eq!(ai.api_key, Some("test_key".to_string()));
        assert_eq!(
            ai.system_prompt,
            Some("You are a helpful assistant".to_string())
        );
        assert_eq!(ai.user_prefix, Some("User: ".to_string()));
    }

    #[test]
    fn test_app_config_v2_into_v1_with_tools() {
        // 测试包含工具的 AI profile 转换
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[tools]]
id = "tool1"
program = "echo"
args = ["hello"]
description = "Echo tool"
timeout_secs = 30

[[ai_profiles]]
id = "gpt4"
model = "gpt-4"
tool_ids = ["tool1"]

[[rule_templates]]
id = "ai_template"
[rule_templates.action]
ai_profile = "gpt4"

[[rule_instances]]
id = "ai_instance"
template = "ai_template"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        let rule = &v1.bots[0].rules[0];
        assert!(rule.action.ai.is_some());
        let ai = rule.action.ai.as_ref().unwrap();
        assert_eq!(ai.tools.len(), 1);
        assert_eq!(ai.tools[0].name, "tool1");
        assert_eq!(ai.tools[0].description, Some("Echo tool".to_string()));
        assert!(ai.tools[0].command.is_some());
        let cmd = ai.tools[0].command.as_ref().unwrap();
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello"]);
        assert_eq!(cmd.timeout_secs, Some(30));
    }

    #[test]
    fn test_app_config_v2_into_v1_tool_default_parameters() {
        // 测试工具默认 parameters
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[tools]]
id = "tool1"
program = "echo"

[[ai_profiles]]
id = "gpt4"
model = "gpt-4"
tool_ids = ["tool1"]

[[rule_templates]]
id = "ai_template"
[rule_templates.action]
ai_profile = "gpt4"

[[rule_instances]]
id = "ai_instance"
template = "ai_template"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        let ai = v1.bots[0].rules[0].action.ai.as_ref().unwrap();
        assert_eq!(ai.tools[0].parameters, Some(json!({"type": "object"})));
    }

    #[test]
    fn test_app_config_v2_into_v1_system_prompt_from_file() {
        // 测试从文件读取 system_prompt
        use std::io::Write;
        use tempfile::{NamedTempFile, TempDir};

        let tempdir = TempDir::new().unwrap();
        let prompt_file = tempdir.path().join("prompt.txt");
        std::fs::write(&prompt_file, "System prompt from file").unwrap();

        let mut config_file = NamedTempFile::new_in(tempdir.path()).unwrap();
        let config_content = format!(
            r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[ai_profiles]]
id = "gpt4"
model = "gpt-4"
system_prompt_file = "prompt.txt"

[[rule_templates]]
id = "ai_template"
[rule_templates.action]
ai_profile = "gpt4"

[[rule_instances]]
id = "ai_instance"
template = "ai_template"
"#
        );
        config_file.write_all(config_content.as_bytes()).unwrap();
        config_file.flush().unwrap();

        let v2 = AppConfigV2::parse(&config_content).unwrap();
        let v1 = v2.into_v1(config_file.path()).unwrap();

        let ai = v1.bots[0].rules[0].action.ai.as_ref().unwrap();
        assert_eq!(
            ai.system_prompt,
            Some("System prompt from file".to_string())
        );
    }

    #[test]
    fn test_app_config_v2_into_v1_missing_template() {
        // 测试引用不存在的模板
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_instances]]
id = "instance1"
template = "nonexistent_template"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let result = v2.into_v1(tmpfile.path());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到规则模板"));
    }

    #[test]
    fn test_app_config_v2_into_v1_missing_ai_profile() {
        // 测试引用不存在的 AI profile
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"
[rule_templates.action]
ai_profile = "nonexistent_profile"

[[rule_instances]]
id = "instance1"
template = "template1"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let result = v2.into_v1(tmpfile.path());

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("未找到 AI Profile"));
    }

    #[test]
    fn test_app_config_v2_into_v1_bot_token_env() {
        // 测试使用环境变量的 bot token
        use std::io::Write;
        use tempfile::NamedTempFile;

        std::env::set_var("TEST_BOT_TOKEN", "token_from_env");

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token_env = "TEST_BOT_TOKEN"
base_url = "https://api.example.com"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(v1.bots[0].token, "token_from_env");
        std::env::remove_var("TEST_BOT_TOKEN");
    }

    #[test]
    fn test_app_config_v2_into_v1_bot_webhook_secret_env() {
        // 测试使用环境变量的 webhook_secret
        use std::io::Write;
        use tempfile::NamedTempFile;

        std::env::set_var("TEST_WEBHOOK_SECRET", "secret_from_env");

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"
webhook_secret_env = "TEST_WEBHOOK_SECRET"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        assert_eq!(
            v1.bots[0].webhook_secret,
            Some("secret_from_env".to_string())
        );
        std::env::remove_var("TEST_WEBHOOK_SECRET");
    }

    #[test]
    fn test_app_config_v2_into_v1_require_mention_cascade() {
        // 测试 require_mention 的级联配置
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmpfile = NamedTempFile::new().unwrap();
        let config_content = r#"
config_version = 2

[defaults.ai]
require_mention = true

[[bots]]
app_id = "test_app"
token = "test_token"
base_url = "https://api.example.com"

[[rule_templates]]
id = "template1"
[rule_templates.defaults]
require_mention = false

[[rule_instances]]
id = "instance1"
template = "template1"

[[rule_templates]]
id = "template2"

[[rule_instances]]
id = "instance2"
template = "template2"
"#;
        tmpfile.write_all(config_content.as_bytes()).unwrap();
        tmpfile.flush().unwrap();

        let v2 = AppConfigV2::parse(config_content).unwrap();
        let v1 = v2.into_v1(tmpfile.path()).unwrap();

        // instance1 使用模板 defaults
        assert_eq!(v1.bots[0].rules[0].action.require_mention, Some(false));
        // instance2 使用全局 defaults
        assert_eq!(v1.bots[0].rules[1].action.require_mention, Some(true));
    }

    #[test]
    fn test_rule_kind_all_variants() {
        // 测试所有 RuleKind 变体的序列化
        let kinds = vec![
            (RuleKind::Text, "text"),
            (RuleKind::Image, "image"),
            (RuleKind::Voice, "voice"),
            (RuleKind::Video, "video"),
            (RuleKind::Emoji, "emoji"),
            (RuleKind::Link, "link"),
            (RuleKind::FileNotice, "file_notice"),
            (RuleKind::ContactEvent, "contact_event"),
            (RuleKind::Any, "any"),
        ];

        for (kind, expected) in kinds {
            let json = serde_json::to_string(&kind).unwrap();
            assert_eq!(json, format!("\"{}\"", expected));
        }
    }

    #[test]
    fn test_reply_mode_all_variants() {
        // 测试所有 ReplyMode 变体的序列化
        let modes = vec![
            (ReplyMode::None, "none"),
            (ReplyMode::Quote, "quote"),
            (ReplyMode::At, "at"),
            (ReplyMode::QuoteAndAt, "quote_and_at"),
        ];

        for (mode, expected) in modes {
            let json = serde_json::to_string(&mode).unwrap();
            assert_eq!(json, format!("\"{}\"", expected));
        }
    }
}
