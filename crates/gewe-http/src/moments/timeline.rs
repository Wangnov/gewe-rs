use crate::client::GeweHttpClient;
use gewe_core::{
    GetContactsSnsListRequest, GetContactsSnsListResponse, GetSelfSnsListRequest,
    GetSelfSnsListResponse, GetSnsDetailsRequest, GetSnsDetailsResponse, GeweError,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn get_self_sns_list(
        &self,
        req: GetSelfSnsListRequest<'_>,
    ) -> Result<GetSelfSnsListResponse, GeweError> {
        let env = self
            .post_api::<_, GetSelfSnsListResponse>("gewe/v2/api/sns/snsList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contacts_sns_list(
        &self,
        req: GetContactsSnsListRequest<'_>,
    ) -> Result<GetContactsSnsListResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactsSnsListResponse>("gewe/v2/api/sns/contactsSnsList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_sns_details(
        &self,
        req: GetSnsDetailsRequest<'_>,
    ) -> Result<GetSnsDetailsResponse, GeweError> {
        let env = self
            .post_api::<_, GetSnsDetailsResponse>("gewe/v2/api/sns/snsDetails", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_self_sns_list_request() {
        let req = GetSelfSnsListRequest {
            app_id: "test_app",
            max_id: Some(123456),
            decrypt: None,
            first_page_md5: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("maxId"));
    }

    #[test]
    fn test_get_contacts_sns_list_request() {
        let req = GetContactsSnsListRequest {
            app_id: "test_app",
            wxid: "contact_wxid",
            max_id: None,
            decrypt: None,
            first_page_md5: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("wxid"));
    }

    #[test]
    fn test_get_sns_details_request() {
        let req = GetSnsDetailsRequest {
            app_id: "test_app",
            sns_id: 123456,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsId"));
    }
}
