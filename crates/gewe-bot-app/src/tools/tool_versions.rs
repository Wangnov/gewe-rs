//! AI 开发工具版本查询
//!
//! 查询 Claude Code、CodeX、Gemini CLI 的最新版本信息
//! 数据来源：https://mirror.duckcoding.com/api/v1/tools

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time;

const API_URL: &str = "https://mirror.duckcoding.com/api/v1/tools";
const CACHE_TTL: Duration = Duration::from_secs(60); // 1 分钟缓存
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// 查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct VersionQuery {
    /// 工具 ID: all, claude-code, codex, gemini-cli
    #[serde(default)]
    pub tool: Option<String>,
    /// 是否返回详细信息
    #[serde(default)]
    pub detail: Option<bool>,
}

impl VersionQuery {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }

    pub fn tool(&self) -> &str {
        self.tool.as_deref().unwrap_or("all")
    }

    pub fn detail(&self) -> bool {
        self.detail.unwrap_or(false)
    }
}

/// 执行结果
pub struct VersionResult {
    pub content: String,
    pub truncated: bool,
    pub duration: Duration,
    pub error: Option<String>,
    pub timed_out: bool,
}

/// API 响应结构
#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    tools: Vec<ToolInfo>,
    updated_at: String,
    status: String,
}

/// 工具信息
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ToolInfo {
    id: String,
    name: String,
    latest_version: String,
    mirror_version: Option<String>,
    mirror_synced_at: Option<String>,
    is_stale: Option<bool>,
    release_date: Option<String>,
    download_url: Option<String>,
    release_notes_url: Option<String>,
    package_name: Option<String>,
}

/// 缓存
#[derive(Default)]
struct VersionCache {
    fetched_at: Option<Instant>,
    data: Option<Vec<ToolInfo>>,
    updated_at: Option<String>,
}

static CACHE: OnceLock<RwLock<VersionCache>> = OnceLock::new();

/// 执行版本查询
pub async fn run_tool_versions(
    query: VersionQuery,
    timeout_secs: Option<u64>,
    max_output: usize,
) -> VersionResult {
    let timeout = timeout_secs
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TIMEOUT);
    let start = Instant::now();

    match time::timeout(timeout, execute_query(&query)).await {
        Ok(Ok(content)) => {
            let (text, truncated) = clamp_output(content, max_output);
            VersionResult {
                content: text,
                truncated,
                duration: start.elapsed(),
                error: None,
                timed_out: false,
            }
        }
        Ok(Err(err)) => VersionResult {
            content: format!("查询工具版本失败: {}", err),
            truncated: false,
            duration: start.elapsed(),
            error: Some(err.to_string()),
            timed_out: false,
        },
        Err(_) => VersionResult {
            content: "查询工具版本超时".to_string(),
            truncated: false,
            duration: timeout,
            error: Some("timeout".to_string()),
            timed_out: true,
        },
    }
}

/// 执行查询
async fn execute_query(query: &VersionQuery) -> Result<String> {
    let (tools, updated_at) = fetch_and_cache().await?;

    let tool_id = query.tool();
    let detail = query.detail();

    // 规范化工具 ID
    let normalized = normalize_tool_id(tool_id);

    let selected: Vec<&ToolInfo> = if normalized == "all" {
        tools.iter().collect()
    } else {
        tools.iter().filter(|t| t.id == normalized).collect()
    };

    if selected.is_empty() && normalized != "all" {
        return Err(anyhow!(
            "未找到工具: {}，支持: all, claude-code, codex, gemini-cli",
            tool_id
        ));
    }

    Ok(format_output(&selected, detail, &updated_at))
}

/// 规范化工具 ID
fn normalize_tool_id(id: &str) -> &str {
    match id.to_lowercase().as_str() {
        "all" | "" => "all",
        "claude-code" | "claude" | "cc" => "claude-code",
        "codex" | "openai-codex" => "codex",
        "gemini-cli" | "gemini" | "gcli" => "gemini-cli",
        _ => id,
    }
}

/// 获取并缓存数据
async fn fetch_and_cache() -> Result<(Vec<ToolInfo>, String)> {
    let cache = CACHE.get_or_init(|| RwLock::new(VersionCache::default()));
    let now = Instant::now();

    // 检查缓存
    {
        let guard = cache.read().await;
        if let (Some(ref data), Some(ref updated_at)) = (&guard.data, &guard.updated_at) {
            if guard
                .fetched_at
                .map(|t| now.duration_since(t) < CACHE_TTL)
                .unwrap_or(false)
            {
                return Ok((data.clone(), updated_at.clone()));
            }
        }
    }

    // 获取新数据
    let resp = reqwest::get(API_URL)
        .await
        .map_err(|e| anyhow!("请求失败: {e}"))?;

    let api_resp: ApiResponse = resp
        .json()
        .await
        .map_err(|e| anyhow!("解析响应失败: {e}"))?;

    if api_resp.status != "ok" {
        return Err(anyhow!("API 状态异常: {}", api_resp.status));
    }

    let tools = api_resp.tools;
    let updated_at = api_resp.updated_at;

    // 更新缓存
    {
        let mut guard = cache.write().await;
        guard.data = Some(tools.clone());
        guard.updated_at = Some(updated_at.clone());
        guard.fetched_at = Some(Instant::now());
    }

    Ok((tools, updated_at))
}

/// 格式化输出
fn format_output(tools: &[&ToolInfo], detail: bool, updated_at: &str) -> String {
    let mut lines = Vec::new();

    lines.push("AI 开发工具最新版本：".to_string());
    lines.push(String::new());

    for tool in tools {
        if detail {
            lines.push(format!("【{}】", tool.name));
            lines.push(format!("  版本: {}", tool.latest_version));
            if let Some(ref date) = tool.release_date {
                lines.push(format!("  发布: {}", format_date(date)));
            }
            if let Some(ref mirror) = tool.mirror_version {
                let sync_status = if tool.is_stale.unwrap_or(false) {
                    " (待同步)"
                } else {
                    ""
                };
                lines.push(format!("  镜像: {}{}", mirror, sync_status));
            }
            if let Some(ref url) = tool.download_url {
                lines.push(format!("  下载: {}", url));
            }
            if let Some(ref url) = tool.release_notes_url {
                lines.push(format!("  说明: {}", url));
            }
            lines.push(String::new());
        } else {
            lines.push(format!("• {}: v{}", tool.name, tool.latest_version));
        }
    }

    lines.push(format!("数据更新: {}", format_date(updated_at)));

    lines.join("\n")
}

/// 格式化日期时间
fn format_date(iso: &str) -> String {
    // 简化 ISO 日期为更易读的格式
    if let Some(t_pos) = iso.find('T') {
        let date = &iso[..t_pos];
        let time = &iso[t_pos + 1..];
        let time_short = time.split('.').next().unwrap_or(time);
        let time_short = time_short.trim_end_matches('Z');
        format!("{} {}", date, time_short)
    } else {
        iso.to_string()
    }
}

/// 截断输出
fn clamp_output(text: String, max: usize) -> (String, bool) {
    if text.len() <= max {
        return (text, false);
    }
    if max == 0 {
        return (String::new(), true);
    }
    let mut cut = max.min(text.len());
    while cut > 0 && !text.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut truncated = text;
    truncated.truncate(cut);
    (truncated, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query() {
        let q = VersionQuery::from_json(r#"{"tool":"claude-code","detail":true}"#);
        assert_eq!(q.tool(), "claude-code");
        assert!(q.detail());

        let q = VersionQuery::from_json(r#"{}"#);
        assert_eq!(q.tool(), "all");
        assert!(!q.detail());
    }

    #[test]
    fn test_normalize_tool_id() {
        assert_eq!(normalize_tool_id("cc"), "claude-code");
        assert_eq!(normalize_tool_id("claude"), "claude-code");
        assert_eq!(normalize_tool_id("gemini"), "gemini-cli");
        assert_eq!(normalize_tool_id("all"), "all");
    }
}
