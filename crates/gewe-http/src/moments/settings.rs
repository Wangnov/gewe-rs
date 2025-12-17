use crate::client::GeweHttpClient;
use gewe_core::{
    GeweError, SetSnsPrivacyRequest, SetSnsVisibleScopeRequest, StrangerVisibilityRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn set_stranger_visibility(
        &self,
        req: StrangerVisibilityRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/strangerVisibilityEnabled", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_sns_visible_scope(
        &self,
        req: SetSnsVisibleScopeRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/snsVisibleScope", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_sns_privacy(&self, req: SetSnsPrivacyRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/snsSetPrivacy", &req)
            .await?;
        Ok(())
    }
}
