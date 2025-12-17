use crate::client::GeweHttpClient;
use gewe_core::{
    CreateFinderRequest, CreateFinderResponse, GetFinderProfileRequest, GetFinderQrCodeRequest,
    GetFinderQrCodeResponse, GeweError, UpdateFinderProfileRequest, UserPageRequest,
    UserPageResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn create_finder(
        &self,
        req: CreateFinderRequest<'_>,
    ) -> Result<CreateFinderResponse, GeweError> {
        let env = self
            .post_api::<_, CreateFinderResponse>("gewe/v2/api/finder/createFinder", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn update_finder_profile(
        &self,
        req: UpdateFinderProfileRequest<'_>,
    ) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/updateProfile", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_finder_profile(
        &self,
        req: GetFinderProfileRequest<'_>,
    ) -> Result<gewe_core::FinderProfileInfo, GeweError> {
        let env = self
            .post_api::<_, gewe_core::FinderProfileInfo>("gewe/v2/api/finder/getProfile", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_finder_qr_code(
        &self,
        req: GetFinderQrCodeRequest<'_>,
    ) -> Result<GetFinderQrCodeResponse, GeweError> {
        let env = self
            .post_api::<_, GetFinderQrCodeResponse>("gewe/v2/api/finder/getQrCode", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn user_page(&self, req: UserPageRequest<'_>) -> Result<UserPageResponse, GeweError> {
        let env = self
            .post_api::<_, UserPageResponse>("gewe/v2/api/finder/userPage", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
