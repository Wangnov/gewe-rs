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
