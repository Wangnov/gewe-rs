use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BotId(pub String);

impl From<String> for BotId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for BotId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotContext {
    pub app_id: AppId,
    pub token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbackEnvelope<T> {
    pub appid: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiEnvelope<T> {
    pub ret: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

#[derive(Debug, Error)]
pub enum GeweError {
    #[error("http error: {0}")]
    Http(String),
    #[error("api error code={code}: {message}")]
    Api { code: i32, message: String },
    #[error("decode error: {0}")]
    Decode(String),
    #[error("missing data")]
    MissingData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_id_serialization() {
        let app_id = AppId("test_app_id".to_string());
        let json = serde_json::to_string(&app_id).unwrap();
        assert_eq!(json, r#""test_app_id""#);
    }

    #[test]
    fn test_app_id_deserialization() {
        let json = r#""test_app_id""#;
        let app_id: AppId = serde_json::from_str(json).unwrap();
        assert_eq!(app_id.0, "test_app_id");
    }

    #[test]
    fn test_bot_id_from_string() {
        let bot_id: BotId = "test_bot_id".to_string().into();
        assert_eq!(bot_id.0, "test_bot_id");
    }

    #[test]
    fn test_bot_id_from_str() {
        let bot_id: BotId = "test_bot_id".into();
        assert_eq!(bot_id.0, "test_bot_id");
    }

    #[test]
    fn test_bot_id_serialization() {
        let bot_id = BotId("test_bot_id".to_string());
        let json = serde_json::to_string(&bot_id).unwrap();
        assert_eq!(json, r#""test_bot_id""#);
    }

    #[test]
    fn test_bot_id_deserialization() {
        let json = r#""test_bot_id""#;
        let bot_id: BotId = serde_json::from_str(json).unwrap();
        assert_eq!(bot_id.0, "test_bot_id");
    }

    #[test]
    fn test_bot_context_serialization() {
        let ctx = BotContext {
            app_id: AppId("app123".to_string()),
            token: "token123".to_string(),
            webhook_secret: Some("secret123".to_string()),
            description: Some("Test bot".to_string()),
        };
        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("app123"));
        assert!(json.contains("token123"));
        assert!(json.contains("secret123"));
        assert!(json.contains("Test bot"));
    }

    #[test]
    fn test_bot_context_deserialization() {
        let json = r#"{
            "appId": "app123",
            "token": "token123",
            "webhookSecret": "secret123",
            "description": "Test bot"
        }"#;
        let ctx: BotContext = serde_json::from_str(json).unwrap();
        assert_eq!(ctx.app_id.0, "app123");
        assert_eq!(ctx.token, "token123");
        assert_eq!(ctx.webhook_secret, Some("secret123".to_string()));
        assert_eq!(ctx.description, Some("Test bot".to_string()));
    }

    #[test]
    fn test_bot_context_optional_fields() {
        let json = r#"{
            "appId": "app123",
            "token": "token123"
        }"#;
        let ctx: BotContext = serde_json::from_str(json).unwrap();
        assert_eq!(ctx.app_id.0, "app123");
        assert_eq!(ctx.token, "token123");
        assert_eq!(ctx.webhook_secret, None);
        assert_eq!(ctx.description, None);
    }

    #[test]
    fn test_callback_envelope_serialization() {
        let envelope = CallbackEnvelope {
            appid: "app123".to_string(),
            data: "test_data".to_string(),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("app123"));
        assert!(json.contains("test_data"));
    }

    #[test]
    fn test_callback_envelope_deserialization() {
        let json = r#"{"appid":"app123","data":"test_data"}"#;
        let envelope: CallbackEnvelope<String> = serde_json::from_str(json).unwrap();
        assert_eq!(envelope.appid, "app123");
        assert_eq!(envelope.data, "test_data");
    }

    #[test]
    fn test_api_envelope_with_data() {
        let envelope = ApiEnvelope {
            ret: 200,
            msg: "success".to_string(),
            data: Some("result".to_string()),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("200"));
        assert!(json.contains("success"));
        assert!(json.contains("result"));
    }

    #[test]
    fn test_api_envelope_without_data() {
        let json = r#"{"ret":200,"msg":"success"}"#;
        let envelope: ApiEnvelope<String> = serde_json::from_str(json).unwrap();
        assert_eq!(envelope.ret, 200);
        assert_eq!(envelope.msg, "success");
        assert_eq!(envelope.data, None);
    }

    #[test]
    fn test_gewe_error_http() {
        let err = GeweError::Http("connection failed".to_string());
        assert_eq!(err.to_string(), "http error: connection failed");
    }

    #[test]
    fn test_gewe_error_api() {
        let err = GeweError::Api {
            code: 404,
            message: "not found".to_string(),
        };
        assert_eq!(err.to_string(), "api error code=404: not found");
    }

    #[test]
    fn test_gewe_error_decode() {
        let err = GeweError::Decode("invalid json".to_string());
        assert_eq!(err.to_string(), "decode error: invalid json");
    }

    #[test]
    fn test_gewe_error_missing_data() {
        let err = GeweError::MissingData;
        assert_eq!(err.to_string(), "missing data");
    }
}
