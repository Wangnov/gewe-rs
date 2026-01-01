use gewe_core::{ApiEnvelope, GeweError};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct GeweHttpClient {
    client: Client,
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) base_url: String,
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

    pub(crate) fn endpoint(&self, path: &str) -> String {
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

        let ret = raw.get("ret").and_then(|v| v.as_i64()).unwrap_or(-1) as i32;
        let msg = raw
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        // 如果 ret != 200，直接返回 API 错误，不尝试解析 data
        if ret != 200 {
            return Err(GeweError::Api {
                code: ret,
                message: msg,
            });
        }

        // ret == 200 时，再解析完整的响应结构
        let env: ApiEnvelope<R> = serde_json::from_str(&text)
            .map_err(|e| GeweError::Decode(format!("{e}; body={text}")))?;

        Ok(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gewe_core::ApiEnvelope;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        value: String,
    }

    #[test]
    fn test_client_creation() {
        let client = GeweHttpClient::new("test_token", "https://api.example.com");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_endpoint_construction() {
        let client = GeweHttpClient::new("token", "https://api.example.com")
            .expect("Failed to create client");

        // 测试基本路径
        assert_eq!(
            client.endpoint("api/test"),
            "https://api.example.com/api/test"
        );

        // 测试带前导斜杠的路径
        assert_eq!(
            client.endpoint("/api/test"),
            "https://api.example.com/api/test"
        );
    }

    #[test]
    fn test_client_with_invalid_token() {
        // 测试包含非法字符的 token
        let result = GeweHttpClient::new("invalid\ntoken", "https://api.example.com");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, GeweError::Http(_)));
        }
    }

    #[test]
    fn test_client_clone() {
        let client = GeweHttpClient::new("token", "https://api.example.com")
            .expect("Failed to create client");
        let cloned = client.clone();

        assert_eq!(client.base_url, cloned.base_url);
    }

    #[test]
    fn test_endpoint_with_trailing_slashes() {
        let client = GeweHttpClient::new("token", "https://api.example.com/")
            .expect("Failed to create client");

        // base_url 有尾部斜杠，path 有前导斜杠
        assert_eq!(
            client.endpoint("/api/test"),
            "https://api.example.com/api/test"
        );

        // base_url 有尾部斜杠，path 无前导斜杠
        assert_eq!(
            client.endpoint("api/test"),
            "https://api.example.com/api/test"
        );
    }

    #[test]
    fn test_endpoint_with_query_params() {
        let client = GeweHttpClient::new("token", "https://api.example.com")
            .expect("Failed to create client");

        // 虽然通常不用 query params，但测试边界情况
        assert_eq!(
            client.endpoint("api/test?key=value"),
            "https://api.example.com/api/test?key=value"
        );
    }

    #[test]
    fn test_api_envelope_parsing() {
        // 测试 ApiEnvelope 的解析
        let json = r#"{
            "ret": 200,
            "msg": "success",
            "data": {
                "value": "test"
            }
        }"#;

        let envelope: ApiEnvelope<TestData> = serde_json::from_str(json).unwrap();
        assert_eq!(envelope.ret, 200);
        assert_eq!(envelope.msg, "success");
        assert!(envelope.data.is_some());
        assert_eq!(envelope.data.unwrap().value, "test");
    }

    #[test]
    fn test_api_envelope_without_data() {
        // 测试没有 data 字段的响应
        let json = r#"{
            "ret": 200,
            "msg": "success"
        }"#;

        let envelope: ApiEnvelope<TestData> = serde_json::from_str(json).unwrap();
        assert_eq!(envelope.ret, 200);
        assert_eq!(envelope.msg, "success");
        assert!(envelope.data.is_none());
    }

    #[test]
    fn test_empty_base_url() {
        let client = GeweHttpClient::new("token", "").expect("Failed to create client");

        assert_eq!(client.endpoint("api/test"), "/api/test");
    }
}
