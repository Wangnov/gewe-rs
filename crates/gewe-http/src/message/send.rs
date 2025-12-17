use crate::client::GeweHttpClient;
use gewe_core::{
    GeweError, PostAppMsgRequest, PostAppMsgResponse, PostEmojiRequest, PostEmojiResponse,
    PostFileRequest, PostFileResponse, PostImageRequest, PostImageResponse, PostLinkRequest,
    PostLinkResponse, PostMiniAppRequest, PostMiniAppResponse, PostNameCardRequest,
    PostNameCardResponse, PostVideoRequest, PostVideoResponse, PostVoiceRequest, PostVoiceResponse,
    SendTextRequest, SendTextResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn send_text(
        &self,
        app_id: &str,
        to_wxid: &str,
        content: &str,
        ats: Option<&str>,
    ) -> Result<SendTextResponse, GeweError> {
        let body = SendTextRequest {
            app_id,
            to_wxid,
            content,
            ats,
        };
        let env = self
            .post_api::<_, SendTextResponse>("gewe/v2/api/message/postText", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_image(
        &self,
        app_id: &str,
        to_wxid: &str,
        img_url: &str,
    ) -> Result<PostImageResponse, GeweError> {
        let body = PostImageRequest {
            app_id,
            to_wxid,
            img_url,
        };
        let env = self
            .post_api::<_, PostImageResponse>("gewe/v2/api/message/postImage", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_voice(
        &self,
        app_id: &str,
        to_wxid: &str,
        voice_url: &str,
        voice_duration: i64,
    ) -> Result<PostVoiceResponse, GeweError> {
        let body = PostVoiceRequest {
            app_id,
            to_wxid,
            voice_url,
            voice_duration,
        };
        let env = self
            .post_api::<_, PostVoiceResponse>("gewe/v2/api/message/postVoice", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_video(
        &self,
        app_id: &str,
        to_wxid: &str,
        video_url: &str,
        thumb_url: &str,
        video_duration: i64,
    ) -> Result<PostVideoResponse, GeweError> {
        let body = PostVideoRequest {
            app_id,
            to_wxid,
            video_url,
            thumb_url,
            video_duration,
        };
        let env = self
            .post_api::<_, PostVideoResponse>("gewe/v2/api/message/postVideo", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_file(
        &self,
        app_id: &str,
        to_wxid: &str,
        file_url: &str,
        file_name: &str,
    ) -> Result<PostFileResponse, GeweError> {
        let body = PostFileRequest {
            app_id,
            to_wxid,
            file_url,
            file_name,
        };
        let env = self
            .post_api::<_, PostFileResponse>("gewe/v2/api/message/postFile", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_link(
        &self,
        app_id: &str,
        to_wxid: &str,
        title: &str,
        desc: &str,
        link_url: &str,
        thumb_url: &str,
    ) -> Result<PostLinkResponse, GeweError> {
        let body = PostLinkRequest {
            app_id,
            to_wxid,
            title,
            desc,
            link_url,
            thumb_url,
        };
        let env = self
            .post_api::<_, PostLinkResponse>("gewe/v2/api/message/postLink", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_emoji(
        &self,
        app_id: &str,
        to_wxid: &str,
        emoji_md5: &str,
        emoji_size: i64,
    ) -> Result<PostEmojiResponse, GeweError> {
        let body = PostEmojiRequest {
            app_id,
            to_wxid,
            emoji_md5,
            emoji_size,
        };
        let env = self
            .post_api::<_, PostEmojiResponse>("gewe/v2/api/message/postEmoji", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_app_msg(
        &self,
        app_id: &str,
        to_wxid: &str,
        appmsg: &str,
    ) -> Result<PostAppMsgResponse, GeweError> {
        let body = PostAppMsgRequest {
            app_id,
            to_wxid,
            appmsg,
        };
        let env = self
            .post_api::<_, PostAppMsgResponse>("gewe/v2/api/message/postAppMsg", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[allow(clippy::too_many_arguments)]
    #[instrument(skip(self))]
    pub async fn send_mini_app(
        &self,
        app_id: &str,
        to_wxid: &str,
        mini_app_id: &str,
        display_name: &str,
        page_path: &str,
        cover_img_url: &str,
        title: &str,
        user_name: &str,
    ) -> Result<PostMiniAppResponse, GeweError> {
        let body = PostMiniAppRequest {
            app_id,
            to_wxid,
            mini_app_id,
            display_name,
            page_path,
            cover_img_url,
            title,
            user_name,
        };
        let env = self
            .post_api::<_, PostMiniAppResponse>("gewe/v2/api/message/postMiniApp", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn send_name_card(
        &self,
        app_id: &str,
        to_wxid: &str,
        nick_name: &str,
        name_card_wxid: &str,
    ) -> Result<PostNameCardResponse, GeweError> {
        let body = PostNameCardRequest {
            app_id,
            to_wxid,
            nick_name,
            name_card_wxid,
        };
        let env = self
            .post_api::<_, PostNameCardResponse>("gewe/v2/api/message/postNameCard", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
