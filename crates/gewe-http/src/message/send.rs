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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_text_request_serialization() {
        let req = SendTextRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            content: "Hello World",
            ats: Some("@wxid1,@wxid2"),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("toWxid"));
        assert!(json.contains("Hello World"));
    }

    #[test]
    fn test_send_text_without_ats() {
        let req = SendTextRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            content: "Hello",
            ats: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_post_image_request_serialization() {
        let req = PostImageRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            img_url: "https://example.com/image.jpg",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("imgUrl"));
        assert!(json.contains("https://example.com/image.jpg"));
    }

    #[test]
    fn test_post_voice_request_serialization() {
        let req = PostVoiceRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            voice_url: "https://example.com/voice.mp3",
            voice_duration: 30000,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("voiceUrl"));
        assert!(json.contains("30000"));
    }

    #[test]
    fn test_post_video_request_serialization() {
        let req = PostVideoRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            video_url: "https://example.com/video.mp4",
            thumb_url: "https://example.com/thumb.jpg",
            video_duration: 60000,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("videoUrl"));
        assert!(json.contains("thumbUrl"));
        assert!(json.contains("60000"));
    }

    #[test]
    fn test_post_file_request_serialization() {
        let req = PostFileRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            file_url: "https://example.com/file.pdf",
            file_name: "document.pdf",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("fileUrl"));
        assert!(json.contains("fileName"));
        assert!(json.contains("document.pdf"));
    }

    #[test]
    fn test_post_link_request_serialization() {
        let req = PostLinkRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            title: "Link Title",
            desc: "Link Description",
            link_url: "https://example.com",
            thumb_url: "https://example.com/thumb.jpg",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("Link Title"));
        assert!(json.contains("Link Description"));
        assert!(json.contains("linkUrl"));
    }

    #[test]
    fn test_post_emoji_request_serialization() {
        let req = PostEmojiRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            emoji_md5: "abc123def456",
            emoji_size: 102400,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("emojiMd5"));
        assert!(json.contains("abc123def456"));
        assert!(json.contains("102400"));
    }

    #[test]
    fn test_post_app_msg_request_serialization() {
        let req = PostAppMsgRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            appmsg: "<xml>app message</xml>",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appmsg"));
        assert!(json.contains("<xml>app message</xml>"));
    }

    #[test]
    fn test_post_mini_app_request_serialization() {
        let req = PostMiniAppRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            mini_app_id: "mini_app_123",
            display_name: "Mini App",
            page_path: "pages/index",
            cover_img_url: "https://example.com/cover.jpg",
            title: "App Title",
            user_name: "gh_123456",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("miniAppId"));
        assert!(json.contains("displayName"));
        assert!(json.contains("pagePath"));
        assert!(json.contains("Mini App"));
    }

    #[test]
    fn test_post_name_card_request_serialization() {
        let req = PostNameCardRequest {
            app_id: "test_app",
            to_wxid: "recipient_wxid",
            nick_name: "John Doe",
            name_card_wxid: "card_wxid_123",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("nickName"));
        assert!(json.contains("John Doe"));
        assert!(json.contains("nameCardWxid"));
    }

    #[test]
    fn test_send_text_with_unicode() {
        let req = SendTextRequest {
            app_id: "测试应用",
            to_wxid: "收件人",
            content: "你好世界",
            ats: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("测试应用"));
        assert!(json.contains("你好世界"));
    }
}
