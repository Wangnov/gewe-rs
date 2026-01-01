use crate::client::GeweHttpClient;
use gewe_core::{
    ContactListEntry, ContactListRequest, GeweError, MentionListRequest, MentionListResponse,
    PostPrivateLetterImgRequest, PostPrivateLetterRequest, PrivateLetterResponse,
    SendFinderMsgRequest, SyncPrivateLetterMsgRequest, SyncPrivateLetterMsgResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn send_finder_msg(&self, req: SendFinderMsgRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/message/sendFinderMsg", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn post_private_letter(
        &self,
        req: PostPrivateLetterRequest<'_>,
    ) -> Result<PrivateLetterResponse, GeweError> {
        let env = self
            .post_api::<_, PrivateLetterResponse>("gewe/v2/api/finder/postPrivateLetter", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn post_private_letter_img(
        &self,
        req: PostPrivateLetterImgRequest<'_>,
    ) -> Result<PrivateLetterResponse, GeweError> {
        let env = self
            .post_api::<_, PrivateLetterResponse>("gewe/v2/api/finder/postPrivateLetterImg", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn sync_private_letter_msg(
        &self,
        req: SyncPrivateLetterMsgRequest<'_>,
    ) -> Result<SyncPrivateLetterMsgResponse, GeweError> {
        let env = self
            .post_api::<_, SyncPrivateLetterMsgResponse>(
                "gewe/v2/api/finder/syncPrivateLetterMsg",
                &req,
            )
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn mention_list(
        &self,
        req: MentionListRequest<'_>,
    ) -> Result<MentionListResponse, GeweError> {
        let env = self
            .post_api::<_, MentionListResponse>("gewe/v2/api/finder/mentionList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn contact_list(
        &self,
        req: ContactListRequest<'_>,
    ) -> Result<Vec<ContactListEntry>, GeweError> {
        let env = self
            .post_api::<_, Vec<ContactListEntry>>("gewe/v2/api/finder/contactList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_finder_msg_request() {
        let req = SendFinderMsgRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            id: 123456,
            username: "username",
            nickname: "nickname",
            head_url: "https://example.com/head.jpg",
            nonce_id: "nonce_123",
            media_type: "video",
            width: "720",
            height: "480",
            url: "https://example.com/video.mp4",
            thumb_url: "https://example.com/thumb.jpg",
            thumb_url_token: "token",
            description: "Video description",
            video_play_len: "120",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("toWxid"));
    }

    #[test]
    fn test_post_private_letter_request() {
        let req = PostPrivateLetterRequest {
            app_id: "test_app",
            content: "Private message",
            to_user_name: "recipient",
            my_user_name: "my_user",
            msg_session_id: "session_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("content"));
    }

    #[test]
    fn test_sync_private_letter_msg_request() {
        let req = SyncPrivateLetterMsgRequest {
            app_id: "test_app",
            key_buff: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }

    #[test]
    fn test_contact_list_request() {
        let req = ContactListRequest {
            app_id: "test_app",
            my_user_name: "my_user",
            my_role_type: 1,
            query_info: "query",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("myUserName"));
    }
}
