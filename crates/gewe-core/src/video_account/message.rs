use super::common::FinderRawData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendFinderMsgRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPrivateLetterRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub content: &'a str,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "msgSessionId")]
    pub msg_session_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPrivateLetterImgRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "msgSessionId")]
    pub msg_session_id: &'a str,
    #[serde(rename = "imgUrl")]
    pub img_url: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrivateLetterResponse {
    #[serde(rename = "newMsgId")]
    pub new_msg_id: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncPrivateLetterMsgRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyBuff")]
    pub key_buff: Option<&'a str>,
}

pub type SyncPrivateLetterMsgResponse = FinderRawData;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MentionListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "reqScene")]
    pub req_scene: i32,
    #[serde(rename = "lastBuff")]
    pub last_buff: &'a str,
}

pub type MentionListResponse = FinderRawData;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "queryInfo")]
    pub query_info: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactExtInfo {
    pub country: String,
    pub province: String,
    pub city: String,
    pub sex: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactMsgInfo {
    #[serde(rename = "msgUsername")]
    pub msg_username: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactListEntry {
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[serde(rename = "headUrl")]
    pub head_url: Option<String>,
    pub signature: Option<String>,
    #[serde(rename = "extInfo")]
    pub ext_info: Option<ContactExtInfo>,
    #[serde(rename = "msgInfo")]
    pub msg_info: Option<ContactMsgInfo>,
    #[serde(rename = "wxUsernameV5")]
    pub wx_username_v5: Option<String>,
}
