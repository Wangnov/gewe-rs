use crate::client::GeweHttpClient;
use gewe_core::{
    AddWecomContactRequest, GetWecomContactDetailRequest, GetWecomContactDetailResponse, GeweError,
    SearchWecomRequest, SearchWecomResponse, SyncWecomContactsRequest, SyncWecomContactsResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn search_wecom_contact(
        &self,
        req: SearchWecomRequest<'_>,
    ) -> Result<SearchWecomResponse, GeweError> {
        let env = self
            .post_api::<_, SearchWecomResponse>("gewe/v2/api/im/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn sync_wecom_contacts(
        &self,
        req: SyncWecomContactsRequest<'_>,
    ) -> Result<SyncWecomContactsResponse, GeweError> {
        let env = self
            .post_api::<_, SyncWecomContactsResponse>("gewe/v2/api/im/sync", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn add_wecom_contact(
        &self,
        req: AddWecomContactRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self.post_api::<_, ()>("gewe/v2/api/im/add", &req).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_wecom_contact_detail(
        &self,
        req: GetWecomContactDetailRequest<'_>,
    ) -> Result<GetWecomContactDetailResponse, GeweError> {
        let env = self
            .post_api::<_, GetWecomContactDetailResponse>("gewe/v2/api/im/detail", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use gewe_core::{
        AddWecomContactRequest, GetWecomContactDetailRequest, SearchWecomRequest,
        SyncWecomContactsRequest,
    };

    #[test]
    fn test_search_wecom_request_serialization() {
        let req = SearchWecomRequest {
            app_id: "test_app",
            scene: 1,
            content: "search_keyword",
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("scene"));
        assert!(json.contains("content"));
        assert!(json.contains("search_keyword"));
    }

    #[test]
    fn test_search_wecom_request_field_names() {
        let req = SearchWecomRequest {
            app_id: "app123",
            scene: 2,
            content: "test",
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("scene").is_some());
        assert!(value.get("content").is_some());
    }

    #[test]
    fn test_sync_wecom_contacts_request_serialization() {
        let req = SyncWecomContactsRequest {
            app_id: "test_app_id",
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("test_app_id"));
    }

    #[test]
    fn test_add_wecom_contact_request_serialization() {
        let req = AddWecomContactRequest {
            app_id: "my_app",
            v3: "v3_value",
            v4: "v4_value",
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("my_app"));
        assert!(json.contains("v3"));
        assert!(json.contains("v3_value"));
        assert!(json.contains("v4"));
        assert!(json.contains("v4_value"));
    }

    #[test]
    fn test_get_wecom_contact_detail_request_serialization() {
        let req = GetWecomContactDetailRequest {
            app_id: "app_test",
            to_user_name: "wecom_user_123",
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("app_test"));
        assert!(json.contains("toUserName"));
        assert!(json.contains("wecom_user_123"));
    }

    #[test]
    fn test_get_wecom_contact_detail_request_field_names() {
        let req = GetWecomContactDetailRequest {
            app_id: "test",
            to_user_name: "user",
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("toUserName").is_some());
    }

    #[test]
    fn test_search_wecom_request_with_special_chars() {
        let req = SearchWecomRequest {
            app_id: "测试应用",
            scene: 1,
            content: "搜索关键词",
        };

        let json = serde_json::to_string(&req).unwrap();
        // 验证 Unicode 字符能正确序列化
        assert!(json.contains("测试应用") || json.contains("\\u"));
        assert!(json.contains("搜索关键词") || json.contains("\\u"));
    }

    #[test]
    fn test_search_wecom_request_clone() {
        let req = SearchWecomRequest {
            app_id: "app",
            scene: 3,
            content: "content",
        };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
        assert_eq!(req.scene, cloned.scene);
        assert_eq!(req.content, cloned.content);
    }

    #[test]
    fn test_sync_wecom_contacts_request_clone() {
        let req = SyncWecomContactsRequest { app_id: "app123" };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
    }
}
