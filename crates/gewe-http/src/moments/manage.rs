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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_sns_request() {
        let req = DeleteSnsRequest {
            app_id: "test_app",
            sns_id: 123456,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsId"));
    }
}
