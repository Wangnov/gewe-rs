//! Prompts 相关 API 处理函数

use super::state::ApiState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 通用 API 响应
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

/// Prompt 文件信息
#[derive(Serialize)]
pub struct PromptInfo {
    pub name: String,
    pub size: u64,
    pub modified_at: Option<String>,
}

/// Prompt 内容响应
#[derive(Serialize)]
pub struct PromptContent {
    pub name: String,
    pub content: String,
}

/// Prompt 列表响应
#[derive(Serialize)]
pub struct PromptListResponse {
    pub prompts: Vec<PromptInfo>,
}

/// 验证文件名是否安全（防止路径遍历）
fn is_safe_filename(name: &str) -> bool {
    // 不能包含路径分隔符
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return false;
    }
    // 必须以 .txt 或 .md 结尾
    if !name.ends_with(".txt") && !name.ends_with(".md") {
        return false;
    }
    // 不能是空或只有扩展名
    let stem = name.trim_end_matches(".txt").trim_end_matches(".md");
    if stem.is_empty() {
        return false;
    }
    true
}

/// 构建安全的文件路径
fn safe_prompt_path(prompts_dir: &std::path::Path, name: &str) -> Option<PathBuf> {
    if !is_safe_filename(name) {
        return None;
    }
    let path = prompts_dir.join(name);
    // 确保路径在 prompts 目录内
    if let Ok(canonical) = path.canonicalize() {
        if let Ok(base) = prompts_dir.canonicalize() {
            if canonical.starts_with(&base) {
                return Some(canonical);
            }
        }
    }
    // 如果文件不存在，检查父目录
    if !path.exists() {
        if let Ok(base) = prompts_dir.canonicalize() {
            let normalized = base.join(name);
            if normalized.starts_with(&base) {
                return Some(normalized);
            }
        }
    }
    None
}

/// GET /api/prompts - 列出所有 prompt 文件
pub async fn list_prompts(State(state): State<ApiState>) -> impl IntoResponse {
    let prompts_dir = state.prompts_dir();

    // 确保目录存在
    if !prompts_dir.exists() {
        if let Err(e) = tokio::fs::create_dir_all(prompts_dir).await {
            return Json(ApiResponse::<PromptListResponse>::error(format!(
                "创建目录失败: {}",
                e
            )));
        }
    }

    let mut prompts = Vec::new();

    let mut entries = match tokio::fs::read_dir(prompts_dir).await {
        Ok(e) => e,
        Err(e) => {
            return Json(ApiResponse::<PromptListResponse>::error(format!(
                "读取目录失败: {}",
                e
            )));
        }
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let filename = entry.file_name().to_string_lossy().to_string();
        if !filename.ends_with(".txt") && !filename.ends_with(".md") {
            continue;
        }

        let metadata = match entry.metadata().await {
            Ok(m) => m,
            Err(_) => continue,
        };

        if !metadata.is_file() {
            continue;
        }

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs())
            })
            .map(|secs| {
                chrono::DateTime::from_timestamp(secs as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default()
            });

        prompts.push(PromptInfo {
            name: filename,
            size: metadata.len(),
            modified_at,
        });
    }

    // 按名称排序
    prompts.sort_by(|a, b| a.name.cmp(&b.name));

    Json(ApiResponse::success(PromptListResponse { prompts }))
}

/// GET /api/prompts/:name - 获取指定 prompt 内容
pub async fn get_prompt(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let prompts_dir = state.prompts_dir();

    // 安全性检查
    if !is_safe_filename(&name) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<PromptContent>::error(
                "文件名无效，必须是 .txt 或 .md 文件且不能包含路径",
            )),
        );
    }

    let path = prompts_dir.join(&name);

    // 检查文件是否存在
    if !path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<PromptContent>::error(format!(
                "文件不存在: {}",
                name
            ))),
        );
    }

    // 再次验证路径安全
    match safe_prompt_path(prompts_dir, &name) {
        Some(_) => {}
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(ApiResponse::<PromptContent>::error("禁止访问该路径")),
            );
        }
    }

    // 读取内容
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => (
            StatusCode::OK,
            Json(ApiResponse::success(PromptContent { name, content })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<PromptContent>::error(format!(
                "读取文件失败: {}",
                e
            ))),
        ),
    }
}

/// PUT 请求体
#[derive(Deserialize)]
pub struct PutPromptRequest {
    pub content: String,
}

/// PUT 响应
#[derive(Serialize)]
pub struct PutPromptResponse {
    pub name: String,
    pub size: usize,
    pub saved_at: String,
}

/// PUT /api/prompts/:name - 写入指定 prompt 内容
pub async fn put_prompt(
    State(state): State<ApiState>,
    Path(name): Path<String>,
    Json(req): Json<PutPromptRequest>,
) -> impl IntoResponse {
    let prompts_dir = state.prompts_dir();

    // 安全性检查
    if !is_safe_filename(&name) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<PutPromptResponse>::error(
                "文件名无效，必须是 .txt 或 .md 文件且不能包含路径",
            )),
        );
    }

    // 确保目录存在
    if !prompts_dir.exists() {
        if let Err(e) = tokio::fs::create_dir_all(prompts_dir).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<PutPromptResponse>::error(format!(
                    "创建目录失败: {}",
                    e
                ))),
            );
        }
    }

    let path = prompts_dir.join(&name);

    // 写入内容
    let content_len = req.content.len();
    match tokio::fs::write(&path, &req.content).await {
        Ok(()) => {
            let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            tracing::info!(name = %name, size = content_len, "prompt 文件已保存");
            (
                StatusCode::OK,
                Json(ApiResponse::success(PutPromptResponse {
                    name,
                    size: content_len,
                    saved_at: now,
                })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<PutPromptResponse>::error(format!(
                "写入文件失败: {}",
                e
            ))),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_filename() {
        assert!(is_safe_filename("ai_system.txt"));
        assert!(is_safe_filename("readme.md"));
        assert!(is_safe_filename("my_prompt_v2.txt"));

        assert!(!is_safe_filename("../etc/passwd"));
        assert!(!is_safe_filename("/etc/passwd"));
        assert!(!is_safe_filename("foo/bar.txt"));
        assert!(!is_safe_filename("..\\windows\\system32"));
        assert!(!is_safe_filename(".txt"));
        assert!(!is_safe_filename("config.toml"));
        assert!(!is_safe_filename("script.sh"));
    }
}
