use crate::client::GeweHttpClient;
use gewe_core::{GetSafetyInfoRequest, GetSafetyInfoResponse, GeweError};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn get_safety_info(
        &self,
        req: GetSafetyInfoRequest<'_>,
    ) -> Result<GetSafetyInfoResponse, GeweError> {
        let env = self
            .post_api::<_, GetSafetyInfoResponse>("gewe/v2/api/personal/getSafetyInfo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
