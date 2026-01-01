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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_safety_info_request() {
        let req = GetSafetyInfoRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }
}
