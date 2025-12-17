use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncFavorRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "syncKey", skip_serializing_if = "Option::is_none")]
    pub sync_key: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FavorSummary {
    #[serde(rename = "favId")]
    pub fav_id: i64,
    #[serde(rename = "type")]
    pub favor_type: i32,
    pub flag: i32,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncFavorResponse {
    #[serde(rename = "syncKey")]
    pub sync_key: String,
    pub list: Vec<FavorSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFavorContentRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "favId")]
    pub fav_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFavorContentResponse {
    #[serde(rename = "favId")]
    pub fav_id: i64,
    pub status: i32,
    pub flag: i32,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFavorRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "favId")]
    pub fav_id: i64,
}
