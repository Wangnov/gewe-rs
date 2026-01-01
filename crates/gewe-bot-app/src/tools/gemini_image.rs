//! Gemini 图像生成工具
//!
//! 使用 Gemini API 根据用户描述生成图片
//! 支持 gemini-3-pro-image-preview 等模型

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::time;
use uuid::Uuid;

/// 默认图像生成模型
const DEFAULT_MODEL: &str = "gemini-3-pro-image-preview";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

/// 图像生成查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImageQuery {
    /// 图像描述 prompt
    #[serde(default)]
    pub prompt: Option<String>,
    /// 模型名称，默认 gemini-3-pro-image-preview
    #[serde(default)]
    pub model: Option<String>,
    /// 图像宽高比: 1:1, 2:3, 3:2, 3:4, 4:3, 9:16, 16:9, 21:9
    #[serde(default)]
    pub aspect_ratio: Option<String>,
    /// 图像尺寸: 1K, 2K, 4K
    #[serde(default)]
    pub image_size: Option<String>,
    /// 是否启用 Google 搜索（让模型先搜索信息再生成图片）
    #[serde(default)]
    pub google_search: Option<bool>,
}

impl ImageQuery {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }

    pub fn prompt(&self) -> &str {
        self.prompt.as_deref().unwrap_or("")
    }

    pub fn model(&self) -> &str {
        self.model.as_deref().unwrap_or(DEFAULT_MODEL)
    }
}

/// 图像生成配置
#[derive(Debug, Clone)]
pub struct ImageConfig {
    /// Gemini API Key
    pub api_key: String,
    /// 基础 API URL（可选，用于代理）
    pub base_url: Option<String>,
    /// 图片存储目录
    pub image_dir: String,
    /// 图片 URL 前缀
    pub image_url_prefix: String,
    /// 外部访问基础 URL
    pub external_base_url: Option<String>,
}

/// 图像生成结果
pub struct ImageResult {
    /// 文本回复
    pub text: Option<String>,
    /// 生成的图片 URL 列表
    pub image_urls: Vec<String>,
    /// 是否截断
    pub truncated: bool,
    /// 执行时长
    pub duration: Duration,
    /// 错误信息
    pub error: Option<String>,
    /// 是否超时
    pub timed_out: bool,
}

/// Gemini API 请求结构
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GoogleSearchTool>>,
}

/// Google 搜索工具
#[derive(Debug, Serialize)]
struct GoogleSearchTool {
    google_search: EmptyObject,
}

/// 空对象 {}
#[derive(Debug, Serialize)]
struct EmptyObject {}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    #[serde(rename = "responseModalities")]
    response_modalities: Vec<String>,
    #[serde(rename = "responseMimeType", skip_serializing_if = "Option::is_none")]
    response_mime_type: Option<String>,
    #[serde(rename = "imageConfig", skip_serializing_if = "Option::is_none")]
    image_config: Option<ImageGenerationConfig>,
}

#[derive(Debug, Serialize)]
struct ImageGenerationConfig {
    #[serde(rename = "aspectRatio", skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    #[serde(rename = "imageSize", skip_serializing_if = "Option::is_none")]
    image_size: Option<String>,
}

/// Gemini API 响应结构
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<GeminiError>,
}

#[derive(Debug, Deserialize)]
struct GeminiError {
    message: String,
    #[allow(dead_code)]
    code: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<ResponseContent>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Option<Vec<ResponsePart>>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: Option<String>,
    #[serde(rename = "inlineData")]
    inline_data: Option<InlineData>,
}

#[derive(Debug, Deserialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

/// 执行图像生成
pub async fn run_gemini_image(
    query: ImageQuery,
    config: &ImageConfig,
    timeout_secs: Option<u64>,
    max_output: usize,
) -> ImageResult {
    let timeout = timeout_secs
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TIMEOUT);
    let start = Instant::now();

    match time::timeout(timeout, execute_image_generation(&query, config)).await {
        Ok(Ok((text, image_urls))) => {
            let (final_text, truncated) = clamp_output(text, max_output);
            ImageResult {
                text: if final_text.is_empty() {
                    None
                } else {
                    Some(final_text)
                },
                image_urls,
                truncated,
                duration: start.elapsed(),
                error: None,
                timed_out: false,
            }
        }
        Ok(Err(err)) => ImageResult {
            text: Some(format!("图像生成失败: {}", err)),
            image_urls: vec![],
            truncated: false,
            duration: start.elapsed(),
            error: Some(err.to_string()),
            timed_out: false,
        },
        Err(_) => ImageResult {
            text: Some("图像生成超时".to_string()),
            image_urls: vec![],
            truncated: false,
            duration: timeout,
            error: Some("timeout".to_string()),
            timed_out: true,
        },
    }
}

/// 执行图像生成请求
async fn execute_image_generation(
    query: &ImageQuery,
    config: &ImageConfig,
) -> Result<(String, Vec<String>)> {
    let prompt = query.prompt();
    if prompt.is_empty() {
        return Err(anyhow!("prompt 不能为空"));
    }

    let model = query.model();
    let aspect_ratio = query.aspect_ratio.clone();
    let image_size = query.image_size.clone();
    let google_search = query.google_search.unwrap_or(false);
    let base = config
        .base_url
        .as_deref()
        .unwrap_or("https://generativelanguage.googleapis.com");
    let api_url = format!(
        "{}/v1beta/models/{}:generateContent?key={}",
        base.trim_end_matches('/'),
        model,
        config.api_key
    );

    tracing::debug!(
        model,
        prompt,
        ?aspect_ratio,
        ?image_size,
        google_search,
        "发送 Gemini 图像生成请求"
    );

    // 构建 imageConfig（如果有 aspect_ratio 或 image_size）
    let image_config = if aspect_ratio.is_some() || image_size.is_some() {
        Some(ImageGenerationConfig {
            aspect_ratio,
            image_size,
        })
    } else {
        None
    };

    // 构建 tools（如果启用 google_search）
    let tools = if google_search {
        Some(vec![GoogleSearchTool {
            google_search: EmptyObject {},
        }])
    } else {
        None
    };

    // 构建请求
    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part::Text {
                text: prompt.to_string(),
            }],
        }],
        generation_config: GenerationConfig {
            response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
            response_mime_type: None,
            image_config,
        },
        tools,
    };

    // 发送请求
    let client = reqwest::Client::new();
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| anyhow!("请求失败: {}", e))?;

    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| anyhow!("读取响应失败: {}", e))?;

    if !status.is_success() {
        tracing::warn!(status = %status, body = %body, "Gemini API 错误响应");
        return Err(anyhow!("API 请求失败 ({}): {}", status, body));
    }

    let gemini_response: GeminiResponse =
        serde_json::from_str(&body).map_err(|e| anyhow!("解析响应失败: {}", e))?;

    if let Some(err) = gemini_response.error {
        return Err(anyhow!("Gemini API 错误: {}", err.message));
    }

    // 解析响应
    let candidates = gemini_response
        .candidates
        .ok_or_else(|| anyhow!("响应中无 candidates"))?;

    let mut text_parts = Vec::new();
    let mut image_urls = Vec::new();

    for candidate in candidates {
        if let Some(content) = candidate.content {
            if let Some(parts) = content.parts {
                for part in parts {
                    if let Some(text) = part.text {
                        if !text.trim().is_empty() {
                            text_parts.push(text);
                        }
                    }
                    if let Some(inline_data) = part.inline_data {
                        // 保存图片并获取 URL
                        match save_image(&inline_data, config).await {
                            Ok(url) => image_urls.push(url),
                            Err(e) => tracing::warn!(?e, "保存图片失败"),
                        }
                    }
                }
            }
        }
    }

    let combined_text = text_parts.join("\n");
    Ok((combined_text, image_urls))
}

/// 保存图片到本地并返回访问 URL
async fn save_image(inline_data: &InlineData, config: &ImageConfig) -> Result<String> {
    // 解码 base64 数据
    let image_data = BASE64
        .decode(&inline_data.data)
        .map_err(|e| anyhow!("Base64 解码失败: {}", e))?;

    // 根据 MIME 类型确定扩展名
    let ext = match inline_data.mime_type.as_str() {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpg",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "png", // 默认 png
    };

    // 生成唯一文件名
    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = Path::new(&config.image_dir).join(&filename);

    // 确保目录存在
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    // 写入文件
    fs::write(&file_path, &image_data)
        .await
        .map_err(|e| anyhow!("写入文件失败: {}", e))?;

    tracing::info!(
        path = %file_path.display(),
        size = image_data.len(),
        mime = %inline_data.mime_type,
        "图片已保存"
    );

    // 构建访问 URL
    let url = if let Some(ref base_url) = config.external_base_url {
        format!(
            "{}{}/{}",
            base_url.trim_end_matches('/'),
            config.image_url_prefix,
            filename
        )
    } else {
        format!("{}/{}", config.image_url_prefix, filename)
    };

    Ok(url)
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
        let q =
            ImageQuery::from_json(r#"{"prompt":"画一只猫","model":"gemini-3-pro-image-preview"}"#);
        assert_eq!(q.prompt(), "画一只猫");
        assert_eq!(q.model(), "gemini-3-pro-image-preview");

        let q = ImageQuery::from_json(r#"{}"#);
        assert_eq!(q.prompt(), "");
        assert_eq!(q.model(), DEFAULT_MODEL);
    }

    #[test]
    fn test_parse_query_with_aspect_ratio() {
        let q = ImageQuery::from_json(
            r#"{"prompt":"画一只猫","aspect_ratio":"16:9","image_size":"4K"}"#,
        );
        assert_eq!(q.prompt(), "画一只猫");
        assert_eq!(q.aspect_ratio.as_deref(), Some("16:9"));
        assert_eq!(q.image_size.as_deref(), Some("4K"));
    }

    #[test]
    fn test_parse_query_with_google_search() {
        let q = ImageQuery::from_json(r#"{"prompt":"画一只猫","google_search":true}"#);
        assert_eq!(q.prompt(), "画一只猫");
        assert_eq!(q.google_search, Some(true));

        let q = ImageQuery::from_json(r#"{"prompt":"画一只猫"}"#);
        assert_eq!(q.google_search, None);
    }

    #[test]
    fn test_image_query_defaults() {
        let q = ImageQuery::default();
        assert_eq!(q.prompt(), "");
        assert_eq!(q.model(), DEFAULT_MODEL);
        assert!(q.aspect_ratio.is_none());
        assert!(q.image_size.is_none());
        assert!(q.google_search.is_none());
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
    fn test_image_config_creation() {
        let config = ImageConfig {
            api_key: "test_key".to_string(),
            base_url: Some("https://api.test.com".to_string()),
            image_dir: "/tmp/images".to_string(),
            image_url_prefix: "/images".to_string(),
            external_base_url: Some("https://example.com".to_string()),
        };

        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.base_url, Some("https://api.test.com".to_string()));
        assert_eq!(config.image_dir, "/tmp/images");
        assert_eq!(config.image_url_prefix, "/images");
        assert_eq!(
            config.external_base_url,
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_image_query_from_json_invalid() {
        // 测试无效 JSON 应返回默认值
        let q = ImageQuery::from_json("invalid json");
        assert_eq!(q.prompt(), "");
        assert_eq!(q.model(), DEFAULT_MODEL);
    }

    #[test]
    fn test_image_query_all_aspect_ratios() {
        let ratios = vec!["1:1", "2:3", "3:2", "3:4", "4:3", "9:16", "16:9", "21:9"];
        for ratio in ratios {
            let json = format!(r#"{{"prompt":"test","aspect_ratio":"{}"}}"#, ratio);
            let q = ImageQuery::from_json(&json);
            assert_eq!(q.aspect_ratio.as_deref(), Some(ratio));
        }
    }

    #[test]
    fn test_image_query_all_sizes() {
        let sizes = vec!["1K", "2K", "4K"];
        for size in sizes {
            let json = format!(r#"{{"prompt":"test","image_size":"{}"}}"#, size);
            let q = ImageQuery::from_json(&json);
            assert_eq!(q.image_size.as_deref(), Some(size));
        }
    }

    #[test]
    fn test_gemini_request_structure() {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part::Text {
                    text: "test prompt".to_string(),
                }],
            }],
            generation_config: GenerationConfig {
                response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
                response_mime_type: None,
                image_config: Some(ImageGenerationConfig {
                    aspect_ratio: Some("16:9".to_string()),
                    image_size: Some("4K".to_string()),
                }),
            },
            tools: None,
        };

        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.generation_config.response_modalities.len(), 2);
        assert!(request.generation_config.image_config.is_some());
        assert!(request.tools.is_none());
    }

    #[test]
    fn test_gemini_request_with_google_search() {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part::Text {
                    text: "test prompt".to_string(),
                }],
            }],
            generation_config: GenerationConfig {
                response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
                response_mime_type: None,
                image_config: None,
            },
            tools: Some(vec![GoogleSearchTool {
                google_search: EmptyObject {},
            }]),
        };

        assert!(request.tools.is_some());
        assert_eq!(request.tools.unwrap().len(), 1);
    }

    #[test]
    fn test_inline_data_mime_types() {
        let mime_types = vec![
            ("image/png", "png"),
            ("image/jpeg", "jpg"),
            ("image/jpg", "jpg"),
            ("image/webp", "webp"),
            ("image/gif", "gif"),
        ];

        for (mime, expected_ext) in mime_types {
            let inline_data = InlineData {
                mime_type: mime.to_string(),
                data: "test_base64_data".to_string(),
            };
            assert_eq!(inline_data.mime_type, mime);

            // 根据 mime_type 确定扩展名的逻辑
            let ext = match inline_data.mime_type.as_str() {
                "image/png" => "png",
                "image/jpeg" | "image/jpg" => "jpg",
                "image/webp" => "webp",
                "image/gif" => "gif",
                _ => "png",
            };
            assert_eq!(ext, expected_ext);
        }
    }

    #[test]
    fn test_image_query_model_variants() {
        let models = vec![
            "gemini-3-pro-image-preview",
            "gemini-2-flash-image-preview",
            "custom-model",
        ];

        for model in models {
            let json = format!(r#"{{"prompt":"test","model":"{}"}}"#, model);
            let q = ImageQuery::from_json(&json);
            assert_eq!(q.model(), model);
        }
    }

    #[test]
    fn test_image_result_structure() {
        let result = ImageResult {
            text: Some("Generated successfully".to_string()),
            image_urls: vec![
                "https://example.com/image1.png".to_string(),
                "https://example.com/image2.png".to_string(),
            ],
            truncated: false,
            duration: Duration::from_secs(5),
            error: None,
            timed_out: false,
        };

        assert_eq!(result.text, Some("Generated successfully".to_string()));
        assert_eq!(result.image_urls.len(), 2);
        assert!(!result.truncated);
        assert!(!result.timed_out);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_image_result_with_error() {
        let result = ImageResult {
            text: Some("Failed to generate".to_string()),
            image_urls: vec![],
            truncated: false,
            duration: Duration::from_secs(1),
            error: Some("API error".to_string()),
            timed_out: false,
        };

        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap(), "API error");
        assert!(result.image_urls.is_empty());
    }

    #[test]
    fn test_image_result_timeout() {
        let result = ImageResult {
            text: Some("Timeout".to_string()),
            image_urls: vec![],
            truncated: false,
            duration: Duration::from_secs(120),
            error: Some("timeout".to_string()),
            timed_out: true,
        };

        assert!(result.timed_out);
        assert_eq!(result.error.unwrap(), "timeout");
    }
}
