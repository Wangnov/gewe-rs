use crate::client::GeweHttpClient;
use gewe_core::{
    GetChatroomAnnouncementRequest, GetChatroomAnnouncementResponse, GetChatroomQrCodeRequest,
    GetChatroomQrCodeResponse, PinChatRequest, SaveContractListRequest,
    SetChatroomAnnouncementRequest, SetMsgSilenceRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn set_chatroom_announcement(
        &self,
        req: SetChatroomAnnouncementRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/setChatroomAnnouncement", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_chatroom_announcement(
        &self,
        req: GetChatroomAnnouncementRequest<'_>,
    ) -> Result<GetChatroomAnnouncementResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, GetChatroomAnnouncementResponse>(
                "gewe/v2/api/group/getChatroomAnnouncement",
                &req,
            )
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_chatroom_qr_code(
        &self,
        req: GetChatroomQrCodeRequest<'_>,
    ) -> Result<GetChatroomQrCodeResponse, gewe_core::GeweError> {
        let env = self
            .post_api::<_, GetChatroomQrCodeResponse>("gewe/v2/api/group/getChatroomQrCode", &req)
            .await?;
        env.data.ok_or(gewe_core::GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn save_contract_list(
        &self,
        req: SaveContractListRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/saveContractList", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn pin_chat(&self, req: PinChatRequest<'_>) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/pinChat", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_msg_silence(
        &self,
        req: SetMsgSilenceRequest<'_>,
    ) -> Result<(), gewe_core::GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/setMsgSilence", &req)
            .await?;
        Ok(())
    }
}
