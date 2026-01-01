use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFinderVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "videoUrl")]
    pub video_url: &'a str,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadFinderVideoResponse {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "mp4Identify")]
    pub mp4_identify: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "thumbMD5")]
    pub thumb_md5: String,
    #[serde(rename = "fileKey")]
    pub file_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderVideoCdn {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "mp4Identify")]
    pub mp4_identify: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "thumbMD5")]
    pub thumb_md5: String,
    #[serde(rename = "fileKey")]
    pub file_key: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishFinderCdnRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub topic: Vec<&'a str>,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    pub description: &'a str,
    #[serde(rename = "videoCdn")]
    pub video_cdn: FinderVideoCdn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFinderCdnResponse {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishFinderWebRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub title: &'a str,
    #[serde(rename = "videoUrl")]
    pub video_url: &'a str,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFinderWebResponse {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendFinderSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "allowWxIds")]
    pub allow_wx_ids: Vec<&'a str>,
    #[serde(rename = "atWxIds")]
    pub at_wx_ids: Vec<&'a str>,
    #[serde(rename = "disableWxIds")]
    pub disable_wx_ids: Vec<&'a str>,
    pub id: i64,
    pub username: &'a str,
    pub nickname: &'a str,
    #[serde(rename = "headUrl")]
    pub head_url: &'a str,
    #[serde(rename = "nonceId")]
    pub nonce_id: &'a str,
    #[serde(rename = "mediaType")]
    pub media_type: &'a str,
    pub width: &'a str,
    pub height: &'a str,
    pub url: &'a str,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
    #[serde(rename = "thumbUrlToken")]
    pub thumb_url_token: &'a str,
    pub description: &'a str,
    #[serde(rename = "videoPlayLen")]
    pub video_play_len: &'a str,
}
