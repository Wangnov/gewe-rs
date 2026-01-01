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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_like_request() {
        let req = IdLikeRequest {
            app_id: "test_app",
            object_id: 123456,
            session_buffer: None,
            object_nonce_id: "nonce_123",
            op_type: 1,
            my_user_name: "my_user",
            my_role_type: 1,
            to_user_name: "to_user",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("objectId"));
        assert!(json.contains("opType"));
    }

    #[test]
    fn test_comment_finder_request() {
        let req = CommentFinderRequest {
            app_id: "test_app",
            proxy_ip: "",
            my_user_name: "my_user",
            op_type: 1,
            object_nonce_id: "nonce_123",
            session_buffer: "buffer",
            object_id: 123456,
            my_role_type: 1,
            content: "Nice video!",
            comment_id: "",
            reply_user_name: "",
            ref_comment_id: 0,
            root_comment_id: 0,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("content"));
        assert!(json.contains("Nice video!"));
    }

    #[test]
    fn test_comment_list_request() {
        let req = CommentListRequest {
            app_id: "test_app",
            object_id: 123456,
            last_buffer: None,
            session_buffer: "buffer",
            object_nonce_id: Some("nonce_123"),
            ref_comment_id: None,
            root_comment_id: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("objectId"));
    }
}
