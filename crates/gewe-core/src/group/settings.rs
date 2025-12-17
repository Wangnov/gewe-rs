use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChatroomAnnouncementRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "content")]
    pub content: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomAnnouncementRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "type")]
    pub r#type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomAnnouncementResponse {
    #[serde(rename = "chatRoomAnnouncement")]
    pub chat_room_announcement: String,
    pub sender: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "expireTime")]
    pub expire_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomQrCodeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomQrCodeResponse {
    #[serde(rename = "qrImgBase64")]
    pub qr_img_base64: String,
    #[serde(rename = "headImgBase64")]
    pub head_img_base64: String,
    #[serde(rename = "qrUrl")]
    pub qr_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveContractListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "save")]
    pub save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinChatRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    pub add: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMsgSilenceRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "switch")]
    pub switch_: bool,
}

pub type SimpleGroupResponse = ();
