use crate::client::GeweHttpClient;
use gewe_core::{DeleteSnsRequest, GeweError};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn delete_sns(&self, req: DeleteSnsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/delSns", &req)
            .await?;
        Ok(())
    }
}
