use crate::client::GeweHttpClient;
use gewe_core::{AdminOperateRequest, GeweError};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn admin_operate(&self, req: AdminOperateRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/adminOperate", &req)
            .await?;
        Ok(())
    }
}
