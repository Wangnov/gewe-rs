use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteMemberRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    pub reason: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMemberRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinRoomUsingQrCodeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "qrUuid")]
    pub qr_uuid: &'a str,
    #[serde(rename = "chatroomName")]
    pub chatroom_name: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreeJoinRoomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "msgId")]
    pub msg_id: &'a str,
    #[serde(rename = "newMsgId")]
    pub new_msg_id: &'a str,
    #[serde(rename = "createTime")]
    pub create_time: &'a str,
    #[serde(rename = "fromUsername")]
    pub from_username: &'a str,
    #[serde(rename = "toUsername")]
    pub to_username: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomAccessApplyCheckApproveRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    pub wxid: &'a str,
    pub ticket: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteAddEnterRoomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "expId")]
    pub exp_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGroupMemberAsFriendRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "wxid")]
    pub wxid: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomMemberListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomMemberListResponse {
    #[serde(rename = "chatRoomOwner")]
    pub chat_room_owner: String,
    #[serde(rename = "chatroomMembers")]
    pub chatroom_members: Vec<ChatroomMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChatroomMember {
    pub wxid: String,
    #[serde(rename = "inviterUserName")]
    pub inviter_user_name: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(rename = "inviteTicket")]
    pub invite_ticket: String,
    #[serde(rename = "memberFlag")]
    pub member_flag: i64,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "remarkName")]
    pub remark_name: String,
    pub sex: i64,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomMemberDetailRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
    #[serde(rename = "wxid")]
    pub wxid: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomMemberDetailResponse {
    pub wxid: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "remarkName")]
    pub remark_name: String,
    pub sex: i64,
    pub country: String,
    pub province: String,
    pub city: String,
    pub signature: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "chatroomId")]
    pub chatroom_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomInfoResponse {
    #[serde(rename = "chatroomId")]
    pub chatroom_id: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "pyInitial")]
    pub py_initial: String,
    #[serde(rename = "quanPin")]
    pub quan_pin: String,
    pub sex: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(rename = "remarkPyInitial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark_py_initial: Option<String>,
    #[serde(rename = "remarkQuanPin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark_quan_pin: Option<String>,
    #[serde(rename = "chatRoomNotify")]
    pub chat_room_notify: i64,
    #[serde(rename = "chatRoomOwner")]
    pub chat_room_owner: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(rename = "memberList")]
    pub member_list: Vec<GetChatroomInfoMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetChatroomInfoMember {
    pub wxid: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "inviterUserName", skip_serializing_if = "Option::is_none")]
    pub inviter_user_name: Option<String>,
    #[serde(rename = "memberFlag")]
    pub member_flag: i64,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "bigHeadImgUrl", skip_serializing_if = "Option::is_none")]
    pub big_head_img_url: Option<String>,
    #[serde(rename = "smallHeadImgUrl", skip_serializing_if = "Option::is_none")]
    pub small_head_img_url: Option<String>,
}

pub type SimpleGroupResponse = ();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatroom_member_default() {
        let member = ChatroomMember::default();
        assert_eq!(member.wxid, "");
        assert_eq!(member.nick_name, "");
        assert_eq!(member.sex, 0);
    }

    #[test]
    fn test_chatroom_member_deserialization() {
        let json = r#"{
            "wxid": "wxid_test",
            "inviterUserName": "inviter_wxid",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg",
            "inviteTicket": "ticket123",
            "memberFlag": 1,
            "nickName": "TestUser",
            "remarkName": "Remark",
            "sex": 1,
            "userName": "user_test",
            "displayName": "Display Name"
        }"#;
        let member: ChatroomMember = serde_json::from_str(json).unwrap();
        assert_eq!(member.wxid, "wxid_test");
        assert_eq!(member.nick_name, "TestUser");
        assert_eq!(member.sex, 1);
        assert_eq!(member.display_name, "Display Name");
    }

    #[test]
    fn test_get_chatroom_member_list_response_default() {
        let resp = GetChatroomMemberListResponse::default();
        assert_eq!(resp.chat_room_owner, "");
        assert!(resp.chatroom_members.is_empty());
    }

    #[test]
    fn test_get_chatroom_member_detail_response_default() {
        let resp = GetChatroomMemberDetailResponse::default();
        assert_eq!(resp.wxid, "");
        assert_eq!(resp.nick_name, "");
        assert_eq!(resp.country, "");
    }

    #[test]
    fn test_get_chatroom_info_response_default() {
        let resp = GetChatroomInfoResponse::default();
        assert_eq!(resp.chatroom_id, "");
        assert_eq!(resp.nick_name, "");
        assert!(resp.member_list.is_empty());
    }

    #[test]
    fn test_get_chatroom_info_member_default() {
        let member = GetChatroomInfoMember::default();
        assert_eq!(member.wxid, "");
        assert_eq!(member.nick_name, "");
        assert_eq!(member.member_flag, 0);
    }

    #[test]
    fn test_invite_member_request_serialization() {
        let req = InviteMemberRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            reason: "Join us!",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("Join us!"));
    }

    #[test]
    fn test_remove_member_request_serialization() {
        let req = RemoveMemberRequest {
            app_id: "test_app",
            chatroom_id: "room123",
            wxids: vec!["wxid1"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
        assert!(json.contains("wxid1"));
    }

    #[test]
    fn test_get_chatroom_member_list_request_serialization() {
        let req = GetChatroomMemberListRequest {
            app_id: "test_app",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
    }

    #[test]
    fn test_get_chatroom_info_request_serialization() {
        let req = GetChatroomInfoRequest {
            app_id: "test_app",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("room123"));
    }
}
