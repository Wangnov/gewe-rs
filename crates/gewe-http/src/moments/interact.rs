use crate::client::GeweHttpClient;
use gewe_core::{CommentSnsRequest, GeweError, LikeSnsRequest};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn like_sns(&self, req: LikeSnsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/likeSns", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn comment_sns(&self, req: CommentSnsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/sns/commentSns", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_like_sns_request() {
        let req = LikeSnsRequest {
            app_id: "test_app",
            sns_id: 123456,
            oper_type: 1,
            wxid: "wxid_test",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsId"));
        assert!(json.contains("operType"));
    }

    #[test]
    fn test_comment_sns_request() {
        let req = CommentSnsRequest {
            app_id: "test_app",
            sns_id: 123456,
            oper_type: 1,
            wxid: "wxid_test",
            comment_id: Some("comment_123"),
            content: Some("Great post!"),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("snsId"));
        assert!(json.contains("Great post!"));
    }
}
