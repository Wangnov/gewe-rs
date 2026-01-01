use crate::client::GeweHttpClient;
use gewe_core::{
    AddGroupMemberAsFriendRequest, AgreeJoinRoomRequest, GetChatroomInfoRequest,
    GetChatroomInfoResponse, GetChatroomMemberDetailRequest, GetChatroomMemberDetailResponse,
    GetChatroomMemberListRequest, GetChatroomMemberListResponse, InviteAddEnterRoomRequest,
    InviteMemberRequest, JoinRoomUsingQrCodeRequest, RemoveMemberRequest,
    RoomAccessApplyCheckApproveRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn invite_member(
        &self,
        req: InviteMemberRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/inviteMember", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn remove_member(
        &self,
        req: RemoveMemberRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/removeMember", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn join_room_using_qr_code(
        &self,
        req: JoinRoomUsingQrCodeRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/joinRoomUsingQRCode", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn agree_join_room(
        &self,
        req: AgreeJoinRoomRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/agreeJoinRoom", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn room_access_apply_check_approve(
        &self,
        req: RoomAccessApplyCheckApproveRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/roomAccessApplyCheckApprove", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn invite_add_enter_room(
        &self,
        req: InviteAddEnterRoomRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/inviteAddEnterRoom", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn add_group_member_as_friend(
        &self,
        req: AddGroupMemberAsFriendRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/addGroupMemberAsFriend", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_chatroom_member_list(
        &self,
        req: GetChatroomMemberListRequest<'_>,
    ) -> Result<GetChatroomMemberListResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, GetChatroomMemberListResponse>(
                "gewe/v2/api/group/getChatroomMemberList",
                &req,
            )
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_chatroom_member_detail(
        &self,
        req: GetChatroomMemberDetailRequest<'_>,
    ) -> Result<GetChatroomMemberDetailResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, GetChatroomMemberDetailResponse>(
                "gewe/v2/api/group/getChatroomMemberDetail",
                &req,
            )
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_chatroom_info(
        &self,
        req: GetChatroomInfoRequest<'_>,
    ) -> Result<GetChatroomInfoResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, GetChatroomInfoResponse>("gewe/v2/api/group/getChatroomInfo", &req)
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invite_member_request_serialization() {
        let req = InviteMemberRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
            reason: "Join us!",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("wxids"));
        assert!(json.contains("reason"));
    }

    #[test]
    fn test_remove_member_request_serialization() {
        let req = RemoveMemberRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
            wxids: vec!["wxid1"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
    }

    #[test]
    fn test_get_chatroom_member_list_request_serialization() {
        let req = GetChatroomMemberListRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
    }

    #[test]
    fn test_get_chatroom_info_request_serialization() {
        let req = GetChatroomInfoRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
    }
}
