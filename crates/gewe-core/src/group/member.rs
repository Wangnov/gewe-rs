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
