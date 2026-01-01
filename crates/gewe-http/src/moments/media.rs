use crate::client::GeweHttpClient;
use gewe_core::{
    DownloadSnsVideoRequest, DownloadSnsVideoResponse, GeweError, UploadSnsImageRequest,
    UploadSnsImageResponse, UploadSnsVideoRequest, UploadSnsVideoResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn upload_sns_image(
        &self,
        req: UploadSnsImageRequest<'_>,
    ) -> Result<UploadSnsImageResponse, GeweError> {
        let env = self
            .post_api::<_, UploadSnsImageResponse>("gewe/v2/api/sns/uploadSnsImage", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn upload_sns_video(
        &self,
        req: UploadSnsVideoRequest<'_>,
    ) -> Result<UploadSnsVideoResponse, GeweError> {
        let env = self
            .post_api::<_, UploadSnsVideoResponse>("gewe/v2/api/sns/uploadSnsVideo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_sns_video(
        &self,
        req: DownloadSnsVideoRequest<'_>,
    ) -> Result<DownloadSnsVideoResponse, GeweError> {
        let env = self
            .post_api::<_, DownloadSnsVideoResponse>("gewe/v2/api/sns/downloadSnsVideo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_sns_image_request() {
        let req = UploadSnsImageRequest {
            app_id: "test_app",
            img_urls: vec!["https://example.com/image.jpg"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("imgUrls"));
    }

    #[test]
    fn test_upload_sns_video_request() {
        let req = UploadSnsVideoRequest {
            app_id: "test_app",
            thumb_url: "https://example.com/thumb.jpg",
            video_url: "https://example.com/video.mp4",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("videoUrl"));
        assert!(json.contains("thumbUrl"));
    }

    #[test]
    fn test_download_sns_video_request() {
        let req = DownloadSnsVideoRequest {
            app_id: "test_app",
            sns_xml: "<xml>test</xml>",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsXml"));
    }
}
