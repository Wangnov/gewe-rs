use crate::client::GeweHttpClient;
use gewe_core::{
    CheckRelationRequest, CheckRelationResponse, FetchContactsListCacheRequest,
    FetchContactsListRequest, FetchContactsListResponse, GetContactBriefInfoRequest,
    GetContactBriefInfoResponse, GetContactDetailInfoRequest, GetContactDetailInfoResponse,
    GetPhoneAddressListRequest, GetPhoneAddressListResponse, GeweError, SearchContactsRequest,
    SearchContactsResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn fetch_contacts_list(
        &self,
        req: FetchContactsListRequest<'_>,
    ) -> Result<FetchContactsListResponse, GeweError> {
        let env = self
            .post_api::<_, FetchContactsListResponse>(
                "gewe/v2/api/contacts/fetchContactsList",
                &req,
            )
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn fetch_contacts_list_cache(
        &self,
        req: FetchContactsListCacheRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/fetchContactsListCache", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn search_contacts(
        &self,
        req: SearchContactsRequest<'_>,
    ) -> Result<SearchContactsResponse, GeweError> {
        let env = self
            .post_api::<_, SearchContactsResponse>("gewe/v2/api/contacts/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contact_brief_info(
        &self,
        req: GetContactBriefInfoRequest<'_>,
    ) -> Result<GetContactBriefInfoResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactBriefInfoResponse>("gewe/v2/api/contacts/getBriefInfo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contact_detail_info(
        &self,
        req: GetContactDetailInfoRequest<'_>,
    ) -> Result<GetContactDetailInfoResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactDetailInfoResponse>("gewe/v2/api/contacts/getDetailInfo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_phone_address_list(
        &self,
        req: GetPhoneAddressListRequest<'_>,
    ) -> Result<GetPhoneAddressListResponse, GeweError> {
        let env = self
            .post_api::<_, GetPhoneAddressListResponse>(
                "gewe/v2/api/contacts/getPhoneAddressList",
                &req,
            )
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn check_contact_relation(
        &self,
        req: CheckRelationRequest<'_>,
    ) -> Result<CheckRelationResponse, GeweError> {
        let env = self
            .post_api::<_, CheckRelationResponse>("gewe/v2/api/contacts/checkRelation", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contacts_list_request_serialization() {
        let req = FetchContactsListRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_fetch_contacts_list_cache_request_serialization() {
        let req = FetchContactsListCacheRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_search_contacts_request_serialization() {
        let req = SearchContactsRequest {
            app_id: "test_app",
            contacts_info: "search_keyword",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("contactsInfo"));
        assert!(json.contains("search_keyword"));
    }

    #[test]
    fn test_get_contact_brief_info_request_serialization() {
        let req = GetContactBriefInfoRequest {
            app_id: "test_app",
            wxids: vec!["wxid1", "wxid2", "wxid3"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxids"));
        assert!(json.contains("wxid1"));
        assert!(json.contains("wxid2"));
    }

    #[test]
    fn test_get_contact_brief_info_request_empty_wxids() {
        let req = GetContactBriefInfoRequest {
            app_id: "test_app",
            wxids: vec![],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("wxids"));
        assert!(json.contains("[]"));
    }

    #[test]
    fn test_get_contact_detail_info_request_serialization() {
        let req = GetContactDetailInfoRequest {
            app_id: "test_app",
            wxids: vec!["target_wxid"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxids"));
        assert!(json.contains("target_wxid"));
    }

    #[test]
    fn test_get_phone_address_list_request_serialization() {
        let req = GetPhoneAddressListRequest {
            app_id: "test_app",
            phones: Some(vec!["+1234567890"]),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_check_relation_request_serialization() {
        let req = CheckRelationRequest {
            app_id: "test_app",
            wxids: vec!["check_wxid"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxids"));
        assert!(json.contains("check_wxid"));
    }

    #[test]
    fn test_search_contacts_with_unicode() {
        let req = SearchContactsRequest {
            app_id: "测试应用",
            contacts_info: "搜索内容",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("测试应用"));
        assert!(json.contains("搜索内容"));
    }

    #[test]
    fn test_get_contact_brief_info_with_special_chars() {
        let req = GetContactBriefInfoRequest {
            app_id: "app-123_test",
            wxids: vec!["wxid_with_underscore", "wxid-with-dash"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("app-123_test"));
        assert!(json.contains("wxid_with_underscore"));
        assert!(json.contains("wxid-with-dash"));
    }
}
