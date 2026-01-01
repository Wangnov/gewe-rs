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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_operate_request_serialization() {
        let req = AdminOperateRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            wxid: "wxid_test",
            is_admin: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("wxid_test"));
        assert!(json.contains("true"));
    }
}
