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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_finder_request() {
        let req = CreateFinderRequest {
            app_id: "test_app",
            nick_name: "MyFinderNickname",
            head_img: "https://example.com/head.jpg",
            signature: None,
            sex: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("nickName"));
    }

    #[test]
    fn test_update_finder_profile_request() {
        let req = UpdateFinderProfileRequest {
            app_id: "test_app",
            nick_name: Some("NewNickname"),
            head_img: None,
            signature: Some("New signature"),
            sex: None,
            country: None,
            province: None,
            city: None,
            my_user_name: "my_user",
            my_role_type: 1,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("nickName"));
    }

    #[test]
    fn test_get_finder_profile_request() {
        let req = GetFinderProfileRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }

    #[test]
    fn test_user_page_request() {
        let req = UserPageRequest {
            app_id: "test_app",
            to_user_name: "finder_user",
            last_buffer: None,
            max_id: None,
            search_info: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("toUserName"));
    }
}
