use crate::client::GeweHttpClient;
use gewe_core::{
    GetProfileRequest, GetProfileResponse, GetQrCodeRequest, GetQrCodeResponse, GeweError,
    UpdateHeadImgRequest, UpdateProfileRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn get_profile(
        &self,
        req: GetProfileRequest<'_>,
    ) -> Result<GetProfileResponse, GeweError> {
        let env = self
            .post_api::<_, GetProfileResponse>("gewe/v2/api/personal/getProfile", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn update_profile(&self, req: UpdateProfileRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/personal/updateProfile", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn update_head_img(&self, req: UpdateHeadImgRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/personal/updateHeadImg", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_qr_code(
        &self,
        req: GetQrCodeRequest<'_>,
    ) -> Result<GetQrCodeResponse, GeweError> {
        let env = self
            .post_api::<_, GetQrCodeResponse>("gewe/v2/api/personal/getQrCode", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
