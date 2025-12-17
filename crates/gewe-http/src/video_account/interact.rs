use crate::client::GeweHttpClient;
use gewe_core::{
    BrowseFinderRequest, CommentFinderRequest, CommentFinderResponse, CommentListRequest,
    CommentListResponse, FinderOptRequest, GeweError, IdFavRequest, IdLikeRequest,
    LikeFavListRequest, LikeFavListResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn finder_opt(&self, req: FinderOptRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/finderOpt", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn browse_finder(&self, req: BrowseFinderRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/browse", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn id_fav(&self, req: IdFavRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/idFav", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn id_like(&self, req: IdLikeRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/finder/idLike", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn like_fav_list(
        &self,
        req: LikeFavListRequest<'_>,
    ) -> Result<LikeFavListResponse, GeweError> {
        let env = self
            .post_api::<_, LikeFavListResponse>("gewe/v2/api/finder/likeFavList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn comment_finder(
        &self,
        req: CommentFinderRequest<'_>,
    ) -> Result<CommentFinderResponse, GeweError> {
        let env = self
            .post_api::<_, CommentFinderResponse>("gewe/v2/api/finder/comment", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn comment_list(
        &self,
        req: CommentListRequest<'_>,
    ) -> Result<CommentListResponse, GeweError> {
        let env = self
            .post_api::<_, CommentListResponse>("gewe/v2/api/finder/commentList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
