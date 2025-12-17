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
