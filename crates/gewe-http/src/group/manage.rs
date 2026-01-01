use crate::client::GeweHttpClient;
use gewe_core::{
    CreateChatroomRequest, CreateChatroomResponse, DisbandChatroomRequest,
    ModifyChatroomNameRequest, ModifyChatroomNickNameForSelfRequest, ModifyChatroomRemarkRequest,
    QuitChatroomRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn create_chatroom(
        &self,
        req: CreateChatroomRequest<'_>,
    ) -> Result<CreateChatroomResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, CreateChatroomResponse>("gewe/v2/api/group/createChatroom", &req)
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn disband_chatroom(
        &self,
        req: DisbandChatroomRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/disbandChatroom", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn quit_chatroom(
        &self,
        req: QuitChatroomRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/quitChatroom", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn modify_chatroom_name(
        &self,
        req: ModifyChatroomNameRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/modifyChatroomName", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn modify_chatroom_remark(
        &self,
        req: ModifyChatroomRemarkRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/modifyChatroomRemark", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn modify_chatroom_nick_name_for_self(
        &self,
        req: ModifyChatroomNickNameForSelfRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/modifyChatroomNickNameForSelf", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chatroom_request_serialization() {
        let req = CreateChatroomRequest {
            app_id: "test_app",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("wxids"));
        assert!(json.contains("wxid1"));
    }

    #[test]
    fn test_disband_chatroom_request_serialization() {
        let req = DisbandChatroomRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
    }

    #[test]
    fn test_quit_chatroom_request_serialization() {
        let req = QuitChatroomRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
    }

    #[test]
    fn test_modify_chatroom_name_request_serialization() {
        let req = ModifyChatroomNameRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
            chatroom_name: "New Group Name",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomId"));
        assert!(json.contains("New Group Name"));
    }

    #[test]
    fn test_modify_chatroom_remark_request_serialization() {
        let req = ModifyChatroomRemarkRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
            chatroom_remark: "Group Remark",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("chatroomRemark"));
    }

    #[test]
    fn test_modify_chatroom_nick_name_for_self_request_serialization() {
        let req = ModifyChatroomNickNameForSelfRequest {
            app_id: "test_app",
            chatroom_id: "chatroom_123",
            nick_name: "My Nickname",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("nickName"));
        assert!(json.contains("My Nickname"));
    }
}
