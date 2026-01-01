use crate::client::GeweHttpClient;
use gewe_core::{GeweError, RevokeMessageRequest};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn revoke_message(
        &self,
        app_id: &str,
        to_wxid: &str,
        msg_id: &str,
        new_msg_id: &str,
        create_time: &str,
    ) -> Result<(), GeweError> {
        let body = RevokeMessageRequest {
            app_id,
            to_wxid,
            msg_id,
            new_msg_id,
            create_time,
        };
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/message/revokeMsg", &body)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revoke_message_request_serialization() {
        let req = RevokeMessageRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            msg_id: "msg_123456",
            new_msg_id: "new_msg_123456",
            create_time: "1234567890",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("toWxid"));
        assert!(json.contains("msgId"));
        assert!(json.contains("newMsgId"));
        assert!(json.contains("createTime"));
    }
}
