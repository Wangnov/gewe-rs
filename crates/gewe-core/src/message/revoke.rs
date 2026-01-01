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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revoke_message_request_serialization() {
        let req = RevokeMessageRequest {
            app_id: "test_app",
            to_wxid: "wxid_test",
            msg_id: "123456",
            new_msg_id: "789012",
            create_time: "1234567890",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid_test"));
        assert!(json.contains("123456"));
        assert!(json.contains("789012"));
        assert!(json.contains("1234567890"));
    }
}
