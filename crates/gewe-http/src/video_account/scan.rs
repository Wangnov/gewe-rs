use crate::client::GeweHttpClient;
use gewe_core::{
    GeweError, ScanBrowseRequest, ScanCommentRequest, ScanCommentResponse, ScanFavRequest,
    ScanFollowRequest, ScanFollowResponse, ScanLikeRequest, ScanLoginChannelsRequest,
    ScanLoginChannelsResponse, ScanQrCodeRequest, ScanQrCodeResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn scan_follow(
        &self,
        req: ScanFollowRequest<'_>,
    ) -> Result<ScanFollowResponse, GeweError> {
        let env = self
            .post_api::<_, ScanFollowResponse>("gewe/v2/api/finder/scanFollow", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn scan_browse(&self, req: ScanBrowseRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/scanBrowse", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn scan_like(&self, req: ScanLikeRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/scanLike", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn scan_fav(&self, req: ScanFavRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/scanFav", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn scan_comment(
        &self,
        req: ScanCommentRequest<'_>,
    ) -> Result<ScanCommentResponse, GeweError> {
        let env = self
            .post_api::<_, ScanCommentResponse>("gewe/v2/api/finder/scanComment", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn scan_qr_code(
        &self,
        req: ScanQrCodeRequest<'_>,
    ) -> Result<ScanQrCodeResponse, GeweError> {
        let env = self
            .post_api::<_, ScanQrCodeResponse>("gewe/v2/api/finder/scanQrCode", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn scan_login_channels(
        &self,
        req: ScanLoginChannelsRequest<'_>,
    ) -> Result<ScanLoginChannelsResponse, GeweError> {
        let env = self
            .post_api::<_, ScanLoginChannelsResponse>("gewe/v2/api/finder/scanLoginChannels", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
