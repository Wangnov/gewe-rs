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
