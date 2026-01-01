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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_settings_request() {
        let req = PrivacySettingsRequest {
            app_id: "test_app",
            option: 1,
            open: true,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("option"));
        assert!(json.contains("open"));
    }
}
