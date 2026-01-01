use crate::client::GeweHttpClient;
use gewe_core::{
    GeweError, PublishFinderCdnRequest, PublishFinderCdnResponse, PublishFinderWebRequest,
    PublishFinderWebResponse, SendFinderSnsRequest, UploadFinderVideoRequest,
    UploadFinderVideoResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn upload_finder_video(
        &self,
        req: UploadFinderVideoRequest<'_>,
    ) -> Result<UploadFinderVideoResponse, GeweError> {
        let env = self
            .post_api::<_, UploadFinderVideoResponse>("gewe/v2/api/finder/uploadFinderVideo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn publish_finder_cdn(
        &self,
        req: PublishFinderCdnRequest<'_>,
    ) -> Result<PublishFinderCdnResponse, GeweError> {
        let env = self
            .post_api::<_, PublishFinderCdnResponse>("gewe/v2/api/finder/publishFinderCdn", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn publish_finder_web(
        &self,
        req: PublishFinderWebRequest<'_>,
    ) -> Result<PublishFinderWebResponse, GeweError> {
        let env = self
            .post_api::<_, PublishFinderWebResponse>("gewe/v2/api/finder/publishFinderWeb", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_finder_sns(&self, req: SendFinderSnsRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/sns/sendFinderSns", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gewe_core::FinderVideoCdn;

    #[test]
    fn test_upload_finder_video_request() {
        let req = UploadFinderVideoRequest {
            app_id: "test_app",
            video_url: "https://example.com/video.mp4",
            cover_img_url: "https://example.com/cover.jpg",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("videoUrl"));
        assert!(json.contains("coverImgUrl"));
    }

    #[test]
    fn test_publish_finder_cdn_request() {
        let req = PublishFinderCdnRequest {
            app_id: "test_app",
            topic: vec!["topic1"],
            my_user_name: "my_user",
            my_role_type: 1,
            description: "Video description",
            video_cdn: FinderVideoCdn::default(),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("description"));
        assert!(json.contains("myUserName"));
    }

    #[test]
    fn test_publish_finder_web_request() {
        let req = PublishFinderWebRequest {
            app_id: "test_app",
            title: "Video Title",
            video_url: "https://example.com/video.mp4",
            thumb_url: "https://example.com/thumb.jpg",
            description: "Web video",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("videoUrl"));
        assert!(json.contains("title"));
    }
}
