use crate::client::GeweHttpClient;
use gewe_core::{
    FollowFinderRequest, FollowFinderResponse, FollowListData, FollowListRequest, GeweError,
    SearchFollowRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn follow_finder(
        &self,
        req: FollowFinderRequest<'_>,
    ) -> Result<FollowFinderResponse, GeweError> {
        let env = self
            .post_api::<_, FollowFinderResponse>("gewe/v2/api/finder/follow", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn follow_list(
        &self,
        req: FollowListRequest<'_>,
    ) -> Result<FollowListData, GeweError> {
        let env = self
            .post_api::<_, FollowListData>("gewe/v2/api/finder/followList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn search_follow(&self, req: SearchFollowRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/searchFollow", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_finder_request() {
        let req = FollowFinderRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            to_user_name: "finder_user",
            op_type: 1,
            search_info: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("myUserName"));
        assert!(json.contains("toUserName"));
    }

    #[test]
    fn test_follow_list_request() {
        let req = FollowListRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            last_buffer: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("myUserName"));
    }

    #[test]
    fn test_search_follow_request() {
        let req = SearchFollowRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            to_user_name: "to_user",
            keyword: "search_term",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("keyword"));
    }
}
