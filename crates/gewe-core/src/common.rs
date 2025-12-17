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
