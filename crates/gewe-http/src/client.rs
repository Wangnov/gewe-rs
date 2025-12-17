use gewe_core::{ApiEnvelope, GeweError};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct GeweHttpClient {
    client: Client,
    base_url: String,
}

impl GeweHttpClient {
    pub fn new(token: impl Into<String>, base_url: impl Into<String>) -> Result<Self, GeweError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let token = token.into();
        headers.insert(
            "X-GEWE-TOKEN",
            reqwest::header::HeaderValue::from_str(&token)
                .map_err(|e| GeweError::Http(e.to_string()))?,
        );
        let client = ClientBuilder::new()
            .default_headers(headers)
            .pool_idle_timeout(Duration::from_secs(90))
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|e| GeweError::Http(e.to_string()))?;
        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    pub(crate) async fn post_api<B, R>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<ApiEnvelope<R>, GeweError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let resp = self
            .client
            .post(self.endpoint(path))
            .json(body)
            .send()
            .await
            .map_err(|e| GeweError::Http(e.to_string()))?;
        let text = resp
            .text()
            .await
            .map_err(|e| GeweError::Decode(format!("read body failed: {e}")))?;

        // 先解析为通用 JSON，检查 ret 状态
        let raw: Value = serde_json::from_str(&text)
            .map_err(|e| GeweError::Decode(format!("{e}; body={text}")))?;

        let ret = raw
            .get("ret")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1) as i32;
        let msg = raw
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        // 如果 ret != 200，直接返回 API 错误，不尝试解析 data
        if ret != 200 {
            return Err(GeweError::Api { code: ret, message: msg });
        }

        // ret == 200 时，再解析完整的响应结构
        let env: ApiEnvelope<R> = serde_json::from_str(&text)
            .map_err(|e| GeweError::Decode(format!("{e}; body={text}")))?;

        Ok(env)
    }
}
