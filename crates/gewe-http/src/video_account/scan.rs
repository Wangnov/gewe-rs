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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_follow_request() {
        let req = ScanFollowRequest {
            app_id: "test_app",
            proxy_ip: "",
            my_user_name: "my_user",
            my_role_type: 1,
            qr_content: "qr_content",
            object_id: "obj_123",
            object_nonce_id: "nonce_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("myUserName"));
        assert!(json.contains("qrContent"));
    }

    #[test]
    fn test_scan_like_request() {
        let req = ScanLikeRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            qr_content: "qr_content",
            object_id: 123456,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("objectId"));
    }

    #[test]
    fn test_scan_comment_request() {
        let req = ScanCommentRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            qr_content: "qr_content",
            object_id: 123456,
            comment_content: "Comment content",
            reply_username: None,
            ref_comment_id: None,
            root_comment_id: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("commentContent"));
    }

    #[test]
    fn test_scan_qr_code_request() {
        let req = ScanQrCodeRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            qr_content: "qr_code_data",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("qrContent"));
    }
}
