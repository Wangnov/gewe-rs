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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chatroom_response_default() {
        let resp = CreateChatroomResponse::default();
        assert_eq!(resp.chatroom_id, "");
        assert_eq!(resp.head_img_base64, None);
    }

    #[test]
    fn test_create_chatroom_response_deserialization() {
        let json = r#"{
            "chatroomId": "room123",
            "headImgBase64": "base64_data"
        }"#;
        let resp: CreateChatroomResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chatroom_id, "room123");
        assert_eq!(resp.head_img_base64, Some("base64_data".to_string()));
    }

    #[test]
    fn test_create_chatroom_request_serialization() {
        let req = CreateChatroomRequest {
            app_id: "test_app",
            wxids: vec!["wxid1", "wxid2", "wxid3"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid1"));
        assert!(json.contains("wxid2"));
        assert!(json.contains("wxid3"));
    }

    #[test]
    fn test_disband_chatroom_request_serialization() {
        let req = DisbandChatroomRequest {
            app_id: "test_app",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_quit_chatroom_request_serialization() {
        let req = QuitChatroomRequest {
            app_id: "test_app",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_modify_chatroom_name_request_serialization() {
        let req = ModifyChatroomNameRequest {
            app_id: "test_app",
            chatroom_name: "New Room Name",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("New Room Name"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_modify_chatroom_remark_request_serialization() {
        let req = ModifyChatroomRemarkRequest {
            app_id: "test_app",
            chatroom_remark: "Important Group",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("Important Group"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_modify_chatroom_nickname_for_self_request_serialization() {
        let req = ModifyChatroomNickNameForSelfRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            nick_name: "My Nickname",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("My Nickname"));
    }
}
