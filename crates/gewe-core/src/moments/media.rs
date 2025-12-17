use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSnsImageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "imgUrls")]
    pub img_urls: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadedSnsImage {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "fileMd5")]
    pub file_md5: String,
    pub length: i64,
    pub width: i64,
    pub height: i64,
}

pub type UploadSnsImageResponse = Vec<UploadedSnsImage>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSnsVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
    #[serde(rename = "videoUrl")]
    pub video_url: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadSnsVideoResponse {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "fileMd5")]
    pub file_md5: String,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSnsVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "snsXml")]
    pub sns_xml: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSnsVideoResponse {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
}
