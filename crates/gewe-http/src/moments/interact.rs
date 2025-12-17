use crate::client::GeweHttpClient;
use gewe_core::{CommentSnsRequest, GeweError, LikeSnsRequest};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn like_sns(&self, req: LikeSnsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/likeSns", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn comment_sns(&self, req: CommentSnsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/commentSns", &req)
            .await?;
        Ok(())
    }
}
