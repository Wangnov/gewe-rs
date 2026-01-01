use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadImageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub xml: &'a str,
    #[serde(rename = "type")]
    pub image_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadImageResponse {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub xml: &'a str,
}

pub type DownloadVideoResponse = DownloadImageResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub xml: &'a str,
}

pub type DownloadFileResponse = DownloadImageResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadVoiceRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub xml: &'a str,
    #[serde(rename = "msgId")]
    pub msg_id: i64,
}

pub type DownloadVoiceResponse = DownloadImageResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEmojiRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "emojiMd5")]
    pub emoji_md5: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEmojiResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadCdnRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "aesKey")]
    pub aes_key: &'a str,
    #[serde(rename = "fileId")]
    pub file_id: &'a str,
    #[serde(rename = "type")]
    pub r#type: &'a str,
    #[serde(rename = "totalSize")]
    pub total_size: &'a str,
    pub suffix: &'a str,
}

pub type DownloadCdnResponse = DownloadImageResponse;
