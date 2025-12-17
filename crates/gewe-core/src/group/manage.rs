use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatroomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateChatroomResponse {
    #[serde(default)]
    pub head_img_base64: Option<String>,
    pub chatroom_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisbandChatroomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuitChatroomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyChatroomNameRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomName")]
    pub chatroom_name: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyChatroomRemarkRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomRemark")]
    pub chatroom_remark: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyChatroomNickNameForSelfRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "nickName")]
    pub nick_name: &'a str,
}

pub type SimpleGroupResponse = ();
