//! Claude Code Changelog 工具
//!
//! 支持多种模式查询 Claude Code 的变更日志：
//! - `latest`: 获取最新一个版本（默认）
//! - `recent`: 获取最近 N 个版本
//! - `first`: 获取最早 N 个版本
//! - `version`: 获取指定版本
//! - `range`: 获取版本范围
//! - `list`: 列出所有版本号

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time;

const CLAUDE_CHANGELOG_URL: &str =
    "https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md";
const CACHE_TTL: Duration = Duration::from_secs(300);
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// Changelog 查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChangelogQuery {
    /// 查询模式: latest, recent, first, version, range, list
    #[serde(default)]
    pub mode: Option<String>,
    /// 数量（用于 recent/first 模式）
    #[serde(default)]
    pub count: Option<usize>,
    /// 版本号（用于 version 模式）
    #[serde(default)]
    pub version: Option<String>,
    /// 起始版本（用于 range 模式）
    #[serde(default)]
    pub from: Option<String>,
    /// 结束版本（用于 range 模式）
    #[serde(default)]
    pub to: Option<String>,
}

impl ChangelogQuery {
    /// 从 JSON 字符串解析查询参数
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }

    /// 获取查询模式，默认为 "latest"
    pub fn mode(&self) -> &str {
        self.mode.as_deref().unwrap_or("latest")
    }

    /// 获取数量，默认为 3
    pub fn count(&self) -> usize {
        self.count.unwrap_or(3)
    }
}

/// 执行结果
pub struct ChangelogResult {
    pub content: String,
    pub truncated: bool,
    pub duration: Duration,
    pub error: Option<String>,
    pub timed_out: bool,
}

/// 单个版本的 Changelog
#[derive(Debug, Clone)]
struct ChangelogEntry {
    version: String,
    content: String,
}

/// Changelog 缓存
#[derive(Default)]
struct ChangelogCache {
    fetched_at: Option<Instant>,
    raw_body: Option<String>,
    entries: Option<Vec<ChangelogEntry>>,
}

static CACHE: OnceLock<RwLock<ChangelogCache>> = OnceLock::new();

/// 执行 claude_changelog 查询
pub async fn run_claude_changelog(
    query: ChangelogQuery,
    timeout_secs: Option<u64>,
    max_output: usize,
) -> ChangelogResult {
    let timeout = timeout_secs
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TIMEOUT);
    let start = Instant::now();

    match time::timeout(timeout, execute_query(&query)).await {
        Ok(Ok(content)) => {
            let (text, truncated) = clamp_output(content, max_output);
            ChangelogResult {
                content: text,
                truncated,
                duration: start.elapsed(),
                error: None,
                timed_out: false,
            }
        }
        Ok(Err(err)) => ChangelogResult {
            content: format!("获取 Changelog 失败: {}", err),
            truncated: false,
            duration: start.elapsed(),
            error: Some(err.to_string()),
            timed_out: false,
        },
        Err(_) => ChangelogResult {
            content: "获取 Changelog 超时".to_string(),
            truncated: false,
            duration: timeout,
            error: Some("timeout".to_string()),
            timed_out: true,
        },
    }
}

/// 执行查询
async fn execute_query(query: &ChangelogQuery) -> Result<String> {
    let entries = fetch_and_parse().await?;

    match query.mode() {
        "latest" => entries
            .first()
            .map(|e| e.content.clone())
            .ok_or_else(|| anyhow!("没有找到任何版本")),
        "recent" => {
            let count = query.count().min(entries.len());
            let selected: Vec<_> = entries.iter().take(count).collect();
            if selected.is_empty() {
                Err(anyhow!("没有找到任何版本"))
            } else {
                Ok(format_entries(&selected))
            }
        }
        "first" => {
            let count = query.count().min(entries.len());
            let selected: Vec<_> = entries.iter().rev().take(count).rev().collect();
            if selected.is_empty() {
                Err(anyhow!("没有找到任何版本"))
            } else {
                Ok(format_entries(&selected))
            }
        }
        "version" => {
            let ver = query
                .version
                .as_deref()
                .ok_or_else(|| anyhow!("缺少 version 参数"))?;
            entries
                .iter()
                .find(|e| e.version == ver || e.version.ends_with(ver))
                .map(|e| e.content.clone())
                .ok_or_else(|| anyhow!("未找到版本: {}", ver))
        }
        "range" => {
            let from = query.from.as_deref();
            let to = query.to.as_deref();

            let (start_idx, end_idx) = find_range_indices(&entries, from, to)?;
            let selected: Vec<_> = entries[start_idx..=end_idx].iter().collect();

            if selected.is_empty() {
                Err(anyhow!("指定范围内没有版本"))
            } else {
                Ok(format_entries(&selected))
            }
        }
        "list" => {
            let versions: Vec<_> = entries.iter().map(|e| e.version.as_str()).collect();
            Ok(format!(
                "共 {} 个版本：\n{}",
                versions.len(),
                versions.join(", ")
            ))
        }
        other => Err(anyhow!(
            "未知模式: {}，支持: latest, recent, first, version, range, list",
            other
        )),
    }
}

/// 获取并解析 Changelog
async fn fetch_and_parse() -> Result<Vec<ChangelogEntry>> {
    let cache = CACHE.get_or_init(|| RwLock::new(ChangelogCache::default()));
    let now = Instant::now();

    // 检查缓存
    {
        let guard = cache.read().await;
        if let Some(ref entries) = guard.entries {
            if guard
                .fetched_at
                .map(|t| now.duration_since(t) < CACHE_TTL)
                .unwrap_or(false)
            {
                return Ok(entries.clone());
            }
        }
    }

    // 获取新数据
    let resp = reqwest::get(CLAUDE_CHANGELOG_URL)
        .await
        .map_err(|e| anyhow!("获取 changelog 失败: {e}"))?;
    let text = resp
        .text()
        .await
        .map_err(|e| anyhow!("读取 changelog 失败: {e}"))?;

    let entries = parse_changelog(&text)?;

    // 更新缓存
    {
        let mut guard = cache.write().await;
        guard.raw_body = Some(text);
        guard.entries = Some(entries.clone());
        guard.fetched_at = Some(Instant::now());
    }

    Ok(entries)
}

/// 解析 Changelog 为版本列表
fn parse_changelog(body: &str) -> Result<Vec<ChangelogEntry>> {
    let mut entries = Vec::new();
    let mut lines = body.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("## ") {
            // 提取版本号
            let version = trimmed.trim_start_matches("## ").trim().to_string();

            // 收集该版本的内容
            let mut section = vec![line.to_string()];
            while let Some(next) = lines.peek() {
                if next.trim_start().starts_with("## ") {
                    break;
                }
                section.push(lines.next().unwrap_or_default().to_string());
            }

            let content = section.join("\n").trim().to_string();
            if !content.is_empty() && !version.is_empty() {
                entries.push(ChangelogEntry { version, content });
            }
        }
    }

    if entries.is_empty() {
        Err(anyhow!("未能解析任何 changelog 条目"))
    } else {
        Ok(entries)
    }
}

/// 查找版本范围的索引
fn find_range_indices(
    entries: &[ChangelogEntry],
    from: Option<&str>,
    to: Option<&str>,
) -> Result<(usize, usize)> {
    // 默认范围：最新到最早
    let mut start_idx = 0;
    let mut end_idx = entries.len().saturating_sub(1);

    // to 版本（较新）-> 起始索引
    if let Some(to_ver) = to {
        if let Some(idx) = entries
            .iter()
            .position(|e| e.version == to_ver || e.version.ends_with(to_ver))
        {
            start_idx = idx;
        } else {
            return Err(anyhow!("未找到版本: {}", to_ver));
        }
    }

    // from 版本（较旧）-> 结束索引
    if let Some(from_ver) = from {
        if let Some(idx) = entries
            .iter()
            .position(|e| e.version == from_ver || e.version.ends_with(from_ver))
        {
            end_idx = idx;
        } else {
            return Err(anyhow!("未找到版本: {}", from_ver));
        }
    }

    // 确保顺序正确
    if start_idx > end_idx {
        std::mem::swap(&mut start_idx, &mut end_idx);
    }

    Ok((start_idx, end_idx))
}

/// 格式化多个条目
fn format_entries(entries: &[&ChangelogEntry]) -> String {
    entries
        .iter()
        .map(|e| e.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n---\n\n")
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
        let q = ChangelogQuery::from_json(r#"{"mode":"recent","count":5}"#);
        assert_eq!(q.mode(), "recent");
        assert_eq!(q.count(), 5);

        let q = ChangelogQuery::from_json(r#"{}"#);
        assert_eq!(q.mode(), "latest");
        assert_eq!(q.count(), 3);
    }

    #[test]
    fn test_parse_query_with_version() {
        let q = ChangelogQuery::from_json(r#"{"mode":"version","version":"2.0.55"}"#);
        assert_eq!(q.mode(), "version");
        assert_eq!(q.version.as_deref(), Some("2.0.55"));
    }

    #[test]
    fn test_parse_query_with_range() {
        let q = ChangelogQuery::from_json(r#"{"mode":"range","from":"2.0.50","to":"2.0.55"}"#);
        assert_eq!(q.mode(), "range");
        assert_eq!(q.from.as_deref(), Some("2.0.50"));
        assert_eq!(q.to.as_deref(), Some("2.0.55"));
    }

    #[test]
    fn test_parse_query_list_mode() {
        let q = ChangelogQuery::from_json(r#"{"mode":"list"}"#);
        assert_eq!(q.mode(), "list");
    }

    #[test]
    fn test_parse_changelog() {
        let body = r#"# Changelog

## 2.0.55
- Feature A
- Fix B

## 2.0.54
- Feature C

## 2.0.53
- Fix D
"#;
        let entries = parse_changelog(body).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].version, "2.0.55");
        assert_eq!(entries[1].version, "2.0.54");
        assert_eq!(entries[2].version, "2.0.53");
        assert!(entries[0].content.contains("Feature A"));
        assert!(entries[0].content.contains("Fix B"));
    }

    #[test]
    fn test_parse_changelog_empty() {
        let body = "# Changelog\n\nNo versions yet.";
        let result = parse_changelog(body);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "未能解析任何 changelog 条目"
        );
    }

    #[test]
    fn test_parse_changelog_with_extra_spaces() {
        let body = r#"# Changelog

##   2.0.55
- Feature A

##  2.0.54
- Feature B
"#;
        let entries = parse_changelog(body).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].version, "2.0.55");
        assert_eq!(entries[1].version, "2.0.54");
    }

    #[test]
    fn test_find_range_indices_full_range() {
        let entries = vec![
            ChangelogEntry {
                version: "2.0.55".to_string(),
                content: "Latest".to_string(),
            },
            ChangelogEntry {
                version: "2.0.54".to_string(),
                content: "Middle".to_string(),
            },
            ChangelogEntry {
                version: "2.0.53".to_string(),
                content: "Oldest".to_string(),
            },
        ];

        // 测试默认范围（全部）
        let (start, end) = find_range_indices(&entries, None, None).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_find_range_indices_with_from() {
        let entries = vec![
            ChangelogEntry {
                version: "2.0.55".to_string(),
                content: "Latest".to_string(),
            },
            ChangelogEntry {
                version: "2.0.54".to_string(),
                content: "Middle".to_string(),
            },
            ChangelogEntry {
                version: "2.0.53".to_string(),
                content: "Oldest".to_string(),
            },
        ];

        let (start, end) = find_range_indices(&entries, Some("2.0.54"), None).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
    }

    #[test]
    fn test_find_range_indices_with_to() {
        let entries = vec![
            ChangelogEntry {
                version: "2.0.55".to_string(),
                content: "Latest".to_string(),
            },
            ChangelogEntry {
                version: "2.0.54".to_string(),
                content: "Middle".to_string(),
            },
            ChangelogEntry {
                version: "2.0.53".to_string(),
                content: "Oldest".to_string(),
            },
        ];

        let (start, end) = find_range_indices(&entries, None, Some("2.0.54")).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_find_range_indices_both() {
        let entries = vec![
            ChangelogEntry {
                version: "2.0.55".to_string(),
                content: "Latest".to_string(),
            },
            ChangelogEntry {
                version: "2.0.54".to_string(),
                content: "Middle".to_string(),
            },
            ChangelogEntry {
                version: "2.0.53".to_string(),
                content: "Oldest".to_string(),
            },
        ];

        let (start, end) = find_range_indices(&entries, Some("2.0.53"), Some("2.0.55")).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_find_range_indices_version_not_found() {
        let entries = vec![ChangelogEntry {
            version: "2.0.55".to_string(),
            content: "Latest".to_string(),
        }];

        let result = find_range_indices(&entries, Some("1.0.0"), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "未找到版本: 1.0.0");

        let result = find_range_indices(&entries, None, Some("3.0.0"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "未找到版本: 3.0.0");
    }

    #[test]
    fn test_format_entries() {
        let entries = vec![
            ChangelogEntry {
                version: "2.0.55".to_string(),
                content: "## 2.0.55\n- Feature A".to_string(),
            },
            ChangelogEntry {
                version: "2.0.54".to_string(),
                content: "## 2.0.54\n- Feature B".to_string(),
            },
        ];

        let refs: Vec<&ChangelogEntry> = entries.iter().collect();
        let result = format_entries(&refs);
        assert!(result.contains("## 2.0.55"));
        assert!(result.contains("## 2.0.54"));
        assert!(result.contains("---"));
    }

    #[test]
    fn test_clamp_output_no_truncation() {
        let text = "Hello, World!".to_string();
        let (result, truncated) = clamp_output(text.clone(), 100);
        assert_eq!(result, text);
        assert!(!truncated);
    }

    #[test]
    fn test_clamp_output_with_truncation() {
        let text = "Hello, World! This is a long text.".to_string();
        let (result, truncated) = clamp_output(text, 10);
        assert_eq!(result.len(), 10);
        assert!(truncated);
    }

    #[test]
    fn test_clamp_output_zero_max() {
        let text = "Hello".to_string();
        let (result, truncated) = clamp_output(text, 0);
        assert_eq!(result, "");
        assert!(truncated);
    }

    #[test]
    fn test_clamp_output_exact_boundary() {
        let text = "Hello".to_string();
        let (result, truncated) = clamp_output(text.clone(), 5);
        assert_eq!(result, text);
        assert!(!truncated);
    }

    #[test]
    fn test_clamp_output_utf8_boundary() {
        // 测试 UTF-8 字符边界处理
        let text = "你好世界".to_string(); // 每个字符 3 字节
        let (result, truncated) = clamp_output(text.clone(), 20);
        assert_eq!(result, text);
        assert!(!truncated);

        let (result, truncated) = clamp_output(text, 7);
        assert_eq!(result.len(), 6); // 应该在字符边界处截断
        assert!(truncated);
    }

    #[test]
    fn test_changelog_query_defaults() {
        let q = ChangelogQuery::default();
        assert_eq!(q.mode(), "latest");
        assert_eq!(q.count(), 3);
        assert!(q.version.is_none());
        assert!(q.from.is_none());
        assert!(q.to.is_none());
    }

    #[test]
    fn test_parse_changelog_with_multiline_content() {
        let body = r#"# Changelog

## 2.0.55
- Feature A
  - Sub feature 1
  - Sub feature 2
- Fix B

## 2.0.54
- Feature C
"#;
        let entries = parse_changelog(body).unwrap();
        assert_eq!(entries.len(), 2);
        assert!(entries[0].content.contains("Sub feature 1"));
        assert!(entries[0].content.contains("Sub feature 2"));
    }

    #[test]
    fn test_parse_changelog_version_ending_match() {
        let body = r#"# Changelog

## v2.0.55 (2024-01-01)
- Feature A
"#;
        let entries = parse_changelog(body).unwrap();
        assert_eq!(entries.len(), 1);
        // 版本号应包含完整的版本行
        assert!(entries[0].version.contains("2.0.55"));
    }
}
