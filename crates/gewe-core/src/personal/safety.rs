use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSafetyInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SafetyDeviceRecord {
    pub uuid: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "lastTime")]
    pub last_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSafetyInfoResponse {
    pub list: Vec<SafetyDeviceRecord>,
}
