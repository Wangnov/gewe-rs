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
