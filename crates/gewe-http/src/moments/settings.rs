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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_stranger_visibility_request() {
        let req = StrangerVisibilityRequest {
            app_id: "test_app",
            enabled: true,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("enabled"));
    }

    #[test]
    fn test_set_sns_visible_scope_request() {
        let req = SetSnsVisibleScopeRequest {
            app_id: "test_app",
            option: 0,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("option"));
    }

    #[test]
    fn test_set_sns_privacy_request() {
        let req = SetSnsPrivacyRequest {
            app_id: "test_app",
            sns_id: 123456,
            open: true,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsId"));
        assert!(json.contains("open"));
    }
}
