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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chatroom_announcement_response_default() {
        let resp = GetChatroomAnnouncementResponse::default();
        assert_eq!(resp.chat_room_announcement, "");
        assert_eq!(resp.sender, "");
        assert_eq!(resp.create_time, "");
        assert_eq!(resp.expire_time, "");
    }

    #[test]
    fn test_get_chatroom_qr_code_response_default() {
        let resp = GetChatroomQrCodeResponse::default();
        assert_eq!(resp.qr_img_base64, "");
        assert_eq!(resp.head_img_base64, "");
        assert_eq!(resp.qr_url, "");
    }

    #[test]
    fn test_set_chatroom_announcement_request_serialization() {
        let req = SetChatroomAnnouncementRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            content: "Important announcement",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("Important announcement"));
    }

    #[test]
    fn test_get_chatroom_announcement_request_serialization() {
        let req = GetChatroomAnnouncementRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            r#type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_pin_chat_request_serialization() {
        let req = PinChatRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            add: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_set_msg_silence_request_serialization() {
        let req = SetMsgSilenceRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            switch_: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("false"));
    }
}
