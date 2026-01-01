//! HTML 页面渲染模块
//!
//! 为 htmx 提供 HTML 片段响应

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use axum_htmx::HxRequest;
use serde::Deserialize;

use super::state::{compute_etag, ApiState};
use crate::config::{
    AiProfileV2, AppConfigV2, BotConfigV2, DefaultsAiV2, DefaultsV2, InstanceOverridesV2,
    MatchConfigV2, RuleInstanceV2, RuleKind, RuleTemplateV2, ServerConfigV2, StorageConfigV2,
    TemplateActionV2, TemplateDefaultsV2, ToolConfigV2,
};

/// 检查是否为 htmx 请求，如果不是则重定向到主页
fn require_htmx(is_htmx: bool) -> Option<Response> {
    if !is_htmx {
        Some(Redirect::to("/").into_response())
    } else {
        None
    }
}

/// 读取配置文件并解析为 V2 配置
async fn load_config(state: &ApiState) -> Result<AppConfigV2, String> {
    let content = tokio::fs::read_to_string(state.config_path())
        .await
        .map_err(|e| format!("读取配置文件失败: {}", e))?;

    AppConfigV2::parse(&content).map_err(|e| format!("解析配置失败: {}", e))
}

/// 保存配置到文件
async fn save_config(state: &ApiState, config: &AppConfigV2) -> Result<(), String> {
    let content = config.to_toml().map_err(|e| format!("序列化失败: {}", e))?;

    tokio::fs::write(state.config_path(), &content)
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    // 更新 ETag
    let etag = compute_etag(&content);
    state
        .update_meta(|m| {
            m.etag = etag;
            m.has_draft = true;
        })
        .await;

    Ok(())
}

/// Dashboard 页面 - 概览
pub async fn dashboard(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let meta = state.get_meta().await;

    // 尝试加载配置获取统计信息
    let (bots_count, profiles_count, tools_count, rules_count) = match load_config(&state).await {
        Ok(config) => (
            config.bots.len(),
            config.ai_profiles.len(),
            config.tools.len(),
            config.rule_instances.len(),
        ),
        Err(_) => (0, 0, 0, 0),
    };

    let backup_rows: String = meta
        .available_backups
        .iter()
        .take(5)
        .map(|b| {
            format!(
                r##"<tr>
                    <td>v{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>
                        <button class="btn btn-xs btn-outline"
                                hx-post="/api/config/rollback"
                                hx-vals='{{"version": {}}}'
                                hx-confirm="确定回滚到 v{} 版本？"
                                hx-target="#main"
                                hx-swap="none">
                            回滚
                        </button>
                    </td>
                </tr>"##,
                b.version,
                b.created_at.format("%Y-%m-%d %H:%M:%S"),
                b.remark.as_deref().unwrap_or("-"),
                b.version,
                b.version
            )
        })
        .collect();

    let last_published = meta
        .last_published_at
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "-".to_string());

    let last_reload = meta
        .last_reload_at
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "-".to_string());

    let reload_result = meta.last_reload_result.as_deref().unwrap_or("-");

    let content = format!(
        r##"
<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-6">
    <!-- 统计卡片 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm text-base-content/70">Bots</h2>
            <p class="text-3xl font-bold">{}</p>
        </div>
    </div>
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm text-base-content/70">AI Profiles</h2>
            <p class="text-3xl font-bold">{}</p>
        </div>
    </div>
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm text-base-content/70">工具</h2>
            <p class="text-3xl font-bold">{}</p>
        </div>
    </div>
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm text-base-content/70">规则实例</h2>
            <p class="text-3xl font-bold">{}</p>
        </div>
    </div>
</div>

<div class="grid gap-4 md:grid-cols-2">
    <!-- 配置状态 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title">配置状态</h2>
            <div class="overflow-x-auto">
                <table class="table table-sm">
                    <tbody>
                        <tr>
                            <td class="text-base-content/70">版本</td>
                            <td class="font-mono">v{}</td>
                        </tr>
                        <tr>
                            <td class="text-base-content/70">ETag</td>
                            <td class="font-mono text-xs">{}</td>
                        </tr>
                        <tr>
                            <td class="text-base-content/70">草稿状态</td>
                            <td>{}</td>
                        </tr>
                        <tr>
                            <td class="text-base-content/70">最后发布</td>
                            <td>{}</td>
                        </tr>
                        <tr>
                            <td class="text-base-content/70">最后加载</td>
                            <td>{}</td>
                        </tr>
                        <tr>
                            <td class="text-base-content/70">加载结果</td>
                            <td>{}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="flex gap-2 mt-4">
                <a href="/api/config/export" class="btn btn-sm btn-outline">
                    导出配置
                </a>
                <label class="btn btn-sm btn-outline cursor-pointer">
                    导入配置
                    <input type="file" class="hidden" accept=".toml" onchange="handleImport(this)" />
                </label>
            </div>
        </div>
    </div>

    <!-- 备份历史 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title">备份历史</h2>
            <div class="overflow-x-auto">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>版本</th>
                            <th>时间</th>
                            <th>备注</th>
                            <th>操作</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
            {}
        </div>
    </div>
</div>
"##,
        bots_count,
        profiles_count,
        tools_count,
        rules_count,
        meta.version,
        if meta.etag.len() > 16 {
            format!("{}...", &meta.etag[..16])
        } else {
            meta.etag.clone()
        },
        if meta.has_draft {
            r##"<span class="badge badge-warning">有草稿</span>"##
        } else {
            r##"<span class="badge badge-success">已同步</span>"##
        },
        last_published,
        last_reload,
        reload_result,
        if backup_rows.is_empty() {
            r##"<tr><td colspan="4" class="text-center text-base-content/50">暂无备份</td></tr>"##
                .to_string()
        } else {
            backup_rows
        },
        if meta.available_backups.len() > 5 {
            format!(
                r##"<p class="text-sm text-base-content/50 mt-2">还有 {} 个历史版本...</p>"##,
                meta.available_backups.len() - 5
            )
        } else {
            String::new()
        }
    );

    Html(content)
}

/// Bots 列表页面
pub async fn bots_list(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let rows: String = config
        .bots
        .iter()
        .map(|bot| {
            let id = bot.id.as_deref().unwrap_or(&bot.app_id);
            let token_display = if bot.token_env.is_some() {
                format!("${{{}}}", bot.token_env.as_ref().unwrap())
            } else {
                "***".to_string()
            };
            let tags = if bot.tags.is_empty() {
                String::new()
            } else {
                bot.tags.join(", ")
            };

            format!(
                r##"<tr>
                    <td class="font-mono">{}</td>
                    <td class="font-mono text-xs">{}</td>
                    <td>{}</td>
                    <td class="font-mono text-xs">{}</td>
                    <td>{}</td>
                    <td>
                        <div class="flex gap-1">
                            <button class="btn btn-ghost btn-xs"
                                    hx-get="/pages/bots/edit/{}"
                                    hx-target="#modal-content"
                                    onclick="openModal()">
                                编辑
                            </button>
                            <button class="btn btn-error btn-xs"
                                    hx-post="/pages/bots/delete/{}"
                                    hx-target="#main"
                                    hx-confirm="确定删除 Bot {} 吗？">
                                删除
                            </button>
                        </div>
                    </td>
                </tr>"##,
                id, bot.app_id, bot.base_url, token_display, tags, id, id, id
            )
        })
        .collect();

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">Bots 管理</h1>
    <button class="btn btn-primary btn-sm"
            hx-get="/pages/bots/new"
            hx-target="#modal-content"
            onclick="openModal()">
        添加 Bot
    </button>
</div>

<div class="card bg-base-100 shadow-sm">
    <div class="card-body">
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>App ID</th>
                        <th>Base URL</th>
                        <th>Token</th>
                        <th>Tags</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
    </div>
</div>
"##,
        if rows.is_empty() {
            r##"<tr><td colspan="6" class="text-center text-base-content/50">暂无 Bot 配置</td></tr>"##.to_string()
        } else {
            rows
        }
    );

    Html(content)
}

/// Bot 编辑表单
pub async fn bot_edit_form(Path(id): Path<String>, State(state): State<ApiState>) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let bot = config
        .bots
        .iter()
        .find(|b| b.id.as_deref().unwrap_or(&b.app_id) == id);

    let (title, bot_id, app_id, base_url, token_env, webhook_secret_env, tags) = match bot {
        Some(b) => (
            format!("编辑 Bot: {}", id),
            b.id.clone().unwrap_or_default(),
            b.app_id.clone(),
            b.base_url.clone(),
            b.token_env.clone().unwrap_or_default(),
            b.webhook_secret_env.clone().unwrap_or_default(),
            if b.tags.is_empty() {
                String::new()
            } else {
                b.tags.join(", ")
            },
        ),
        None => (
            "添加 Bot".to_string(),
            String::new(),
            String::new(),
            "https://www.geweapi.com".to_string(),
            String::new(),
            String::new(),
            String::new(),
        ),
    };

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">{}</h3>
<form hx-post="/pages/bots/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="{}" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID (可选，默认使用 App ID)</span></div>
        <input type="text" class="input input-bordered" name="id" value="{}" placeholder="留空则使用 App ID" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">App ID *</span></div>
        <input type="text" class="input input-bordered" name="app_id" value="{}" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Base URL *</span></div>
        <input type="text" class="input input-bordered" name="base_url" value="{}" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Token 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="token_env" value="{}" placeholder="GEWE_BOT_TOKEN_XXX" />
        <div class="label"><span class="label-text-alt text-base-content/50">留空则需要在 token 字段直接填写</span></div>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Webhook Secret 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="webhook_secret_env" value="{}" placeholder="GEWE_WEBHOOK_SECRET_XXX" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Tags (逗号分隔)</span></div>
        <input type="text" class="input input-bordered" name="tags" value="{}" placeholder="prod, test" />
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        title, id, bot_id, app_id, base_url, token_env, webhook_secret_env, tags
    );

    Html(content)
}

/// 新建 Bot 表单
pub async fn bot_new_form() -> Html<String> {
    let content = r##"
<h3 class="font-bold text-lg mb-4">添加 Bot</h3>
<form hx-post="/pages/bots/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID (可选，默认使用 App ID)</span></div>
        <input type="text" class="input input-bordered" name="id" placeholder="留空则使用 App ID" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">App ID *</span></div>
        <input type="text" class="input input-bordered" name="app_id" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Base URL *</span></div>
        <input type="text" class="input input-bordered" name="base_url" value="https://www.geweapi.com" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Token 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="token_env" placeholder="GEWE_BOT_TOKEN_XXX" />
        <div class="label"><span class="label-text-alt text-base-content/50">留空则需要在 token 字段直接填写</span></div>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Webhook Secret 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="webhook_secret_env" placeholder="GEWE_WEBHOOK_SECRET_XXX" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Tags (逗号分隔)</span></div>
        <input type="text" class="input input-bordered" name="tags" placeholder="prod, test" />
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##;

    Html(content.to_string())
}

/// AI Profiles 列表页面
pub async fn ai_profiles_list(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let rows: String = config
        .ai_profiles
        .iter()
        .map(|profile| {
            let api_key_display = if profile.api_key_env.is_some() {
                format!("${{{}}}", profile.api_key_env.as_ref().unwrap())
            } else {
                "***".to_string()
            };
            let tools = if profile.tool_ids.is_empty() {
                String::new()
            } else {
                profile.tool_ids.join(", ")
            };

            format!(
                r##"<tr>
                    <td class="font-mono">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="font-mono text-xs">{}</td>
                    <td class="text-xs">{}</td>
                    <td>
                        <div class="flex gap-1">
                            <button class="btn btn-ghost btn-xs"
                                    hx-get="/pages/ai-profiles/edit/{}"
                                    hx-target="#modal-content"
                                    onclick="openModal()">
                                编辑
                            </button>
                            <button class="btn btn-error btn-xs"
                                    hx-post="/pages/ai-profiles/delete/{}"
                                    hx-target="#main"
                                    hx-confirm="确定删除吗？">
                                删除
                            </button>
                        </div>
                    </td>
                </tr>"##,
                profile.id,
                profile.provider.as_deref().unwrap_or("-"),
                profile.model,
                api_key_display,
                tools,
                profile.id,
                profile.id
            )
        })
        .collect();

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">AI Profiles 管理</h1>
    <button class="btn btn-primary btn-sm"
            hx-get="/pages/ai-profiles/new"
            hx-target="#modal-content"
            onclick="openModal()">
        添加 Profile
    </button>
</div>

<div class="card bg-base-100 shadow-sm">
    <div class="card-body">
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Provider</th>
                        <th>Model</th>
                        <th>API Key</th>
                        <th>Tools</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
    </div>
</div>
"##,
        if rows.is_empty() {
            r##"<tr><td colspan="6" class="text-center text-base-content/50">暂无 AI Profile 配置</td></tr>"##.to_string()
        } else {
            rows
        }
    );

    Html(content)
}

/// AI Profile 编辑表单
pub async fn ai_profile_edit_form(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let profile = config.ai_profiles.iter().find(|p| p.id == id);

    // 获取所有可用工具
    let tools_options: String = config
        .tools
        .iter()
        .map(|t| {
            let checked = profile
                .map(|p| p.tool_ids.contains(&t.id))
                .unwrap_or(false);
            format!(
                r##"<label class="label cursor-pointer justify-start gap-2">
                    <input type="checkbox" class="checkbox checkbox-sm" name="tool_ids" value="{}" {} />
                    <span class="label-text">{}</span>
                </label>"##,
                t.id,
                if checked { "checked" } else { "" },
                t.id
            )
        })
        .collect();

    let (title, profile_id, provider, model, base_url, api_key_env, system_prompt_file) =
        match profile {
            Some(p) => (
                format!("编辑 AI Profile: {}", id),
                p.id.clone(),
                p.provider.clone().unwrap_or_default(),
                p.model.clone(),
                p.base_url.clone().unwrap_or_default(),
                p.api_key_env.clone().unwrap_or_default(),
                p.system_prompt_file.clone().unwrap_or_default(),
            ),
            None => (
                "添加 AI Profile".to_string(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ),
        };

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">{}</h3>
<form hx-post="/pages/ai-profiles/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="{}" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID *</span></div>
        <input type="text" class="input input-bordered" name="id" value="{}" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Provider *</span></div>
        <select class="select select-bordered" name="provider" required>
            <option value="openai" {}>OpenAI</option>
            <option value="gemini" {}>Gemini</option>
            <option value="anthropic" {}>Anthropic</option>
            <option value="deepseek" {}>DeepSeek</option>
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Model *</span></div>
        <input type="text" class="input input-bordered" name="model" value="{}" required placeholder="gpt-4, gemini-pro, claude-3..." />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Base URL</span></div>
        <input type="text" class="input input-bordered" name="base_url" value="{}" placeholder="留空使用默认" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">API Key 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="api_key_env" value="{}" placeholder="OPENAI_API_KEY" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">System Prompt 文件</span></div>
        <input type="text" class="input input-bordered" name="system_prompt_file" value="{}" placeholder="prompts/ai_system.txt" />
    </label>

    <div class="form-control">
        <div class="label"><span class="label-text">关联工具</span></div>
        <div class="grid grid-cols-2 gap-1">
            {}
        </div>
    </div>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        title,
        id,
        profile_id,
        if provider == "openai" { "selected" } else { "" },
        if provider == "gemini" { "selected" } else { "" },
        if provider == "anthropic" {
            "selected"
        } else {
            ""
        },
        if provider == "deepseek" {
            "selected"
        } else {
            ""
        },
        model,
        base_url,
        api_key_env,
        system_prompt_file,
        if tools_options.is_empty() {
            r##"<span class="text-base-content/50">暂无工具</span>"##.to_string()
        } else {
            tools_options
        }
    );

    Html(content)
}

/// 新建 AI Profile 表单
pub async fn ai_profile_new_form(State(state): State<ApiState>) -> Html<String> {
    let config = load_config(&state).await.ok();

    let tools_options: String = config
        .map(|c| c.tools)
        .unwrap_or_default()
        .iter()
        .map(|t| {
            format!(
                r##"<label class="label cursor-pointer justify-start gap-2">
                    <input type="checkbox" class="checkbox checkbox-sm" name="tool_ids" value="{}" />
                    <span class="label-text">{}</span>
                </label>"##,
                t.id, t.id
            )
        })
        .collect();

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">添加 AI Profile</h3>
<form hx-post="/pages/ai-profiles/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID *</span></div>
        <input type="text" class="input input-bordered" name="id" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Provider *</span></div>
        <select class="select select-bordered" name="provider" required>
            <option value="openai">OpenAI</option>
            <option value="gemini" selected>Gemini</option>
            <option value="anthropic">Anthropic</option>
            <option value="deepseek">DeepSeek</option>
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Model *</span></div>
        <input type="text" class="input input-bordered" name="model" required placeholder="gpt-4, gemini-pro, claude-3..." />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">Base URL</span></div>
        <input type="text" class="input input-bordered" name="base_url" placeholder="留空使用默认" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">API Key 环境变量名</span></div>
        <input type="text" class="input input-bordered" name="api_key_env" placeholder="OPENAI_API_KEY" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">System Prompt 文件</span></div>
        <input type="text" class="input input-bordered" name="system_prompt_file" placeholder="prompts/ai_system.txt" />
    </label>

    <div class="form-control">
        <div class="label"><span class="label-text">关联工具</span></div>
        <div class="grid grid-cols-2 gap-1">
            {}
        </div>
    </div>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        if tools_options.is_empty() {
            r##"<span class="text-base-content/50">暂无工具</span>"##.to_string()
        } else {
            tools_options
        }
    );

    Html(content)
}

/// Tools 列表页面
pub async fn tools_list(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let rows: String = config
        .tools
        .iter()
        .map(|tool| {
            format!(
                r##"<tr>
                    <td class="font-mono">{}</td>
                    <td>{}</td>
                    <td class="font-mono">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>
                        <div class="flex gap-1">
                            <button class="btn btn-ghost btn-xs"
                                    hx-get="/pages/tools/edit/{}"
                                    hx-target="#modal-content"
                                    onclick="openModal()">
                                编辑
                            </button>
                            <button class="btn btn-error btn-xs"
                                    hx-post="/pages/tools/delete/{}"
                                    hx-target="#main"
                                    hx-confirm="确定删除吗？">
                                删除
                            </button>
                        </div>
                    </td>
                </tr>"##,
                tool.id,
                tool.kind.as_deref().unwrap_or("command"),
                tool.program,
                tool.timeout_secs.unwrap_or(30),
                tool.description.as_deref().unwrap_or("-"),
                tool.id,
                tool.id
            )
        })
        .collect();

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">工具管理</h1>
    <button class="btn btn-primary btn-sm"
            hx-get="/pages/tools/new"
            hx-target="#modal-content"
            onclick="openModal()">
        添加工具
    </button>
</div>

<div class="card bg-base-100 shadow-sm">
    <div class="card-body">
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>类型</th>
                        <th>程序</th>
                        <th>超时(秒)</th>
                        <th>描述</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
    </div>
</div>
"##,
        if rows.is_empty() {
            r##"<tr><td colspan="6" class="text-center text-base-content/50">暂无工具配置</td></tr>"##.to_string()
        } else {
            rows
        }
    );

    Html(content)
}

/// Tool 编辑表单
pub async fn tool_edit_form(Path(id): Path<String>, State(state): State<ApiState>) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    let tool = config.tools.iter().find(|t| t.id == id);

    let (title, tool_id, kind, program, timeout, description, pre_reply) = match tool {
        Some(t) => (
            format!("编辑工具: {}", id),
            t.id.clone(),
            t.kind.clone().unwrap_or_else(|| "command".to_string()),
            t.program.clone(),
            t.timeout_secs.unwrap_or(30),
            t.description.clone().unwrap_or_default(),
            t.pre_reply.clone().unwrap_or_default(),
        ),
        None => (
            "添加工具".to_string(),
            String::new(),
            "command".to_string(),
            String::new(),
            30,
            String::new(),
            String::new(),
        ),
    };

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">{}</h3>
<form hx-post="/pages/tools/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="{}" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID *</span></div>
        <input type="text" class="input input-bordered" name="id" value="{}" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">类型 *</span></div>
        <select class="select select-bordered" name="kind" required>
            <option value="command" {}>command (内置命令)</option>
            <option value="http" {}>http (HTTP 请求)</option>
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">程序/路径 *</span></div>
        <input type="text" class="input input-bordered" name="program" value="{}" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">超时时间 (秒)</span></div>
        <input type="number" class="input input-bordered" name="timeout_secs" value="{}" min="1" max="600" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">描述</span></div>
        <input type="text" class="input input-bordered" name="description" value="{}" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">预回复 (执行前发送给用户)</span></div>
        <input type="text" class="input input-bordered" name="pre_reply" value="{}" placeholder="例如：正在处理请求..." />
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        title,
        id,
        tool_id,
        if kind == "command" { "selected" } else { "" },
        if kind == "http" { "selected" } else { "" },
        program,
        timeout,
        description,
        pre_reply
    );

    Html(content)
}

/// 新建 Tool 表单
pub async fn tool_new_form() -> Html<String> {
    let content = r##"
<h3 class="font-bold text-lg mb-4">添加工具</h3>
<form hx-post="/pages/tools/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="" />

    <label class="form-control w-full">
        <div class="label"><span class="label-text">ID *</span></div>
        <input type="text" class="input input-bordered" name="id" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">类型 *</span></div>
        <select class="select select-bordered" name="kind" required>
            <option value="command" selected>command (内置命令)</option>
            <option value="http">http (HTTP 请求)</option>
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">程序/路径 *</span></div>
        <input type="text" class="input input-bordered" name="program" required />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">超时时间 (秒)</span></div>
        <input type="number" class="input input-bordered" name="timeout_secs" value="30" min="1" max="600" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">描述</span></div>
        <input type="text" class="input input-bordered" name="description" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">预回复 (执行前发送给用户)</span></div>
        <input type="text" class="input input-bordered" name="pre_reply" placeholder="例如：正在处理请求..." />
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##;

    Html(content.to_string())
}

/// Rules 页面 (包含模板和实例)
pub async fn rules_page(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => {
            return Html(format!(
                r##"<div class="alert alert-error"><span>{}</span></div>"##,
                e
            ))
        }
    };

    // 规则模板列表
    let template_rows: String = config
        .rule_templates
        .iter()
        .map(|t| {
            let kind = t
                .kind
                .as_ref()
                .map(|k| format!("{:?}", k).to_lowercase())
                .unwrap_or_else(|| "any".to_string());
            let action = {
                let mut parts = Vec::new();
                if let Some(ref profile) = t.action.ai_profile {
                    parts.push(format!("ai({})", profile));
                }
                if t.action.log.unwrap_or(false) {
                    parts.push("log".to_string());
                }
                if parts.is_empty() {
                    "-".to_string()
                } else {
                    parts.join(", ")
                }
            };

            format!(
                r##"<tr>
                    <td class="font-mono">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="text-xs">{}</td>
                    <td>
                        <div class="flex gap-1">
                            <button class="btn btn-ghost btn-xs"
                                    hx-get="/pages/rule-templates/edit/{}"
                                    hx-target="#modal-content"
                                    onclick="openModal()">
                                编辑
                            </button>
                            <button class="btn btn-error btn-xs"
                                    hx-post="/pages/rule-templates/delete/{}"
                                    hx-target="#main"
                                    hx-confirm="确定删除吗？">
                                删除
                            </button>
                        </div>
                    </td>
                </tr>"##,
                t.id,
                t.name.as_deref().unwrap_or("-"),
                kind,
                action,
                t.id,
                t.id
            )
        })
        .collect();

    // 规则实例列表
    let instance_rows: String = config
        .rule_instances
        .iter()
        .map(|i| {
            let channel = i.channel.as_deref().unwrap_or("both");
            let priority = i.priority.unwrap_or(100);

            format!(
                r##"<tr>
                    <td class="font-mono">{}</td>
                    <td class="font-mono">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>
                        <div class="flex gap-1">
                            <button class="btn btn-ghost btn-xs"
                                    hx-get="/pages/rule-instances/edit/{}"
                                    hx-target="#modal-content"
                                    onclick="openModal()">
                                编辑
                            </button>
                            <button class="btn btn-error btn-xs"
                                    hx-post="/pages/rule-instances/delete/{}"
                                    hx-target="#main"
                                    hx-confirm="确定删除吗？">
                                删除
                            </button>
                        </div>
                    </td>
                </tr>"##,
                i.id, i.template, channel, priority, i.id, i.id
            )
        })
        .collect();

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">规则管理</h1>
</div>

<div role="tablist" class="tabs tabs-box mb-4">
    <input type="radio" name="rules_tabs" role="tab" class="tab" aria-label="规则模板" checked />
    <div role="tabpanel" class="tab-content bg-base-100 p-4 rounded-box">
        <div class="flex justify-end mb-2">
            <button class="btn btn-primary btn-sm"
                    hx-get="/pages/rule-templates/new"
                    hx-target="#modal-content"
                    onclick="openModal()">
                添加模板
            </button>
        </div>
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>名称</th>
                        <th>类型</th>
                        <th>动作</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
    </div>

    <input type="radio" name="rules_tabs" role="tab" class="tab" aria-label="规则实例" />
    <div role="tabpanel" class="tab-content bg-base-100 p-4 rounded-box">
        <div class="flex justify-end mb-2">
            <button class="btn btn-primary btn-sm"
                    hx-get="/pages/rule-instances/new"
                    hx-target="#modal-content"
                    onclick="openModal()">
                添加实例
            </button>
        </div>
        <div class="overflow-x-auto">
            <table class="table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>模板</th>
                        <th>频道</th>
                        <th>优先级</th>
                        <th>操作</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
    </div>
</div>
"##,
        if template_rows.is_empty() {
            r##"<tr><td colspan="5" class="text-center text-base-content/50">暂无规则模板</td></tr>"##.to_string()
        } else {
            template_rows
        },
        if instance_rows.is_empty() {
            r##"<tr><td colspan="5" class="text-center text-base-content/50">暂无规则实例</td></tr>"##.to_string()
        } else {
            instance_rows
        }
    );

    Html(content)
}

/// Prompts 页面
pub async fn prompts_page(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    // 获取 prompts 目录下的文件列表
    let prompts_dir = state.prompts_dir();
    let mut files = Vec::new();

    if let Ok(mut entries) = tokio::fs::read_dir(prompts_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".txt") || filename.ends_with(".md") {
                if let Ok(metadata) = entry.metadata().await {
                    files.push((filename, metadata.len()));
                }
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let file_rows: String = files
        .iter()
        .map(|(name, size)| {
            format!(
                r##"<tr class="hover cursor-pointer"
                       hx-get="/pages/prompts/edit/{}"
                       hx-target="#prompt-editor">
                    <td class="font-mono">{}</td>
                    <td>{} bytes</td>
                </tr>"##,
                name, name, size
            )
        })
        .collect();

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">Prompts 管理</h1>
    <button class="btn btn-primary btn-sm"
            hx-get="/pages/prompts/new"
            hx-target="#prompt-editor">
        新建 Prompt
    </button>
</div>

<div class="grid md:grid-cols-3 gap-4">
    <!-- 文件列表 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm">文件列表</h2>
            <div class="overflow-x-auto">
                <table class="table table-sm">
                    <thead>
                        <tr>
                            <th>文件名</th>
                            <th>大小</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    <!-- 编辑器 -->
    <div class="card bg-base-100 shadow-sm md:col-span-2" id="prompt-editor">
        <div class="card-body">
            <p class="text-base-content/50 text-center">选择左侧文件进行编辑</p>
        </div>
    </div>
</div>
"##,
        if file_rows.is_empty() {
            r##"<tr><td colspan="2" class="text-center text-base-content/50">暂无 Prompt 文件</td></tr>"##.to_string()
        } else {
            file_rows
        }
    );

    Html(content)
}

/// Prompt 编辑器
pub async fn prompt_edit(Path(name): Path<String>, State(state): State<ApiState>) -> Html<String> {
    let file_path = state.prompts_dir().join(&name);
    let content = tokio::fs::read_to_string(&file_path)
        .await
        .unwrap_or_default();

    let escaped_content = content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");

    let html = format!(
        r##"
<div class="card-body">
    <div class="flex justify-between items-center">
        <h2 class="card-title">{}</h2>
        <div class="flex gap-2">
            <button class="btn btn-error btn-sm"
                    hx-delete="/api/prompts/{}"
                    hx-confirm="确定删除 {}？"
                    hx-target="#main"
                    hx-swap="innerHTML">
                删除
            </button>
        </div>
    </div>
    <form hx-put="/api/prompts/{}" hx-ext="json-enc" class="space-y-4">
        <textarea name="content" class="textarea textarea-bordered w-full h-96 font-mono text-sm">{}</textarea>
        <div class="flex justify-end">
            <button type="submit" class="btn btn-primary">保存</button>
        </div>
    </form>
</div>
"##,
        name, name, name, name, escaped_content
    );

    Html(html)
}

/// 新建 Prompt 编辑器
pub async fn prompt_new() -> Html<String> {
    let html = r##"
<div class="card-body">
    <h2 class="card-title">新建 Prompt</h2>
    <form hx-post="/pages/prompts/create" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
        <label class="form-control w-full">
            <div class="label"><span class="label-text">文件名 *</span></div>
            <input type="text" class="input input-bordered" name="name" required placeholder="example.txt" />
            <div class="label"><span class="label-text-alt">必须以 .txt 或 .md 结尾</span></div>
        </label>
        <textarea name="content" class="textarea textarea-bordered w-full h-64 font-mono text-sm" placeholder="输入 prompt 内容..."></textarea>
        <div class="flex justify-end">
            <button type="submit" class="btn btn-primary">创建</button>
        </div>
    </form>
</div>
"##;

    Html(html.to_string())
}

/// Simulator 页面
pub async fn simulator_page(HxRequest(is_htmx): HxRequest) -> Response {
    if let Some(redirect) = require_htmx(is_htmx) {
        return redirect;
    }

    let content = r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">规则模拟器</h1>
</div>

<div class="grid md:grid-cols-2 gap-4">
    <!-- 模拟表单 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm">模拟参数</h2>
            <form hx-post="/api/config/simulate"
                  hx-ext="json-enc"
                  hx-target="#simulation-result"
                  class="space-y-4">

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">App ID *</span></div>
                    <input type="text" class="input input-bordered" name="app_id" required placeholder="wx_xxx" />
                </label>

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">消息类型 *</span></div>
                    <select class="select select-bordered" name="msg_kind" required>
                        <option value="text" selected>text (文本)</option>
                        <option value="image">image (图片)</option>
                        <option value="voice">voice (语音)</option>
                        <option value="video">video (视频)</option>
                        <option value="emoji">emoji (表情)</option>
                        <option value="link">link (链接)</option>
                        <option value="file_notice">file_notice (文件)</option>
                    </select>
                </label>

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">频道 *</span></div>
                    <select class="select select-bordered" name="chat" required>
                        <option value="private" selected>private (私聊)</option>
                        <option value="group">group (群聊)</option>
                    </select>
                </label>

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">消息内容 *</span></div>
                    <textarea class="textarea textarea-bordered" name="content" required placeholder="输入模拟的消息内容"></textarea>
                </label>

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">发送者 wxid (可选)</span></div>
                    <input type="text" class="input input-bordered" name="from_wxid" placeholder="wxid_xxx" />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <input type="checkbox" class="checkbox" name="mentioned" />
                    <span class="label-text">被 @ 了机器人</span>
                </label>

                <button type="submit" class="btn btn-primary w-full">模拟匹配</button>
            </form>
        </div>
    </div>

    <!-- 结果展示 -->
    <div class="card bg-base-100 shadow-sm" id="simulation-result">
        <div class="card-body">
            <h2 class="card-title text-sm">匹配结果</h2>
            <p class="text-base-content/50">填写参数并点击"模拟匹配"查看结果</p>
        </div>
    </div>
</div>
"##;

    Html(content.to_string()).into_response()
}

// ============================================================================
// 表单数据结构
// ============================================================================

/// Bot 表单数据
#[derive(Debug, Deserialize)]
pub struct BotFormData {
    pub original_id: String,
    pub id: Option<String>,
    pub app_id: String,
    pub base_url: String,
    pub token_env: Option<String>,
    pub webhook_secret_env: Option<String>,
    pub tags: Option<String>,
}

/// AI Profile 表单数据
#[derive(Debug, Deserialize)]
pub struct AiProfileFormData {
    pub original_id: String,
    pub id: String,
    pub provider: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key_env: Option<String>,
    pub system_prompt_file: Option<String>,
    #[serde(default)]
    pub tool_ids: Vec<String>,
}

/// Tool 表单数据
#[derive(Debug, Deserialize)]
pub struct ToolFormData {
    pub original_id: String,
    pub id: String,
    pub kind: String,
    pub program: String,
    pub timeout_secs: Option<u64>,
    pub description: Option<String>,
    pub pre_reply: Option<String>,
}

/// 规则模板表单数据
#[derive(Debug, Deserialize)]
pub struct RuleTemplateFormData {
    pub original_id: String,
    pub id: String,
    pub name: Option<String>,
    pub kind: String,
    pub match_any: Option<String>,
    pub match_equals: Option<String>,
    pub match_contains: Option<String>,
    pub match_regex: Option<String>,
    pub ai_profile: Option<String>,
    pub reply_mode: Option<String>,
    pub log: Option<String>,
    pub require_mention: Option<String>,
}

/// 规则实例表单数据
#[derive(Debug, Deserialize)]
pub struct RuleInstanceFormData {
    pub original_id: String,
    pub id: String,
    pub template: String,
    pub channel: String,
    pub priority: Option<i32>,
    pub from_wxid: Option<String>,
    pub ai_profile: Option<String>,
    pub require_mention: Option<String>,
}

/// Prompt 创建表单数据
#[derive(Debug, Deserialize)]
pub struct PromptFormData {
    pub name: String,
    pub content: String,
}

// ============================================================================
// 表单处理端点
// ============================================================================

/// 保存 Bot
pub async fn bot_save(
    State(state): State<ApiState>,
    Form(form): Form<BotFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 解析 tags
    let tags: Vec<String> = form
        .tags
        .as_ref()
        .map(|s| {
            s.split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let new_bot = BotConfigV2 {
        id: form.id.filter(|s| !s.is_empty()),
        app_id: form.app_id.clone(),
        token: None,
        token_env: form.token_env.filter(|s| !s.is_empty()),
        base_url: form.base_url.clone(),
        webhook_secret: None,
        webhook_secret_env: form.webhook_secret_env.filter(|s| !s.is_empty()),
        tags,
    };

    // 查找并更新或添加
    if !form.original_id.is_empty() {
        if let Some(pos) = config
            .bots
            .iter()
            .position(|b| b.id.as_deref().unwrap_or(&b.app_id) == form.original_id)
        {
            config.bots[pos] = new_bot;
        } else {
            config.bots.push(new_bot);
        }
    } else {
        config.bots.push(new_bot);
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("Bot 已保存", "/pages/bots")
}

/// 保存 AI Profile
pub async fn ai_profile_save(
    State(state): State<ApiState>,
    Form(form): Form<AiProfileFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    let new_profile = AiProfileV2 {
        id: form.id.clone(),
        provider: if form.provider.is_empty() {
            None
        } else {
            Some(form.provider.clone())
        },
        model: form.model.clone(),
        base_url: form.base_url.filter(|s| !s.is_empty()),
        api_key: None,
        api_key_env: form.api_key_env.filter(|s| !s.is_empty()),
        system_prompt: None,
        system_prompt_file: form.system_prompt_file.filter(|s| !s.is_empty()),
        user_prefix: None,
        tool_ids: form.tool_ids,
    };

    // 查找并更新或添加
    if !form.original_id.is_empty() {
        if let Some(pos) = config
            .ai_profiles
            .iter()
            .position(|p| p.id == form.original_id)
        {
            config.ai_profiles[pos] = new_profile;
        } else {
            config.ai_profiles.push(new_profile);
        }
    } else {
        config.ai_profiles.push(new_profile);
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("AI Profile 已保存", "/pages/ai-profiles")
}

/// 保存 Tool
pub async fn tool_save(
    State(state): State<ApiState>,
    Form(form): Form<ToolFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    let new_tool = ToolConfigV2 {
        id: form.id.clone(),
        kind: if form.kind.is_empty() {
            None
        } else {
            Some(form.kind.clone())
        },
        program: form.program.clone(),
        args: Vec::new(),
        timeout_secs: form.timeout_secs,
        max_output: None,
        pre_reply: form.pre_reply.filter(|s| !s.is_empty()),
        post_reply: None,
        description: form.description.filter(|s| !s.is_empty()),
        parameters: None,
    };

    // 查找并更新或添加
    if !form.original_id.is_empty() {
        if let Some(pos) = config.tools.iter().position(|t| t.id == form.original_id) {
            config.tools[pos] = new_tool;
        } else {
            config.tools.push(new_tool);
        }
    } else {
        config.tools.push(new_tool);
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("工具已保存", "/pages/tools")
}

/// 规则模板编辑表单
pub async fn rule_template_edit_form(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    let template = config.rule_templates.iter().find(|t| t.id == id);

    let (
        title,
        tmpl_id,
        name,
        kind,
        match_any,
        match_equals,
        match_contains,
        match_regex,
        ai_profile,
        reply_mode,
        log,
        require_mention,
    ) = match template {
        Some(t) => {
            let match_cfg = &t.r#match;
            (
                format!("编辑规则模板: {}", id),
                t.id.clone(),
                t.name.clone().unwrap_or_default(),
                t.kind
                    .as_ref()
                    .map(|k| format!("{:?}", k).to_lowercase())
                    .unwrap_or_else(|| "any".to_string()),
                match_cfg.any.unwrap_or(false),
                match_cfg.equals.clone().unwrap_or_default(),
                match_cfg.contains.clone().unwrap_or_default(),
                match_cfg.regex.clone().unwrap_or_default(),
                t.action.ai_profile.clone().unwrap_or_default(),
                t.action
                    .reply_mode
                    .as_ref()
                    .map(|r| format!("{:?}", r).to_lowercase())
                    .unwrap_or_else(|| "none".to_string()),
                t.action.log.unwrap_or(false),
                t.action.require_mention.unwrap_or(false),
            )
        }
        None => (
            "添加规则模板".to_string(),
            String::new(),
            String::new(),
            "any".to_string(),
            true,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "none".to_string(),
            false,
            false,
        ),
    };

    // 获取 AI Profiles 列表供选择
    let profile_options: String = config
        .ai_profiles
        .iter()
        .map(|p| {
            format!(
                r##"<option value="{}" {}>{}</option>"##,
                p.id,
                if p.id == ai_profile { "selected" } else { "" },
                p.id
            )
        })
        .collect();

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">{}</h3>
<form hx-post="/pages/rule-templates/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="{}" />

    <div class="grid md:grid-cols-2 gap-4">
        <label class="form-control w-full">
            <div class="label"><span class="label-text">ID *</span></div>
            <input type="text" class="input input-bordered" name="id" value="{}" required />
        </label>

        <label class="form-control w-full">
            <div class="label"><span class="label-text">名称</span></div>
            <input type="text" class="input input-bordered" name="name" value="{}" />
        </label>
    </div>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">规则类型</span></div>
        <select class="select select-bordered" name="kind">
            <option value="any" {}>any (任意)</option>
            <option value="text" {}>text (文本)</option>
            <option value="image" {}>image (图片)</option>
            <option value="voice" {}>voice (语音)</option>
            <option value="video" {}>video (视频)</option>
        </select>
    </label>

    <div class="divider">匹配条件</div>

    <label class="label cursor-pointer justify-start gap-2">
        <input type="checkbox" class="checkbox" name="match_any" value="true" {} />
        <span class="label-text">匹配任意消息</span>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">精确匹配</span></div>
        <input type="text" class="input input-bordered" name="match_equals" value="{}" placeholder="完全匹配的文本" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">包含匹配</span></div>
        <input type="text" class="input input-bordered" name="match_contains" value="{}" />
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">正则匹配</span></div>
        <input type="text" class="input input-bordered" name="match_regex" value="{}" />
    </label>

    <div class="divider">动作配置</div>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">AI Profile</span></div>
        <select class="select select-bordered" name="ai_profile">
            <option value="">不使用 AI</option>
            {}
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">回复模式</span></div>
        <select class="select select-bordered" name="reply_mode">
            <option value="none" {}>none (不回复)</option>
            <option value="quote" {}>quote (引用回复)</option>
            <option value="at" {}>at (@回复)</option>
            <option value="quoteandат" {}>quoteandат (引用+@)</option>
        </select>
    </label>

    <label class="label cursor-pointer justify-start gap-2">
        <input type="checkbox" class="checkbox" name="log" value="true" {} />
        <span class="label-text">记录日志</span>
    </label>

    <label class="label cursor-pointer justify-start gap-2">
        <input type="checkbox" class="checkbox" name="require_mention" value="true" {} />
        <span class="label-text">需要 @ 机器人</span>
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        title,
        id,
        tmpl_id,
        name,
        if kind == "any" { "selected" } else { "" },
        if kind == "text" { "selected" } else { "" },
        if kind == "image" { "selected" } else { "" },
        if kind == "voice" { "selected" } else { "" },
        if kind == "video" { "selected" } else { "" },
        if match_any { "checked" } else { "" },
        match_equals,
        match_contains,
        match_regex,
        profile_options,
        if reply_mode == "none" { "selected" } else { "" },
        if reply_mode == "quote" {
            "selected"
        } else {
            ""
        },
        if reply_mode == "at" { "selected" } else { "" },
        if reply_mode == "quoteandат" {
            "selected"
        } else {
            ""
        },
        if log { "checked" } else { "" },
        if require_mention { "checked" } else { "" },
    );

    Html(content)
}

/// 新建规则模板表单
pub async fn rule_template_new_form(State(state): State<ApiState>) -> Html<String> {
    rule_template_edit_form(Path(String::new()), State(state)).await
}

/// 保存规则模板
pub async fn rule_template_save(
    State(state): State<ApiState>,
    Form(form): Form<RuleTemplateFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    let kind = match form.kind.as_str() {
        "text" => Some(RuleKind::Text),
        "image" => Some(RuleKind::Image),
        "voice" => Some(RuleKind::Voice),
        "video" => Some(RuleKind::Video),
        _ => None,
    };

    let match_config = MatchConfigV2 {
        any: form.match_any.as_ref().map(|_| true),
        equals: form.match_equals.filter(|s| !s.is_empty()),
        contains: form.match_contains.filter(|s| !s.is_empty()),
        regex: form.match_regex.filter(|s| !s.is_empty()),
    };

    let reply_mode = match form.reply_mode.as_deref() {
        Some("quote") => Some(crate::config::ReplyMode::Quote),
        Some("at") => Some(crate::config::ReplyMode::At),
        Some("quoteandат") => Some(crate::config::ReplyMode::QuoteAndAt),
        _ => Some(crate::config::ReplyMode::None),
    };

    let action = TemplateActionV2 {
        ai_profile: form.ai_profile.filter(|s| !s.is_empty()),
        reply_mode,
        log: form.log.as_ref().map(|_| true),
        require_mention: form.require_mention.as_ref().map(|_| true),
        reply_text: None,
    };

    let defaults = TemplateDefaultsV2 {
        require_mention: None,
    };

    let new_template = RuleTemplateV2 {
        id: form.id.clone(),
        name: form.name.filter(|s| !s.is_empty()),
        kind,
        r#match: match_config,
        action,
        defaults,
    };

    // 查找并更新或添加
    if !form.original_id.is_empty() {
        if let Some(pos) = config
            .rule_templates
            .iter()
            .position(|t| t.id == form.original_id)
        {
            config.rule_templates[pos] = new_template;
        } else {
            config.rule_templates.push(new_template);
        }
    } else {
        config.rule_templates.push(new_template);
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("规则模板已保存", "/pages/rules")
}

/// 规则实例编辑表单
pub async fn rule_instance_edit_form(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    let instance = config.rule_instances.iter().find(|i| i.id == id);

    let (title, inst_id, template, channel, priority, from_wxid, ai_profile, require_mention) =
        match instance {
            Some(i) => {
                let from_wxid = i.from.wxid.clone().unwrap_or_default();
                (
                    format!("编辑规则实例: {}", id),
                    i.id.clone(),
                    i.template.clone(),
                    i.channel.clone().unwrap_or_else(|| "both".to_string()),
                    i.priority.unwrap_or(100),
                    from_wxid,
                    i.overrides
                        .as_ref()
                        .and_then(|o| o.ai_profile.clone())
                        .unwrap_or_default(),
                    i.overrides
                        .as_ref()
                        .and_then(|o| o.require_mention)
                        .unwrap_or(false),
                )
            }
            None => (
                "添加规则实例".to_string(),
                String::new(),
                String::new(),
                "both".to_string(),
                100,
                String::new(),
                String::new(),
                false,
            ),
        };

    // 获取模板列表和 AI Profiles 列表
    let template_options: String = config
        .rule_templates
        .iter()
        .map(|t| {
            format!(
                r##"<option value="{}" {}>{} - {}</option>"##,
                t.id,
                if t.id == template { "selected" } else { "" },
                t.id,
                t.name.as_deref().unwrap_or("")
            )
        })
        .collect();

    let profile_options: String = config
        .ai_profiles
        .iter()
        .map(|p| {
            format!(
                r##"<option value="{}" {}>{}</option>"##,
                p.id,
                if p.id == ai_profile { "selected" } else { "" },
                p.id
            )
        })
        .collect();

    let content = format!(
        r##"
<h3 class="font-bold text-lg mb-4">{}</h3>
<form hx-post="/pages/rule-instances/save" hx-target="#main" hx-swap="innerHTML" class="space-y-4">
    <input type="hidden" name="original_id" value="{}" />

    <div class="grid md:grid-cols-2 gap-4">
        <label class="form-control w-full">
            <div class="label"><span class="label-text">ID *</span></div>
            <input type="text" class="input input-bordered" name="id" value="{}" required />
        </label>

        <label class="form-control w-full">
            <div class="label"><span class="label-text">优先级</span></div>
            <input type="number" class="input input-bordered" name="priority" value="{}" />
            <div class="label"><span class="label-text-alt">数字越小优先级越高</span></div>
        </label>
    </div>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">规则模板 *</span></div>
        <select class="select select-bordered" name="template" required>
            <option value="">选择模板</option>
            {}
        </select>
    </label>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">频道</span></div>
        <select class="select select-bordered" name="channel">
            <option value="both" {}>both (全部)</option>
            <option value="private" {}>private (私聊)</option>
            <option value="group" {}>group (群聊)</option>
        </select>
    </label>

    <div class="divider">过滤条件</div>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">发送者 wxid</span></div>
        <input type="text" class="input input-bordered" name="from_wxid" value="{}" placeholder="留空匹配所有人" />
    </label>

    <div class="divider">覆盖配置</div>

    <label class="form-control w-full">
        <div class="label"><span class="label-text">覆盖 AI Profile</span></div>
        <select class="select select-bordered" name="ai_profile">
            <option value="">使用模板默认</option>
            {}
        </select>
    </label>

    <label class="label cursor-pointer justify-start gap-2">
        <input type="checkbox" class="checkbox" name="require_mention" value="true" {} />
        <span class="label-text">需要 @ 机器人</span>
    </label>

    <div class="modal-action">
        <button type="button" class="btn" onclick="closeModal()">取消</button>
        <button type="submit" class="btn btn-primary" onclick="closeModal()">保存</button>
    </div>
</form>
"##,
        title,
        id,
        inst_id,
        priority,
        template_options,
        if channel == "both" { "selected" } else { "" },
        if channel == "private" { "selected" } else { "" },
        if channel == "group" { "selected" } else { "" },
        from_wxid,
        profile_options,
        if require_mention { "checked" } else { "" },
    );

    Html(content)
}

/// 新建规则实例表单
pub async fn rule_instance_new_form(State(state): State<ApiState>) -> Html<String> {
    rule_instance_edit_form(Path(String::new()), State(state)).await
}

/// 保存规则实例
pub async fn rule_instance_save(
    State(state): State<ApiState>,
    Form(form): Form<RuleInstanceFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 构建 from 过滤
    let from = crate::config::FromConfig {
        nick: None,
        wxid: form.from_wxid.filter(|s| !s.is_empty()),
    };

    // 构建 overrides
    let overrides = if form
        .ai_profile
        .as_ref()
        .map(|s| !s.is_empty())
        .unwrap_or(false)
        || form.require_mention.is_some()
    {
        Some(InstanceOverridesV2 {
            ai_profile: form.ai_profile.filter(|s| !s.is_empty()),
            require_mention: form.require_mention.as_ref().map(|_| true),
            reply_mode: None,
            log: None,
            reply_text: None,
        })
    } else {
        None
    };

    let new_instance = RuleInstanceV2 {
        id: form.id.clone(),
        template: form.template.clone(),
        channel: Some(form.channel.clone()),
        from,
        priority: form.priority,
        overrides,
        enabled: None,
    };

    // 查找并更新或添加
    if !form.original_id.is_empty() {
        if let Some(pos) = config
            .rule_instances
            .iter()
            .position(|i| i.id == form.original_id)
        {
            config.rule_instances[pos] = new_instance;
        } else {
            config.rule_instances.push(new_instance);
        }
    } else {
        config.rule_instances.push(new_instance);
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("规则实例已保存", "/pages/rules")
}

/// 创建 Prompt 文件
pub async fn prompt_create(
    State(state): State<ApiState>,
    Form(form): Form<PromptFormData>,
) -> Html<String> {
    // 验证文件名
    if !form.name.ends_with(".txt") && !form.name.ends_with(".md") {
        return error_html("文件名必须以 .txt 或 .md 结尾");
    }

    if form.name.contains('/') || form.name.contains('\\') || form.name.contains("..") {
        return error_html("文件名不能包含路径");
    }

    let file_path = state.prompts_dir().join(&form.name);

    // 检查文件是否已存在
    if file_path.exists() {
        return error_html(&format!("文件 {} 已存在", form.name));
    }

    // 确保目录存在
    if let Err(e) = tokio::fs::create_dir_all(state.prompts_dir()).await {
        return error_html(&format!("创建目录失败: {}", e));
    }

    // 写入文件
    if let Err(e) = tokio::fs::write(&file_path, &form.content).await {
        return error_html(&format!("写入文件失败: {}", e));
    }

    success_redirect_html(&format!("Prompt {} 已创建", form.name), "/pages/prompts")
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 生成错误 HTML
fn error_html(message: &str) -> Html<String> {
    Html(format!(
        r##"<div class="alert alert-error mb-4">
            <span>{}</span>
        </div>
        <button class="btn" onclick="history.back()">返回</button>"##,
        message
    ))
}

/// 生成成功消息并跳转的 HTML
fn success_redirect_html(message: &str, redirect_url: &str) -> Html<String> {
    Html(format!(
        r##"<div class="alert alert-success mb-4">
            <span>{}</span>
        </div>
        <script>
            setTimeout(function() {{
                htmx.ajax('GET', '{}', {{target: '#main', swap: 'innerHTML'}});
            }}, 500);
        </script>"##,
        message, redirect_url
    ))
}

// ============================================================================
// 全局设置
// ============================================================================

/// 全局设置表单数据
#[derive(Debug, Deserialize)]
pub struct SettingsFormData {
    pub listen_addr: String,
    pub queue_size: usize,
    pub image_dir: String,
    pub image_url_prefix: String,
    pub external_base_url: Option<String>,
    pub reply_mode: Option<String>,
    pub log: Option<String>,
    pub default_ai_profile: Option<String>,
    pub default_require_mention: Option<String>,
}

/// 全局设置页面
pub async fn settings_page(
    State(state): State<ApiState>,
    HxRequest(_is_htmx): HxRequest,
) -> Html<String> {
    let config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 获取所有 AI Profiles 供选择
    let profile_options: String = config
        .ai_profiles
        .iter()
        .map(|p| {
            let selected = config
                .defaults
                .ai
                .as_ref()
                .and_then(|ai| ai.profile.as_ref())
                .map(|prof| prof == &p.id)
                .unwrap_or(false);
            format!(
                r##"<option value="{}" {}>{}</option>"##,
                p.id,
                if selected { "selected" } else { "" },
                p.id
            )
        })
        .collect();

    let reply_mode = config
        .defaults
        .reply_mode
        .as_ref()
        .map(|r| format!("{:?}", r).to_lowercase())
        .unwrap_or_else(|| "none".to_string());

    let content = format!(
        r##"
<div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">全局设置</h1>
</div>

<form hx-post="/pages/settings/save" hx-target="#main" hx-swap="innerHTML" class="space-y-6">

    <!-- 服务器配置 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-lg">服务器配置</h2>
            <div class="grid md:grid-cols-2 gap-4">
                <label class="form-control w-full">
                    <div class="label"><span class="label-text">监听地址 *</span></div>
                    <input type="text" class="input input-bordered" name="listen_addr" value="{}" required />
                    <div class="label"><span class="label-text-alt">例如: 0.0.0.0:4399</span></div>
                </label>

                <label class="form-control w-full">
                    <div class="label"><span class="label-text">队列容量 *</span></div>
                    <input type="number" class="input input-bordered" name="queue_size" value="{}" required min="1" />
                    <div class="label"><span class="label-text-alt">Webhook 事件队列大小</span></div>
                </label>
            </div>
        </div>
    </div>

    <!-- 存储配置 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-lg">存储配置</h2>
            <label class="form-control w-full">
                <div class="label"><span class="label-text">图片目录 *</span></div>
                <input type="text" class="input input-bordered" name="image_dir" value="{}" required />
                <div class="label"><span class="label-text-alt">本地存储路径</span></div>
            </label>

            <label class="form-control w-full">
                <div class="label"><span class="label-text">图片 URL 前缀 *</span></div>
                <input type="text" class="input input-bordered" name="image_url_prefix" value="{}" required />
                <div class="label"><span class="label-text-alt">内部访问路径，例如: /images</span></div>
            </label>

            <label class="form-control w-full">
                <div class="label"><span class="label-text">外部访问 Base URL</span></div>
                <input type="text" class="input input-bordered" name="external_base_url" value="{}" placeholder="http://your-domain.com" />
                <div class="label"><span class="label-text-alt">对外暴露的完整 URL 前缀</span></div>
            </label>
        </div>
    </div>

    <!-- 默认配置 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-lg">默认配置</h2>
            <label class="form-control w-full">
                <div class="label"><span class="label-text">回复模式</span></div>
                <select class="select select-bordered" name="reply_mode">
                    <option value="none" {}>none (不回复)</option>
                    <option value="quote" {}>quote (引用回复)</option>
                    <option value="at" {}>at (@回复)</option>
                    <option value="quoteandат" {}>quoteandат (引用+@)</option>
                </select>
            </label>

            <label class="label cursor-pointer justify-start gap-2">
                <input type="checkbox" class="checkbox" name="log" value="true" {} />
                <span class="label-text">默认记录日志</span>
            </label>
        </div>
    </div>

    <!-- AI 默认配置 -->
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-lg">AI 默认配置</h2>
            <label class="form-control w-full">
                <div class="label"><span class="label-text">默认 AI Profile</span></div>
                <select class="select select-bordered" name="default_ai_profile">
                    <option value="">不设置</option>
                    {}
                </select>
            </label>

            <label class="label cursor-pointer justify-start gap-2">
                <input type="checkbox" class="checkbox" name="default_require_mention" value="true" {} />
                <span class="label-text">群聊默认需要 @ 机器人</span>
            </label>
        </div>
    </div>

    <div class="flex justify-end gap-2">
        <button type="submit" class="btn btn-primary">保存设置</button>
    </div>
</form>
"##,
        config.server.listen_addr,
        config.server.queue_size,
        config.storage.image_dir,
        config.storage.image_url_prefix,
        config.storage.external_base_url.unwrap_or_default(),
        if reply_mode == "none" { "selected" } else { "" },
        if reply_mode == "quote" {
            "selected"
        } else {
            ""
        },
        if reply_mode == "at" { "selected" } else { "" },
        if reply_mode == "quoteandат" {
            "selected"
        } else {
            ""
        },
        if config.defaults.log.unwrap_or(false) {
            "checked"
        } else {
            ""
        },
        profile_options,
        if config
            .defaults
            .ai
            .as_ref()
            .and_then(|ai| ai.require_mention)
            .unwrap_or(false)
        {
            "checked"
        } else {
            ""
        },
    );

    Html(content)
}

/// 保存全局设置
pub async fn settings_save(
    State(state): State<ApiState>,
    Form(form): Form<SettingsFormData>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 更新 server 配置
    config.server = ServerConfigV2 {
        listen_addr: form.listen_addr,
        queue_size: form.queue_size,
    };

    // 更新 storage 配置
    config.storage = StorageConfigV2 {
        image_dir: form.image_dir,
        image_url_prefix: form.image_url_prefix,
        external_base_url: form.external_base_url.filter(|s| !s.is_empty()),
    };

    // 更新 defaults 配置
    let reply_mode = match form.reply_mode.as_deref() {
        Some("quote") => Some(crate::config::ReplyMode::Quote),
        Some("at") => Some(crate::config::ReplyMode::At),
        Some("quoteandат") => Some(crate::config::ReplyMode::QuoteAndAt),
        _ => Some(crate::config::ReplyMode::None),
    };

    config.defaults = DefaultsV2 {
        reply_mode,
        log: form.log.as_ref().map(|_| true),
        ai: Some(DefaultsAiV2 {
            profile: form.default_ai_profile.filter(|s| !s.is_empty()),
            require_mention: form.default_require_mention.as_ref().map(|_| true),
        }),
    };

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html("全局设置已保存", "/pages/settings")
}

// ============================================================================
// 删除功能
// ============================================================================

/// 删除 Bot
pub async fn bot_delete(Path(id): Path<String>, State(state): State<ApiState>) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 查找并删除
    if let Some(pos) = config
        .bots
        .iter()
        .position(|b| b.id.as_deref().unwrap_or(&b.app_id) == id)
    {
        config.bots.remove(pos);
    } else {
        return error_html(&format!("未找到 Bot: {}", id));
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html(&format!("Bot {} 已删除", id), "/pages/bots")
}

/// 删除 AI Profile
pub async fn ai_profile_delete(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 查找并删除
    if let Some(pos) = config.ai_profiles.iter().position(|p| p.id == id) {
        config.ai_profiles.remove(pos);
    } else {
        return error_html(&format!("未找到 AI Profile: {}", id));
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html(&format!("AI Profile {} 已删除", id), "/pages/ai-profiles")
}

/// 删除 Tool
pub async fn tool_delete(Path(id): Path<String>, State(state): State<ApiState>) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 查找并删除
    if let Some(pos) = config.tools.iter().position(|t| t.id == id) {
        config.tools.remove(pos);
    } else {
        return error_html(&format!("未找到工具: {}", id));
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html(&format!("工具 {} 已删除", id), "/pages/tools")
}

/// 删除规则模板
pub async fn rule_template_delete(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 查找并删除
    if let Some(pos) = config.rule_templates.iter().position(|t| t.id == id) {
        config.rule_templates.remove(pos);
    } else {
        return error_html(&format!("未找到规则模板: {}", id));
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html(&format!("规则模板 {} 已删除", id), "/pages/rules")
}

/// 删除规则实例
pub async fn rule_instance_delete(
    Path(id): Path<String>,
    State(state): State<ApiState>,
) -> Html<String> {
    let mut config = match load_config(&state).await {
        Ok(c) => c,
        Err(e) => return error_html(&e),
    };

    // 查找并删除
    if let Some(pos) = config.rule_instances.iter().position(|i| i.id == id) {
        config.rule_instances.remove(pos);
    } else {
        return error_html(&format!("未找到规则实例: {}", id));
    }

    // 保存
    if let Err(e) = save_config(&state, &config).await {
        return error_html(&e);
    }

    success_redirect_html(&format!("规则实例 {} 已删除", id), "/pages/rules")
}
