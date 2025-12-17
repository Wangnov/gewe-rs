use crate::client::GeweHttpClient;
use gewe_core::{
    DownloadCdnRequest, DownloadCdnResponse, DownloadEmojiRequest, DownloadEmojiResponse,
    DownloadFileRequest, DownloadFileResponse, DownloadImageRequest, DownloadImageResponse,
    DownloadVideoRequest, DownloadVideoResponse, DownloadVoiceRequest, DownloadVoiceResponse,
    GeweError,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn download_image(
        &self,
        app_id: &str,
        xml: &str,
        image_type: i32,
    ) -> Result<DownloadImageResponse, GeweError> {
        let body = DownloadImageRequest {
            app_id,
            xml,
            image_type,
        };
        let env = self
            .post_api::<_, DownloadImageResponse>("gewe/v2/api/message/downloadImage", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_video(
        &self,
        app_id: &str,
        xml: &str,
    ) -> Result<DownloadVideoResponse, GeweError> {
        let body = DownloadVideoRequest { app_id, xml };
        let env = self
            .post_api::<_, DownloadVideoResponse>("gewe/v2/api/message/downloadVideo", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_file(
        &self,
        app_id: &str,
        xml: &str,
    ) -> Result<DownloadFileResponse, GeweError> {
        let body = DownloadFileRequest { app_id, xml };
        let env = self
            .post_api::<_, DownloadFileResponse>("gewe/v2/api/message/downloadFile", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_voice(
        &self,
        app_id: &str,
        xml: &str,
        msg_id: i64,
    ) -> Result<DownloadVoiceResponse, GeweError> {
        let body = DownloadVoiceRequest {
            app_id,
            xml,
            msg_id,
        };
        let env = self
            .post_api::<_, DownloadVoiceResponse>("gewe/v2/api/message/downloadVoice", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_emoji(
        &self,
        app_id: &str,
        emoji_md5: &str,
    ) -> Result<DownloadEmojiResponse, GeweError> {
        let body = DownloadEmojiRequest { app_id, emoji_md5 };
        let env = self
            .post_api::<_, DownloadEmojiResponse>("gewe/v2/api/message/downloadEmojiMd5", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn download_cdn(
        &self,
        app_id: &str,
        aes_key: &str,
        file_id: &str,
        file_type: &str,
        total_size: &str,
        suffix: &str,
    ) -> Result<DownloadCdnResponse, GeweError> {
        let body = DownloadCdnRequest {
            app_id,
            aes_key,
            file_id,
            r#type: file_type,
            total_size,
            suffix,
        };
        let env = self
            .post_api::<_, DownloadCdnResponse>("gewe/v2/api/message/downloadCdn", &body)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
