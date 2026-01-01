//! 配置相关 API 处理函数

use super::state::{compute_etag, ApiState};
use crate::config::AppConfigV2;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// 通用 API 响应
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            errors: None,
        }
    }

    fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
            errors: None,
        }
    }

    fn validation_errors(errors: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some("配置校验失败".to_string()),
            errors: Some(errors),
        }
    }
}

/// 配置响应（带 ETag）
#[derive(Serialize)]
struct ConfigResponse {
    config: AppConfigV2,
    etag: String,
}

/// GET /api/config - 获取当前配置
pub async fn get_config(State(state): State<ApiState>) -> impl IntoResponse {
    let path = state.config_path();

    match tokio::fs::read_to_string(path).await {
        Ok(content) => {
            match AppConfigV2::parse(&content) {
                Ok(config) => {
                    let etag = compute_etag(&content);
                    // 更新 state 中的 etag
                    state.update_meta(|m| m.etag = etag.clone()).await;
                    Json(ApiResponse::success(ConfigResponse { config, etag }))
                }
                Err(e) => Json(ApiResponse::error(format!("解析配置失败: {}", e))),
            }
        }
        Err(e) => Json(ApiResponse::error(format!("读取配置失败: {}", e))),
    }
}

/// Lint 请求
#[derive(Deserialize)]
pub struct LintRequest {
    /// JSON 格式的配置内容
    pub config: serde_json::Value,
}

/// Lint 响应
#[derive(Serialize)]
pub struct LintResponse {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// POST /api/config/lint - 校验配置
pub async fn lint_config(Json(req): Json<LintRequest>) -> impl IntoResponse {
    // 尝试解析为 AppConfigV2
    let config: AppConfigV2 = match serde_json::from_value(req.config) {
        Ok(c) => c,
        Err(e) => {
            return Json(ApiResponse::success(LintResponse {
                valid: false,
                errors: vec![format!("JSON 解析失败: {}", e)],
            }));
        }
    };

    // 执行语义校验
    let errors = config.validate();
    let valid = errors.is_empty();

    Json(ApiResponse::success(LintResponse { valid, errors }))
}

/// GET /api/config/meta - 获取配置元信息
pub async fn get_meta(State(state): State<ApiState>) -> impl IntoResponse {
    let meta = state.get_meta().await;
    Json(ApiResponse::success(meta))
}

/// Save 请求
#[derive(Deserialize)]
pub struct SaveRequest {
    /// JSON 格式的配置内容
    pub config: serde_json::Value,
    /// 预期的 ETag（用于乐观锁）
    #[serde(default)]
    pub expected_etag: Option<String>,
}

/// Save 响应
#[derive(Serialize)]
pub struct SaveResponse {
    pub etag: String,
    pub saved_at: chrono::DateTime<Utc>,
}

/// POST /api/config/save - 保存配置草稿
pub async fn save_config(
    State(state): State<ApiState>,
    Json(req): Json<SaveRequest>,
) -> impl IntoResponse {
    // 解析配置
    let config: AppConfigV2 = match serde_json::from_value(req.config) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<SaveResponse>::error(format!(
                    "JSON 解析失败: {}",
                    e
                ))),
            );
        }
    };

    // 校验配置
    let errors = config.validate();
    if !errors.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<SaveResponse>::validation_errors(errors)),
        );
    }

    // 检查 ETag（乐观锁）
    if let Some(expected) = req.expected_etag {
        let current_meta = state.get_meta().await;
        if !current_meta.etag.is_empty() && current_meta.etag != expected {
            return (
                StatusCode::CONFLICT,
                Json(ApiResponse::<SaveResponse>::error(
                    "配置已被修改，请刷新后重试",
                )),
            );
        }
    }

    // 序列化为 TOML
    let toml_content = match config.to_toml() {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<SaveResponse>::error(format!(
                    "序列化 TOML 失败: {}",
                    e
                ))),
            );
        }
    };

    // 写入文件
    let path = state.config_path();
    if let Err(e) = tokio::fs::write(path, &toml_content).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<SaveResponse>::error(format!(
                "写入文件失败: {}",
                e
            ))),
        );
    }

    // 计算新 ETag
    let new_etag = compute_etag(&toml_content);
    let now = Utc::now();

    // 更新元信息
    state
        .update_meta(|m| {
            m.etag = new_etag.clone();
            m.has_draft = true;
            m.last_saved_at = Some(now);
        })
        .await;

    tracing::info!(path = %path.display(), "配置已保存");

    (
        StatusCode::OK,
        Json(ApiResponse::success(SaveResponse {
            etag: new_etag,
            saved_at: now,
        })),
    )
}

/// Publish 请求
#[derive(Deserialize)]
pub struct PublishRequest {
    #[serde(default)]
    pub remark: Option<String>,
}

/// Publish 响应
#[derive(Serialize)]
pub struct PublishResponse {
    pub version: u64,
    pub published_at: chrono::DateTime<Utc>,
    pub backup_filename: String,
}

/// POST /api/config/publish - 发布配置
pub async fn publish_config(
    State(state): State<ApiState>,
    Json(req): Json<PublishRequest>,
) -> impl IntoResponse {
    // 确保配置文件存在
    let path = state.config_path();
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<PublishResponse>::error("配置文件不存在")),
        );
    }

    // 验证配置是否有效
    match tokio::fs::read_to_string(path).await {
        Ok(content) => match AppConfigV2::parse(&content) {
            Ok(config) => {
                let errors = config.validate();
                if !errors.is_empty() {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::<PublishResponse>::validation_errors(errors)),
                    );
                }
            }
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<PublishResponse>::error(format!(
                        "配置解析失败: {}",
                        e
                    ))),
                );
            }
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<PublishResponse>::error(format!(
                    "读取配置失败: {}",
                    e
                ))),
            );
        }
    }

    // 创建备份
    match state.create_backup(req.remark).await {
        Ok(info) => {
            // 更新元信息
            state
                .update_meta(|m| {
                    m.has_draft = false;
                    m.last_reload_at = Some(Utc::now());
                    m.last_reload_result = Some("published".to_string());
                })
                .await;

            tracing::info!(
                version = info.version,
                filename = %info.filename,
                "配置已发布"
            );

            (
                StatusCode::OK,
                Json(ApiResponse::success(PublishResponse {
                    version: info.version,
                    published_at: info.created_at,
                    backup_filename: info.filename,
                })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<PublishResponse>::error(format!(
                "创建备份失败: {}",
                e
            ))),
        ),
    }
}

/// Rollback 请求
#[derive(Deserialize)]
pub struct RollbackRequest {
    /// 要回滚到的版本号
    pub version: u64,
}

/// Rollback 响应
#[derive(Serialize)]
pub struct RollbackResponse {
    pub version: u64,
    pub rolled_back_at: chrono::DateTime<Utc>,
}

/// POST /api/config/rollback - 回滚到指定版本
pub async fn rollback_config(
    State(state): State<ApiState>,
    Json(req): Json<RollbackRequest>,
) -> impl IntoResponse {
    match state.restore_backup(req.version).await {
        Ok(()) => {
            let now = Utc::now();
            tracing::info!(version = req.version, "配置已回滚");

            (
                StatusCode::OK,
                Json(ApiResponse::success(RollbackResponse {
                    version: req.version,
                    rolled_back_at: now,
                })),
            )
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<RollbackResponse>::error(format!(
                "回滚失败: {}",
                e
            ))),
        ),
    }
}

/// Simulate 请求
#[derive(Deserialize)]
pub struct SimulateRequest {
    /// Bot 的 app_id
    pub app_id: String,
    /// 消息类型: text, image, voice, video, emoji, link, file_notice
    #[serde(default = "default_msg_kind")]
    pub msg_kind: String,
    /// 聊天类型: private, group
    #[serde(default = "default_chat")]
    pub chat: String,
    /// 消息内容
    pub content: String,
    /// 发送者 wxid
    #[serde(default)]
    pub from_wxid: Option<String>,
    /// 是否被 @ 了机器人
    #[serde(default)]
    pub mentioned: bool,
}

fn default_msg_kind() -> String {
    "text".to_string()
}

fn default_chat() -> String {
    "private".to_string()
}

/// 匹配到的规则信息
#[derive(Serialize)]
pub struct MatchedRule {
    pub instance_id: String,
    pub template_id: String,
    pub priority: i32,
    pub action_summary: String,
}

/// Simulate 响应
#[derive(Serialize)]
pub struct SimulateResponse {
    pub matched: bool,
    pub rules: Vec<MatchedRule>,
    pub final_action: Option<String>,
}

/// POST /api/config/simulate - 模拟消息匹配
pub async fn simulate_config(
    State(state): State<ApiState>,
    Json(req): Json<SimulateRequest>,
) -> impl IntoResponse {
    // 读取配置
    let path = state.config_path();
    let content = match tokio::fs::read_to_string(path).await {
        Ok(c) => c,
        Err(e) => {
            return Json(ApiResponse::<SimulateResponse>::error(format!(
                "读取配置失败: {}",
                e
            )));
        }
    };

    let config = match AppConfigV2::parse(&content) {
        Ok(c) => c,
        Err(e) => {
            return Json(ApiResponse::<SimulateResponse>::error(format!(
                "解析配置失败: {}",
                e
            )));
        }
    };

    // 检查 app_id 是否存在
    if !config.bots.iter().any(|b| b.app_id == req.app_id) {
        return Json(ApiResponse::<SimulateResponse>::error(format!(
            "未找到 bot: {}",
            req.app_id
        )));
    }

    // 构建模板映射
    let tmpl_map: std::collections::HashMap<String, &crate::config::RuleTemplateV2> = config
        .rule_templates
        .iter()
        .map(|t| (t.id.clone(), t))
        .collect();

    // 按优先级排序实例
    let mut instances = config.rule_instances.clone();
    instances.sort_by_key(|i| i.priority.unwrap_or(0));

    let mut matched_rules = Vec::new();
    let mut final_action = None;

    for inst in instances {
        // 跳过禁用的实例
        if matches!(inst.enabled, Some(false)) {
            continue;
        }

        let Some(tmpl) = tmpl_map.get(&inst.template) else {
            continue;
        };

        // 检查 channel
        if let Some(ref channel) = inst.channel {
            match channel.as_str() {
                "private" if req.chat != "private" => continue,
                "group" if req.chat != "group" => continue,
                _ => {}
            }
        }

        // 检查 from_wxid
        if let Some(ref wxid) = inst.from.wxid {
            if req.from_wxid.as_ref() != Some(wxid) {
                continue;
            }
        }

        // 检查 kind
        if let Some(ref kind) = tmpl.kind {
            let kind_str = format!("{:?}", kind).to_lowercase();
            if kind_str != "any" && kind_str != req.msg_kind.to_lowercase() {
                continue;
            }
        }

        // 检查 match 条件
        let match_cfg = &tmpl.r#match;
        let content = &req.content;

        // any 匹配
        if !matches!(match_cfg.any, Some(true)) {
            // equals
            if let Some(ref eq) = match_cfg.equals {
                if content.trim() != eq {
                    continue;
                }
            }
            // contains
            if let Some(ref cn) = match_cfg.contains {
                if !content.contains(cn) {
                    continue;
                }
            }
            // regex
            if let Some(ref re_str) = match_cfg.regex {
                if let Ok(re) = regex::Regex::new(re_str) {
                    if !re.is_match(content) {
                        continue;
                    }
                }
            }
        }

        // 检查 require_mention
        let require_mention = inst
            .overrides
            .as_ref()
            .and_then(|o| o.require_mention)
            .or(tmpl.action.require_mention)
            .or(tmpl.defaults.require_mention)
            .unwrap_or(false);

        if require_mention && req.chat == "group" && !req.mentioned {
            continue;
        }

        // 匹配成功，构建动作摘要
        let mut actions = Vec::new();
        if tmpl.action.ai_profile.is_some()
            || inst
                .overrides
                .as_ref()
                .and_then(|o| o.ai_profile.as_ref())
                .is_some()
        {
            let profile = inst
                .overrides
                .as_ref()
                .and_then(|o| o.ai_profile.clone())
                .or_else(|| tmpl.action.ai_profile.clone())
                .unwrap_or_default();
            actions.push(format!("ai({})", profile));
        }
        if tmpl.action.reply_text.is_some()
            || inst
                .overrides
                .as_ref()
                .and_then(|o| o.reply_text.as_ref())
                .is_some()
        {
            actions.push("reply_text".to_string());
        }
        if tmpl.action.log.unwrap_or(false)
            || inst.overrides.as_ref().and_then(|o| o.log).unwrap_or(false)
        {
            actions.push("log".to_string());
        }

        let action_summary = if actions.is_empty() {
            "no action".to_string()
        } else {
            actions.join(", ")
        };

        matched_rules.push(MatchedRule {
            instance_id: inst.id.clone(),
            template_id: inst.template.clone(),
            priority: inst.priority.unwrap_or(0),
            action_summary: action_summary.clone(),
        });

        // 第一个匹配的规则作为最终动作
        if final_action.is_none() {
            final_action = Some(action_summary);
        }
    }

    Json(ApiResponse::success(SimulateResponse {
        matched: !matched_rules.is_empty(),
        rules: matched_rules,
        final_action,
    }))
}

/// GET /api/config/export - 导出配置为 TOML
pub async fn export_config(State(state): State<ApiState>) -> impl IntoResponse {
    // 读取配置
    let path = state.config_path();
    let content = match tokio::fs::read_to_string(path).await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("读取配置失败: {}", e),
            )
                .into_response();
        }
    };

    // 返回 TOML 文件
    (
        StatusCode::OK,
        [
            ("Content-Type", "application/toml"),
            (
                "Content-Disposition",
                "attachment; filename=\"bot-app.v2.toml\"",
            ),
        ],
        content,
    )
        .into_response()
}

/// POST /api/config/import - 导入 TOML 配置
pub async fn import_config(State(state): State<ApiState>, body: String) -> impl IntoResponse {
    // 解析 TOML
    let config = match AppConfigV2::parse(&body) {
        Ok(c) => c,
        Err(e) => {
            return Json(ApiResponse::<()>::error(format!("解析 TOML 失败: {}", e)))
                .into_response();
        }
    };

    // 校验配置
    let errors = config.validate();
    if !errors.is_empty() {
        return Json(ApiResponse::<()>::validation_errors(errors)).into_response();
    }

    // 保存配置
    if let Err(e) = tokio::fs::write(state.config_path(), &body).await {
        return Json(ApiResponse::<()>::error(format!("写入配置失败: {}", e))).into_response();
    }

    // 更新 ETag
    let etag = compute_etag(&body);
    state
        .update_meta(|m| {
            m.etag = etag;
            m.has_draft = true;
            m.last_saved_at = Some(Utc::now());
        })
        .await;

    Json(ApiResponse::success(())).into_response()
}

/// GET /api/healthz - 健康检查
pub async fn healthz() -> impl IntoResponse {
    #[derive(Serialize)]
    struct Health {
        status: String,
        timestamp: String,
    }

    Json(Health {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_state() -> (ApiState, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("bot-app.v2.toml");
        let prompts_dir = temp_dir.path().join("prompts");
        let backup_dir = temp_dir.path().join("backups");

        let state = ApiState::new(config_path, prompts_dir, backup_dir);
        (state, temp_dir)
    }

    fn create_test_config_toml() -> String {
        r#"
config_version = 2

[server]
listen_addr = "0.0.0.0:3000"

[storage]
backend = "file"

[[bots]]
app_id = "test_bot"
base_url = "http://localhost:2531"
token = "test_token"

[[ai_profiles]]
id = "default"
model = "gpt-4"
provider = "openai"
api_key = "sk-test"
"#
        .to_string()
    }

    #[tokio::test]
    async fn test_healthz() {
        let response = healthz().await;
        let body = axum::body::to_bytes(response.into_response().into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert!(json["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_get_config_success() {
        let (state, temp_dir) = create_test_state();
        let config_path = temp_dir.path().join("bot-app.v2.toml");
        let toml_content = create_test_config_toml();
        fs::write(&config_path, &toml_content).unwrap();

        let response = get_config(axum::extract::State(state)).await;
        let body = axum::body::to_bytes(response.into_response().into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], true);
        assert!(json["data"]["config"]["config_version"].is_number());
        assert!(json["data"]["etag"].is_string());
    }

    #[test]
    fn test_default_msg_kind() {
        assert_eq!(default_msg_kind(), "text");
    }

    #[test]
    fn test_default_chat() {
        assert_eq!(default_chat(), "private");
    }
}
