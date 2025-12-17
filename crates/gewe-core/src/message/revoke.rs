use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeMessageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toWxid")]
    pub to_wxid: &'a str,
    #[serde(rename = "msgId")]
    pub msg_id: &'a str,
    #[serde(rename = "newMsgId")]
    pub new_msg_id: &'a str,
    #[serde(rename = "createTime")]
    pub create_time: &'a str,
}
