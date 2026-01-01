//! 配置管理 API 模块
//!
//! 提供配置的读取、校验、保存、发布、回滚和模拟命中等功能。

pub mod auth;
mod config;
mod pages;
mod prompts;
mod state;

pub use state::ApiState;

use axum::{
    routing::{get, post, put},
    Router,
};

/// 创建 API 路由
pub fn api_router(state: ApiState) -> Router {
    Router::new()
        // 健康检查（无需鉴权）
        .route("/healthz", get(config::healthz))
        // 配置相关
        .route("/config", get(config::get_config))
        .route("/config/lint", post(config::lint_config))
        .route("/config/meta", get(config::get_meta))
        .route("/config/save", post(config::save_config))
        .route("/config/publish", post(config::publish_config))
        .route("/config/rollback", post(config::rollback_config))
        .route("/config/simulate", post(config::simulate_config))
        .route("/config/export", get(config::export_config))
        .route("/config/import", post(config::import_config))
        // Prompts 相关
        .route("/prompts", get(prompts::list_prompts))
        .route("/prompts/:name", get(prompts::get_prompt))
        .route("/prompts/:name", put(prompts::put_prompt))
        .with_state(state)
}

/// 创建 Pages 路由 (htmx HTML 片段)
pub fn pages_router(state: ApiState) -> Router {
    Router::new()
        // Dashboard
        .route("/dashboard", get(pages::dashboard))
        // Bots
        .route("/bots", get(pages::bots_list))
        .route("/bots/new", get(pages::bot_new_form))
        .route("/bots/edit/:id", get(pages::bot_edit_form))
        .route("/bots/save", post(pages::bot_save))
        .route("/bots/delete/:id", post(pages::bot_delete))
        // AI Profiles
        .route("/ai-profiles", get(pages::ai_profiles_list))
        .route("/ai-profiles/new", get(pages::ai_profile_new_form))
        .route("/ai-profiles/edit/:id", get(pages::ai_profile_edit_form))
        .route("/ai-profiles/save", post(pages::ai_profile_save))
        .route("/ai-profiles/delete/:id", post(pages::ai_profile_delete))
        // Tools
        .route("/tools", get(pages::tools_list))
        .route("/tools/new", get(pages::tool_new_form))
        .route("/tools/edit/:id", get(pages::tool_edit_form))
        .route("/tools/save", post(pages::tool_save))
        .route("/tools/delete/:id", post(pages::tool_delete))
        // Rules
        .route("/rules", get(pages::rules_page))
        .route("/rule-templates/new", get(pages::rule_template_new_form))
        .route(
            "/rule-templates/edit/:id",
            get(pages::rule_template_edit_form),
        )
        .route("/rule-templates/save", post(pages::rule_template_save))
        .route(
            "/rule-templates/delete/:id",
            post(pages::rule_template_delete),
        )
        .route("/rule-instances/new", get(pages::rule_instance_new_form))
        .route(
            "/rule-instances/edit/:id",
            get(pages::rule_instance_edit_form),
        )
        .route("/rule-instances/save", post(pages::rule_instance_save))
        .route(
            "/rule-instances/delete/:id",
            post(pages::rule_instance_delete),
        )
        // Prompts
        .route("/prompts", get(pages::prompts_page))
        .route("/prompts/new", get(pages::prompt_new))
        .route("/prompts/edit/:name", get(pages::prompt_edit))
        .route("/prompts/create", post(pages::prompt_create))
        // Simulator
        .route("/simulator", get(pages::simulator_page))
        // Settings
        .route("/settings", get(pages::settings_page))
        .route("/settings/save", post(pages::settings_save))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_state() -> (ApiState, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("bot-app.v2.toml");
        let prompts_dir = temp_dir.path().join("prompts");
        let backup_dir = temp_dir.path().join("backups");

        let state = ApiState::new(config_path, prompts_dir, backup_dir);
        (state, temp_dir)
    }

    #[test]
    fn test_api_router_creation() {
        let (state, _temp_dir) = create_test_state();
        let router = api_router(state);

        assert!(!format!("{:?}", router).is_empty());
    }

    #[test]
    fn test_pages_router_creation() {
        let (state, _temp_dir) = create_test_state();
        let router = pages_router(state);

        assert!(!format!("{:?}", router).is_empty());
    }
}
