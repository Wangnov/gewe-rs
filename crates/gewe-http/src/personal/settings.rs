use crate::client::GeweHttpClient;
use gewe_core::{GeweError, PrivacySettingsRequest};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn privacy_settings(&self, req: PrivacySettingsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/personal/privacySettings", &req)
            .await?;
        Ok(())
    }
}
