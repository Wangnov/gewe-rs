use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminOperateRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "wxid")]
    pub wxid: &'a str,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}

pub type SimpleGroupResponse = ();
