use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacySettingsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub option: i32,
    pub open: bool,
}

pub type PrivacySettingsResponse = ();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_settings_request_serialization() {
        let req = PrivacySettingsRequest {
            app_id: "test_app",
            option: 1,
            open: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("true"));
    }
}
