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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_device_record_default() {
        let record = SafetyDeviceRecord::default();
        assert_eq!(record.uuid, "");
        assert_eq!(record.device_name, "");
        assert_eq!(record.device_type, "");
        assert_eq!(record.last_time, 0);
    }

    #[test]
    fn test_safety_device_record_deserialization() {
        let json = r#"{
            "uuid": "device_uuid",
            "deviceName": "iPhone 12",
            "deviceType": "mobile",
            "lastTime": 1234567890
        }"#;
        let record: SafetyDeviceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.uuid, "device_uuid");
        assert_eq!(record.device_name, "iPhone 12");
        assert_eq!(record.device_type, "mobile");
        assert_eq!(record.last_time, 1234567890);
    }

    #[test]
    fn test_get_safety_info_response_default() {
        let resp = GetSafetyInfoResponse::default();
        assert!(resp.list.is_empty());
    }

    #[test]
    fn test_get_safety_info_request_serialization() {
        let req = GetSafetyInfoRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }
}
