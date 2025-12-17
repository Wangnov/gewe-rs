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
