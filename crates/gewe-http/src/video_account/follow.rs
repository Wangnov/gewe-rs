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
