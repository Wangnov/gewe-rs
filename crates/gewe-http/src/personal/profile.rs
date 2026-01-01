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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profile_request() {
        let req = GetProfileRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }

    #[test]
    fn test_update_profile_request() {
        let req = UpdateProfileRequest {
            app_id: "test_app",
            nick_name: Some("NewName"),
            country: Some("China"),
            province: Some("Beijing"),
            city: Some("Beijing"),
            sex: Some(1),
            signature: Some("New signature"),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("NewName"));
    }

    #[test]
    fn test_update_head_img_request() {
        let req = UpdateHeadImgRequest {
            app_id: "test_app",
            head_img_url: "https://example.com/avatar.jpg",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("headImgUrl"));
    }

    #[test]
    fn test_get_qr_code_request() {
        let req = GetQrCodeRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }
}
