use crate::client::GeweHttpClient;
use gewe_core::{
    DeleteFavorRequest, GetFavorContentRequest, GetFavorContentResponse, GeweError,
    SyncFavorRequest, SyncFavorResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn sync_favorites(
        &self,
        req: SyncFavorRequest<'_>,
    ) -> Result<SyncFavorResponse, GeweError> {
        let env = self
            .post_api::<_, SyncFavorResponse>("gewe/v2/api/favor/sync", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_favor_content(
        &self,
        req: GetFavorContentRequest<'_>,
    ) -> Result<GetFavorContentResponse, GeweError> {
        let env = self
            .post_api::<_, GetFavorContentResponse>("gewe/v2/api/favor/getContent", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn delete_favor(&self, req: DeleteFavorRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/favor/delete", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use gewe_core::{DeleteFavorRequest, GetFavorContentRequest, SyncFavorRequest};

    #[test]
    fn test_sync_favor_request_serialization() {
        let req = SyncFavorRequest {
            app_id: "test_app",
            sync_key: Some("sync_key_123"),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("syncKey"));
        assert!(json.contains("sync_key_123"));
    }

    #[test]
    fn test_sync_favor_request_without_sync_key() {
        let req = SyncFavorRequest {
            app_id: "test_app",
            sync_key: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        // sync_key 为 None 时应该被跳过（skip_serializing_if）
        assert!(!json.contains("syncKey"));
    }

    #[test]
    fn test_sync_favor_request_field_names() {
        let req = SyncFavorRequest {
            app_id: "app",
            sync_key: Some("key"),
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("syncKey").is_some());
    }

    #[test]
    fn test_get_favor_content_request_serialization() {
        let req = GetFavorContentRequest {
            app_id: "my_app",
            fav_id: 123456,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("my_app"));
        assert!(json.contains("favId"));
        assert!(json.contains("123456"));
    }

    #[test]
    fn test_get_favor_content_request_field_names() {
        let req = GetFavorContentRequest {
            app_id: "app",
            fav_id: 999,
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("favId").is_some());
        assert_eq!(value.get("favId").unwrap().as_i64().unwrap(), 999);
    }

    #[test]
    fn test_delete_favor_request_serialization() {
        let req = DeleteFavorRequest {
            app_id: "app_test",
            fav_id: 789,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("app_test"));
        assert!(json.contains("favId"));
        assert!(json.contains("789"));
    }

    #[test]
    fn test_delete_favor_request_field_names() {
        let req = DeleteFavorRequest {
            app_id: "app",
            fav_id: 456,
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("favId").is_some());
        assert_eq!(value.get("favId").unwrap().as_i64().unwrap(), 456);
    }

    #[test]
    fn test_sync_favor_request_with_large_fav_id() {
        let req = GetFavorContentRequest {
            app_id: "app",
            fav_id: 9999999999,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("9999999999"));
    }

    #[test]
    fn test_sync_favor_request_with_special_chars() {
        let req = SyncFavorRequest {
            app_id: "测试应用",
            sync_key: Some("密钥123"),
        };

        let json = serde_json::to_string(&req).unwrap();
        // 验证 Unicode 字符能正确序列化
        assert!(json.contains("测试应用") || json.contains("\\u"));
    }

    #[test]
    fn test_sync_favor_request_clone() {
        let req = SyncFavorRequest {
            app_id: "app",
            sync_key: Some("key"),
        };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
        assert_eq!(req.sync_key, cloned.sync_key);
    }

    #[test]
    fn test_get_favor_content_request_clone() {
        let req = GetFavorContentRequest {
            app_id: "app",
            fav_id: 100,
        };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
        assert_eq!(req.fav_id, cloned.fav_id);
    }

    #[test]
    fn test_delete_favor_request_clone() {
        let req = DeleteFavorRequest {
            app_id: "app",
            fav_id: 200,
        };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
        assert_eq!(req.fav_id, cloned.fav_id);
    }
}
