use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardImageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub xml: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForwardImageResponse {
    pub to_wxid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<i32>,
    pub aes_key: String,
    pub file_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub xml: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForwardVideoResponse {
    pub to_wxid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<i32>,
    pub aes_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardFileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub xml: &'a str,
}

pub type ForwardFileResponse = super::SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardMiniAppRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub xml: &'a str,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: &'a str,
}

pub type ForwardMiniAppResponse = super::SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardUrlRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub xml: &'a str,
}

pub type ForwardUrlResponse = super::SendTextResponse;
