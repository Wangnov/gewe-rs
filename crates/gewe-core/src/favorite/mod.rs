use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncFavorRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "syncKey", skip_serializing_if = "Option::is_none")]
    pub sync_key: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FavorSummary {
    #[serde(rename = "favId")]
    pub fav_id: i64,
    #[serde(rename = "type")]
    pub favor_type: i32,
    pub flag: i32,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncFavorResponse {
    #[serde(rename = "syncKey")]
    pub sync_key: String,
    pub list: Vec<FavorSummary>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFavorContentRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "favId")]
    pub fav_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFavorContentResponse {
    #[serde(rename = "favId")]
    pub fav_id: i64,
    pub status: i32,
    pub flag: i32,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFavorRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "favId")]
    pub fav_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favor_summary_default() {
        let summary = FavorSummary::default();
        assert_eq!(summary.fav_id, 0);
        assert_eq!(summary.favor_type, 0);
        assert_eq!(summary.flag, 0);
        assert_eq!(summary.update_time, 0);
    }

    #[test]
    fn test_sync_favor_response_default() {
        let resp = SyncFavorResponse::default();
        assert_eq!(resp.sync_key, "");
        assert!(resp.list.is_empty());
    }

    #[test]
    fn test_get_favor_content_response_default() {
        let resp = GetFavorContentResponse::default();
        assert_eq!(resp.fav_id, 0);
        assert_eq!(resp.status, 0);
        assert_eq!(resp.content, "");
    }

    #[test]
    fn test_sync_favor_request_serialization() {
        let req = SyncFavorRequest {
            app_id: "test_app",
            sync_key: Some("sync123"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("sync123"));
    }

    #[test]
    fn test_get_favor_content_request_serialization() {
        let req = GetFavorContentRequest {
            app_id: "test_app",
            fav_id: 123456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("123456"));
    }

    #[test]
    fn test_delete_favor_request_serialization() {
        let req = DeleteFavorRequest {
            app_id: "test_app",
            fav_id: 123456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("123456"));
    }
}
