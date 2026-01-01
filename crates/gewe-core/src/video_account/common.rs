use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderExtInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderLotterySetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attend_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderLiveInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_status_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lottery_setting: Option<FinderLotterySetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic_setting: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderContactProfile {
    pub username: String,
    pub nickname: String,
    pub head_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_flag: Option<i32>,
    #[serde(default)]
    pub auth_info: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<FinderExtInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_cover_img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_info: Option<FinderLiveInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_follow_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderSearchInfo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<&'a str>,
}

pub type FinderRawData = Value;
