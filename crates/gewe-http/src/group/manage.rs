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
