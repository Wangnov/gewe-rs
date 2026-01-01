use crate::client::GeweHttpClient;
use gewe_core::{GeweError, SearchFinderRequest, SearchFinderResponse};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn search_finder(
        &self,
        req: SearchFinderRequest<'_>,
    ) -> Result<SearchFinderResponse, GeweError> {
        let env = self
            .post_api::<_, SearchFinderResponse>("gewe/v2/api/finder/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_finder_request() {
        let req = SearchFinderRequest {
            app_id: "test_app",
            content: "search keyword",
            category: None,
            filter: None,
            page: None,
            cookie: None,
            search_id: None,
            offset: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("content"));
    }
}
