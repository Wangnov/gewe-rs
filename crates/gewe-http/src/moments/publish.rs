use crate::client::GeweHttpClient;
use gewe_core::{
    ForwardSnsRequest, GeweError, SendImgSnsRequest, SendSnsResponse, SendTextSnsRequest,
    SendUrlSnsRequest, SendVideoSnsRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn send_text_sns(
        &self,
        req: SendTextSnsRequest<'_>,
    ) -> Result<SendSnsResponse, GeweError> {
        let env = self
            .post_api::<_, SendSnsResponse>("gewe/v2/api/sns/sendTextSns", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_img_sns(
        &self,
        req: SendImgSnsRequest<'_>,
    ) -> Result<SendSnsResponse, GeweError> {
        let env = self
            .post_api::<_, SendSnsResponse>("gewe/v2/api/sns/sendImgSns", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_video_sns(
        &self,
        req: SendVideoSnsRequest<'_>,
    ) -> Result<SendSnsResponse, GeweError> {
        let env = self
            .post_api::<_, SendSnsResponse>("gewe/v2/api/sns/sendVideoSns", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_url_sns(
        &self,
        req: SendUrlSnsRequest<'_>,
    ) -> Result<SendSnsResponse, GeweError> {
        let env = self
            .post_api::<_, SendSnsResponse>("gewe/v2/api/sns/sendUrlSns", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn forward_sns(
        &self,
        req: ForwardSnsRequest<'_>,
    ) -> Result<SendSnsResponse, GeweError> {
        let env = self
            .post_api::<_, SendSnsResponse>("gewe/v2/api/sns/forwardSns", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gewe_core::{SnsAudience, SnsImageInfo, SnsVideoInfo};

    #[test]
    fn test_send_text_sns_request() {
        let req = SendTextSnsRequest {
            app_id: "test_app",
            audience: SnsAudience::default(),
            content: "Hello moments!",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("Hello moments!"));
    }

    #[test]
    fn test_send_img_sns_request() {
        let req = SendImgSnsRequest {
            app_id: "test_app",
            audience: SnsAudience::default(),
            img_infos: vec![SnsImageInfo {
                file_url: "https://example.com/image.jpg".to_string(),
                thumb_url: "https://example.com/thumb.jpg".to_string(),
                file_md5: "abc123".to_string(),
                length: Some(1024),
                width: 800,
                height: 600,
            }],
            content: Some("Image caption"),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("imgInfos"));
    }

    #[test]
    fn test_send_video_sns_request() {
        let req = SendVideoSnsRequest {
            app_id: "test_app",
            audience: SnsAudience::default(),
            content: Some("Video caption"),
            video_info: SnsVideoInfo {
                file_url: "https://example.com/video.mp4".to_string(),
                thumb_url: "https://example.com/thumb.jpg".to_string(),
                file_md5: "abc123".to_string(),
                length: Some(1024),
            },
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("videoInfo"));
    }
}
