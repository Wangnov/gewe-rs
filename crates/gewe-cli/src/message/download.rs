use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct DownloadImageArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub xml: String,
    #[arg(long, default_value_t = 2)]
    pub image_type: i32,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadVideoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadFileArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadVoiceArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub xml: String,
    #[arg(long)]
    pub msg_id: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadEmojiArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub emoji_md5: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadCdnArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub aes_key: String,
    #[arg(long)]
    pub file_id: String,
    #[arg(long)]
    pub file_type: String,
    #[arg(long)]
    pub total_size: String,
    #[arg(long)]
    pub suffix: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_download_image(
    args: DownloadImageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadImageArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        xml,
        image_type,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.download_image(&app_id, &xml, image_type).await?;
    info!(?resp, "image downloaded");
    println!("{}", resp.file_url);
    Ok(())
}

pub async fn handle_download_video(
    args: DownloadVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        xml,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.download_video(&app_id, &xml).await?;
    info!(?resp, "video downloaded");
    println!("{}", resp.file_url);
    Ok(())
}

pub async fn handle_download_file(
    args: DownloadFileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadFileArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        xml,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.download_file(&app_id, &xml).await?;
    info!(?resp, "file downloaded");
    println!("{}", resp.file_url);
    Ok(())
}

pub async fn handle_download_voice(
    args: DownloadVoiceArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadVoiceArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        xml,
        msg_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.download_voice(&app_id, &xml, msg_id).await?;
    info!(?resp, "voice downloaded");
    println!("{}", resp.file_url);
    Ok(())
}

pub async fn handle_download_emoji(
    args: DownloadEmojiArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadEmojiArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        emoji_md5,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.download_emoji(&app_id, &emoji_md5).await?;
    info!(?resp, "emoji downloaded");
    println!("{}", resp.url);
    Ok(())
}

pub async fn handle_download_cdn(
    args: DownloadCdnArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadCdnArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        aes_key,
        file_id,
        file_type,
        total_size,
        suffix,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .download_cdn(
            &app_id,
            &aes_key,
            &file_id,
            &file_type,
            &total_size,
            &suffix,
        )
        .await?;
    info!(?resp, "cdn downloaded");
    println!("{}", resp.file_url);
    Ok(())
}

fn resolve_bot(
    alias: Option<String>,
    explicit: Option<String>,
    config: &CliConfig,
) -> Result<Option<String>> {
    if let Some(alias) = alias {
        Ok(Some(lookup_bot(config, &alias).ok_or_else(|| {
            anyhow!("bot alias not found: {}", alias)
        })?))
    } else {
        Ok(explicit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BotRecord;

    #[test]
    fn test_download_image_args_structure() {
        let args = DownloadImageArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            xml: "<xml>...</xml>".to_string(),
            image_type: 2,
            base_url: None,
        };

        assert_eq!(args.xml, "<xml>...</xml>");
        assert_eq!(args.image_type, 2);
        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_download_image_args_default_image_type() {
        let args = DownloadImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<xml>...</xml>".to_string(),
            image_type: 2,
            base_url: None,
        };

        // 默认值应该是 2 (常规图片)
        assert_eq!(args.image_type, 2);
    }

    #[test]
    fn test_download_image_args_high_quality() {
        let args = DownloadImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<xml>...</xml>".to_string(),
            image_type: 1,
            base_url: None,
        };

        assert_eq!(args.image_type, 1); // 高清图片
    }

    #[test]
    fn test_download_image_args_thumbnail() {
        let args = DownloadImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<xml>...</xml>".to_string(),
            image_type: 3,
            base_url: None,
        };

        assert_eq!(args.image_type, 3); // 缩略图
    }

    #[test]
    fn test_download_video_args_structure() {
        let args = DownloadVideoArgs {
            token: Some("token".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: None,
            bot_alias: None,
            xml: "<video>...</video>".to_string(),
            base_url: Some("http://custom.api.com".to_string()),
        };

        assert_eq!(args.xml, "<video>...</video>");
        assert_eq!(args.base_url, Some("http://custom.api.com".to_string()));
    }

    #[test]
    fn test_download_file_args_structure() {
        let args = DownloadFileArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<file>...</file>".to_string(),
            base_url: None,
        };

        assert_eq!(args.xml, "<file>...</file>");
    }

    #[test]
    fn test_download_voice_args_structure() {
        let args = DownloadVoiceArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<voice>...</voice>".to_string(),
            msg_id: 123456789,
            base_url: None,
        };

        assert_eq!(args.xml, "<voice>...</voice>");
        assert_eq!(args.msg_id, 123456789);
    }

    #[test]
    fn test_download_emoji_args_structure() {
        let args = DownloadEmojiArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            emoji_md5: "abc123def456".to_string(),
            base_url: None,
        };

        assert_eq!(args.emoji_md5, "abc123def456");
    }

    #[test]
    fn test_download_cdn_args_structure() {
        let args = DownloadCdnArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            aes_key: "aeskey123".to_string(),
            file_id: "fileid456".to_string(),
            file_type: "image".to_string(),
            total_size: "1024".to_string(),
            suffix: "jpg".to_string(),
            base_url: None,
        };

        assert_eq!(args.aes_key, "aeskey123");
        assert_eq!(args.file_id, "fileid456");
        assert_eq!(args.file_type, "image");
        assert_eq!(args.total_size, "1024");
        assert_eq!(args.suffix, "jpg");
    }

    #[test]
    fn test_download_image_args_with_bot_alias() {
        let args = DownloadImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("my_bot".to_string()),
            xml: "<xml>...</xml>".to_string(),
            image_type: 2,
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("my_bot".to_string()));
    }

    #[test]
    fn test_download_video_args_with_bot_app_id() {
        let args = DownloadVideoArgs {
            token: None,
            app_id: None,
            bot_app_id: Some("app789".to_string()),
            bot_alias: None,
            xml: "<video>...</video>".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("app789".to_string()));
    }

    #[test]
    fn test_resolve_bot_with_alias() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("dl_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        let result = resolve_bot(Some("dl_bot".to_string()), None, &config).unwrap();
        assert_eq!(result, Some("app123".to_string()));
    }

    #[test]
    fn test_resolve_bot_with_explicit_app_id() {
        let config = CliConfig::default();
        let result = resolve_bot(None, Some("app456".to_string()), &config).unwrap();
        assert_eq!(result, Some("app456".to_string()));
    }

    #[test]
    fn test_resolve_bot_alias_not_found() {
        let config = CliConfig::default();
        let result = resolve_bot(Some("unknown_bot".to_string()), None, &config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("bot alias not found"));
    }

    #[test]
    fn test_download_cdn_args_with_all_fields() {
        let args = DownloadCdnArgs {
            token: Some("token123".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: Some("bot456".to_string()),
            bot_alias: Some("bot_alias".to_string()),
            aes_key: "key1".to_string(),
            file_id: "id1".to_string(),
            file_type: "video".to_string(),
            total_size: "2048".to_string(),
            suffix: "mp4".to_string(),
            base_url: Some("http://custom.com".to_string()),
        };

        assert!(args.token.is_some());
        assert!(args.app_id.is_some());
        assert!(args.bot_app_id.is_some());
        assert!(args.bot_alias.is_some());
        assert!(args.base_url.is_some());
    }

    #[test]
    fn test_download_voice_args_with_large_msg_id() {
        let args = DownloadVoiceArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            xml: "<voice>...</voice>".to_string(),
            msg_id: 9999999999999,
            base_url: None,
        };

        assert_eq!(args.msg_id, 9999999999999);
    }
}
