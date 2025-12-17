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
