use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTextRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ats: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendTextResponse {
    pub to_wxid: String,
    pub create_time: i64,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type")]
    pub msg_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostImageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "imgUrl")]
    pub img_url: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostImageResponse {
    pub to_wxid: String,
    pub create_time: i64,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    pub aes_key: String,
    pub file_id: String,
    pub length: i64,
    pub width: i64,
    pub height: i64,
    pub md5: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostVoiceRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "voiceUrl")]
    pub voice_url: &'a str,
    #[serde(rename = "voiceDuration")]
    pub voice_duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostVoiceResponse {
    pub to_wxid: String,
    pub create_time: i64,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostVideoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "videoUrl")]
    pub video_url: &'a str,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
    #[serde(rename = "videoDuration")]
    pub video_duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostVideoResponse {
    pub to_wxid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<i32>,
    pub aes_key: String,
    pub file_id: String,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "fileUrl")]
    pub file_url: &'a str,
    #[serde(rename = "fileName")]
    pub file_name: &'a str,
}

pub type PostFileResponse = SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostLinkRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub title: &'a str,
    pub desc: &'a str,
    #[serde(rename = "linkUrl")]
    pub link_url: &'a str,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
}

pub type PostLinkResponse = SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostEmojiRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "emojiMd5")]
    pub emoji_md5: &'a str,
    #[serde(rename = "emojiSize")]
    pub emoji_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostEmojiResponse {
    pub to_wxid: String,
    pub create_time: i64,
    pub msg_id: i64,
    pub new_msg_id: i64,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAppMsgRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    pub appmsg: &'a str,
}

pub type PostAppMsgResponse = SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMiniAppRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "miniAppId")]
    pub mini_app_id: &'a str,
    #[serde(rename = "displayName")]
    pub display_name: &'a str,
    #[serde(rename = "pagePath")]
    pub page_path: &'a str,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: &'a str,
    pub title: &'a str,
    #[serde(rename = "userName")]
    pub user_name: &'a str,
}

pub type PostMiniAppResponse = SendTextResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostNameCardRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "nickName")]
    pub nick_name: &'a str,
    #[serde(rename = "nameCardWxid")]
    pub name_card_wxid: &'a str,
}

pub type PostNameCardResponse = SendTextResponse;
