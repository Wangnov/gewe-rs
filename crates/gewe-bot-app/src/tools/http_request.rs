//! 通用 HTTP 请求工具
//!
//! 支持自定义 method/url/headers/query/body，自动格式化 JSON 响应

use anyhow::{anyhow, Result};
use reqwest::{header, Method};
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// HTTP 请求参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HttpRequestQuery {
    /// 完整 URL，必填
    #[serde(default)]
    pub url: Option<String>,
    /// HTTP 方法，默认 GET
    #[serde(default)]
    pub method: Option<String>,
    /// 查询参数，将附加到 URL 上
    #[serde(default)]
    pub query: Option<HashMap<String, String>>,
    /// 请求头
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    /// JSON 请求体（优先级低于 body_text）
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    /// 原始文本请求体
    #[serde(default)]
    pub body_text: Option<String>,
    /// 是否将响应按 JSON 格式化输出（未指定时按 content-type 自动判断）
    #[serde(default)]
    pub expect_json: Option<bool>,
}

impl HttpRequestQuery {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }

    fn url(&self) -> Result<&str> {
        self.url.as_deref().ok_or_else(|| anyhow!("缺少 url 参数"))
    }

    fn method(&self) -> Result<Method> {
        let m = self.method.as_deref().unwrap_or("GET").trim();
        Method::from_bytes(m.as_bytes()).map_err(|_| anyhow!("不支持的 HTTP 方法: {}", m))
    }

    fn expect_json(&self, content_type: Option<&str>) -> bool {
        self.expect_json.unwrap_or_else(|| {
            content_type
                .map(|ct| {
                    let ct = ct.to_ascii_lowercase();
                    ct.contains("application/json") || ct.contains("+json")
                })
                .unwrap_or(false)
        })
    }
}

/// 执行结果
pub struct HttpRequestResult {
    pub content: String,
    pub truncated: bool,
    pub duration: Duration,
    pub error: Option<String>,
    pub timed_out: bool,
}

/// 执行 HTTP 请求
pub async fn run_http_request(
    query: HttpRequestQuery,
    timeout_secs: Option<u64>,
    max_output: usize,
) -> HttpRequestResult {
    let timeout = timeout_secs
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_TIMEOUT);
    let start = Instant::now();

    match time::timeout(timeout, execute_request(&query)).await {
        Ok(Ok(content)) => {
            let (text, truncated) = clamp_output(content, max_output);
            HttpRequestResult {
                content: text,
                truncated,
                duration: start.elapsed(),
                error: None,
                timed_out: false,
            }
        }
        Ok(Err(err)) => HttpRequestResult {
            content: format!("HTTP 请求失败: {}", err),
            truncated: false,
            duration: start.elapsed(),
            error: Some(err.to_string()),
            timed_out: false,
        },
        Err(_) => HttpRequestResult {
            content: "HTTP 请求超时".to_string(),
            truncated: false,
            duration: timeout,
            error: Some("timeout".to_string()),
            timed_out: true,
        },
    }
}

/// 执行请求并格式化输出
async fn execute_request(query: &HttpRequestQuery) -> Result<String> {
    let url = query.url()?;
    let method = query.method()?;

    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| anyhow!("创建 HTTP 客户端失败: {e}"))?;

    let mut builder = client.request(method.clone(), url);

    if let Some(ref params) = query.query {
        builder = builder.query(params);
    }

    if let Some(ref headers) = query.headers {
        let mut header_map = header::HeaderMap::new();
        for (k, v) in headers {
            let name = header::HeaderName::from_bytes(k.as_bytes())
                .map_err(|_| anyhow!("无效 header 名称: {}", k))?;
            let value =
                header::HeaderValue::from_str(v).map_err(|_| anyhow!("无效 header 值: {}", k))?;
            header_map.insert(name, value);
        }
        builder = builder.headers(header_map);
    }

    if let Some(ref body_text) = query.body_text {
        builder = builder.body(body_text.clone());
    } else if let Some(ref body) = query.body {
        builder = builder.json(body);
    }

    let resp = builder.send().await.map_err(|e| anyhow!("请求失败: {e}"))?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string());

    let raw_body = resp
        .text()
        .await
        .map_err(|e| anyhow!("读取响应失败: {e}"))?;

    let formatted_body = if query.expect_json(content_type.as_deref()) {
        match serde_json::from_str::<serde_json::Value>(&raw_body) {
            Ok(val) => serde_json::to_string_pretty(&val).unwrap_or(raw_body),
            Err(_) => raw_body,
        }
    } else {
        raw_body
    };

    let mut lines = Vec::new();
    lines.push("HTTP 请求结果：".to_string());
    lines.push(String::new());
    lines.push(format!("URL: {}", url));
    lines.push(format!("Method: {}", method));
    lines.push(format!(
        "状态: {} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("Unknown")
    ));
    if let Some(ct) = content_type {
        lines.push(format!("Content-Type: {}", ct));
    }
    lines.push(String::new());
    lines.push("响应体:".to_string());
    lines.push(formatted_body);

    Ok(lines.join("\n"))
}

fn clamp_output(text: String, max: usize) -> (String, bool) {
    let bytes = text.as_bytes();
    if bytes.len() <= max {
        (text, false)
    } else {
        let truncated = String::from_utf8_lossy(&bytes[..max]).into_owned();
        (
            format!("{truncated}\n\n[输出已截断，上限 {} 字节]", max),
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_query_from_json() {
        let query = HttpRequestQuery::from_json(
            r#"{"url":"https://api.example.com","method":"POST","expect_json":true}"#,
        );
        assert_eq!(query.url.as_deref(), Some("https://api.example.com"));
        assert_eq!(query.method.as_deref(), Some("POST"));
        assert_eq!(query.expect_json, Some(true));

        // 测试默认值
        let query = HttpRequestQuery::from_json(r#"{}"#);
        assert!(query.url.is_none());
        assert!(query.method.is_none());
        assert!(query.expect_json.is_none());
    }

    #[test]
    fn test_http_request_query_url() {
        let query = HttpRequestQuery {
            url: Some("https://example.com".to_string()),
            ..Default::default()
        };
        assert_eq!(query.url().unwrap(), "https://example.com");

        let query = HttpRequestQuery::default();
        assert!(query.url().is_err());
        assert_eq!(query.url().unwrap_err().to_string(), "缺少 url 参数");
    }

    #[test]
    fn test_http_request_query_method() {
        // 测试默认 GET
        let query = HttpRequestQuery::default();
        assert_eq!(query.method().unwrap(), Method::GET);

        // 测试 POST
        let query = HttpRequestQuery {
            method: Some("POST".to_string()),
            ..Default::default()
        };
        assert_eq!(query.method().unwrap(), Method::POST);

        // 测试 PUT
        let query = HttpRequestQuery {
            method: Some("PUT".to_string()),
            ..Default::default()
        };
        assert_eq!(query.method().unwrap(), Method::PUT);

        // 测试无效方法（应该成功，因为 trim() 后是有效的）
        let query = HttpRequestQuery {
            method: Some(" GET ".to_string()),
            ..Default::default()
        };
        assert_eq!(query.method().unwrap(), Method::GET);
    }

    #[test]
    fn test_http_request_query_expect_json() {
        // 显式指定 expect_json
        let query = HttpRequestQuery {
            expect_json: Some(true),
            ..Default::default()
        };
        assert!(query.expect_json(None));
        assert!(query.expect_json(Some("text/plain")));

        // 根据 content-type 判断
        let query = HttpRequestQuery::default();
        assert!(query.expect_json(Some("application/json")));
        assert!(query.expect_json(Some("application/vnd.api+json")));
        assert!(!query.expect_json(Some("text/html")));
        assert!(!query.expect_json(None));
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
        assert_eq!(result.len(), 10 + "\n\n[输出已截断，上限 10 字节]".len());
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

        let (result, truncated) = clamp_output(text, 6);
        assert!(truncated);
        assert!(result.contains("[输出已截断"));
    }

    #[test]
    fn test_http_request_query_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let query = HttpRequestQuery {
            url: Some("https://api.example.com".to_string()),
            headers: Some(headers.clone()),
            ..Default::default()
        };

        assert_eq!(query.headers, Some(headers));
    }

    #[test]
    fn test_http_request_query_with_query_params() {
        let mut params = HashMap::new();
        params.insert("page".to_string(), "1".to_string());
        params.insert("limit".to_string(), "10".to_string());

        let query = HttpRequestQuery {
            url: Some("https://api.example.com".to_string()),
            query: Some(params.clone()),
            ..Default::default()
        };

        assert_eq!(query.query, Some(params));
    }

    #[test]
    fn test_http_request_query_with_body() {
        let body = serde_json::json!({
            "name": "test",
            "value": 42
        });

        let query = HttpRequestQuery {
            url: Some("https://api.example.com".to_string()),
            method: Some("POST".to_string()),
            body: Some(body.clone()),
            ..Default::default()
        };

        assert_eq!(query.body, Some(body));
    }

    #[test]
    fn test_http_request_query_with_body_text() {
        let body_text = "plain text body".to_string();

        let query = HttpRequestQuery {
            url: Some("https://api.example.com".to_string()),
            method: Some("POST".to_string()),
            body_text: Some(body_text.clone()),
            ..Default::default()
        };

        assert_eq!(query.body_text, Some(body_text));
    }

    #[test]
    fn test_expect_json_with_charset() {
        let query = HttpRequestQuery::default();
        assert!(query.expect_json(Some("application/json; charset=utf-8")));
    }

    #[test]
    fn test_expect_json_case_insensitive() {
        let query = HttpRequestQuery::default();
        assert!(query.expect_json(Some("Application/JSON")));
        assert!(query.expect_json(Some("APPLICATION/JSON")));
    }
}
