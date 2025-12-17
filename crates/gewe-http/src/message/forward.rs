use crate::client::GeweHttpClient;
use gewe_core::{
    ForwardFileRequest, ForwardFileResponse, ForwardImageRequest, ForwardImageResponse,
    ForwardMiniAppRequest, ForwardMiniAppResponse, ForwardUrlRequest, ForwardUrlResponse,
    ForwardVideoRequest, ForwardVideoResponse, GeweError,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn forward_image(
        &self,
        app_id: &str,
        to_wxid: &str,
        xml: &str,
    ) -> Result<ForwardImageResponse, GeweError> {
        let body = ForwardImageRequest {
            app_id,
            to_wxid,
            xml,
        };
        let env = self
            .post_api::<_, ForwardImageResponse>("gewe/v2/api/message/forwardImage", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn forward_video(
        &self,
        app_id: &str,
        to_wxid: &str,
        xml: &str,
    ) -> Result<ForwardVideoResponse, GeweError> {
        let body = ForwardVideoRequest {
            app_id,
            to_wxid,
            xml,
        };
        let env = self
            .post_api::<_, ForwardVideoResponse>("gewe/v2/api/message/forwardVideo", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn forward_file(
        &self,
        app_id: &str,
        to_wxid: &str,
        xml: &str,
    ) -> Result<ForwardFileResponse, GeweError> {
        let body = ForwardFileRequest {
            app_id,
            to_wxid,
            xml,
        };
        let env = self
            .post_api::<_, ForwardFileResponse>("gewe/v2/api/message/forwardFile", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn forward_mini_app(
        &self,
        app_id: &str,
        to_wxid: &str,
        xml: &str,
        cover_img_url: &str,
    ) -> Result<ForwardMiniAppResponse, GeweError> {
        let body = ForwardMiniAppRequest {
            app_id,
            to_wxid,
            xml,
            cover_img_url,
        };
        let env = self
            .post_api::<_, ForwardMiniAppResponse>("gewe/v2/api/message/forwardMiniApp", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn forward_url(
        &self,
        app_id: &str,
        to_wxid: &str,
        xml: &str,
    ) -> Result<ForwardUrlResponse, GeweError> {
        let body = ForwardUrlRequest {
            app_id,
            to_wxid,
            xml,
        };
        let env = self
            .post_api::<_, ForwardUrlResponse>("gewe/v2/api/message/forwardUrl", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
