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
