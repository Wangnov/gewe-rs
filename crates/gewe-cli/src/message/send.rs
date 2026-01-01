use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct SendTextArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub ats: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendImageArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub img_url: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendVoiceArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub voice_url: String,
    #[arg(long)]
    pub voice_duration: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendVideoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub video_url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub video_duration: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendFileArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub file_url: String,
    #[arg(long)]
    pub file_name: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendLinkArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub desc: String,
    #[arg(long)]
    pub link_url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendEmojiArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub emoji_md5: String,
    #[arg(long)]
    pub emoji_size: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendAppmsgArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub appmsg: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendMiniAppArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub mini_app_id: String,
    #[arg(long)]
    pub display_name: String,
    #[arg(long)]
    pub page_path: String,
    #[arg(long)]
    pub cover_img_url: String,
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub user_name: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendNameCardArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub nick_name: String,
    #[arg(long)]
    pub name_card_wxid: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_send_text(
    args: SendTextArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendTextArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        content,
        ats,
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
        .send_text(&app_id, &to_wxid, &content, ats.as_deref())
        .await?;
    info!(?resp, "text sent");
    Ok(())
}

pub async fn handle_send_image(
    args: SendImageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendImageArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        img_url,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.send_image(&app_id, &to_wxid, &img_url).await?;
    info!(?resp, "image sent");
    Ok(())
}

pub async fn handle_send_voice(
    args: SendVoiceArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendVoiceArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        voice_url,
        voice_duration,
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
        .send_voice(&app_id, &to_wxid, &voice_url, voice_duration)
        .await?;
    info!(?resp, "voice sent");
    Ok(())
}

pub async fn handle_send_video(
    args: SendVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        video_url,
        thumb_url,
        video_duration,
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
        .send_video(&app_id, &to_wxid, &video_url, &thumb_url, video_duration)
        .await?;
    info!(?resp, "video sent");
    Ok(())
}

pub async fn handle_send_file(
    args: SendFileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendFileArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        file_url,
        file_name,
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
        .send_file(&app_id, &to_wxid, &file_url, &file_name)
        .await?;
    info!(?resp, "file sent");
    Ok(())
}

pub async fn handle_send_link(
    args: SendLinkArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendLinkArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        title,
        desc,
        link_url,
        thumb_url,
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
        .send_link(&app_id, &to_wxid, &title, &desc, &link_url, &thumb_url)
        .await?;
    info!(?resp, "link sent");
    Ok(())
}

pub async fn handle_send_emoji(
    args: SendEmojiArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendEmojiArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        emoji_md5,
        emoji_size,
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
        .send_emoji(&app_id, &to_wxid, &emoji_md5, emoji_size)
        .await?;
    info!(?resp, "emoji sent");
    Ok(())
}

pub async fn handle_send_appmsg(
    args: SendAppmsgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendAppmsgArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        appmsg,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client.send_app_msg(&app_id, &to_wxid, &appmsg).await?;
    info!(?resp, "appmsg sent");
    Ok(())
}

pub async fn handle_send_mini_app(
    args: SendMiniAppArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendMiniAppArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        mini_app_id,
        display_name,
        page_path,
        cover_img_url,
        title,
        user_name,
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
        .send_mini_app(
            &app_id,
            &to_wxid,
            &mini_app_id,
            &display_name,
            &page_path,
            &cover_img_url,
            &title,
            &user_name,
        )
        .await?;
    info!(?resp, "mini app sent");
    Ok(())
}

pub async fn handle_send_name_card(
    args: SendNameCardArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendNameCardArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        nick_name,
        name_card_wxid,
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
        .send_name_card(&app_id, &to_wxid, &nick_name, &name_card_wxid)
        .await?;
    info!(?resp, "name card sent");
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
    fn test_send_text_args_has_required_fields() {
        let args = SendTextArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid123".to_string(),
            content: "Hello".to_string(),
            ats: None,
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid123");
        assert_eq!(args.content, "Hello");
        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_send_text_args_with_ats() {
        let args = SendTextArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "chatroom@123".to_string(),
            content: "@user test".to_string(),
            ats: Some("wxid_user1,wxid_user2".to_string()),
            base_url: None,
        };

        assert_eq!(args.ats, Some("wxid_user1,wxid_user2".to_string()));
    }

    #[test]
    fn test_send_image_args_structure() {
        let args = SendImageArgs {
            token: Some("token".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid456".to_string(),
            img_url: "http://example.com/image.jpg".to_string(),
            base_url: Some("http://custom.api.com".to_string()),
        };

        assert_eq!(args.img_url, "http://example.com/image.jpg");
        assert_eq!(args.base_url, Some("http://custom.api.com".to_string()));
    }

    #[test]
    fn test_send_voice_args_with_duration() {
        let args = SendVoiceArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid789".to_string(),
            voice_url: "http://example.com/voice.mp3".to_string(),
            voice_duration: 10,
            base_url: None,
        };

        assert_eq!(args.voice_duration, 10);
        assert_eq!(args.voice_url, "http://example.com/voice.mp3");
    }

    #[test]
    fn test_send_video_args_with_thumb() {
        let args = SendVideoArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid000".to_string(),
            video_url: "http://example.com/video.mp4".to_string(),
            thumb_url: "http://example.com/thumb.jpg".to_string(),
            video_duration: 30,
            base_url: None,
        };

        assert_eq!(args.video_url, "http://example.com/video.mp4");
        assert_eq!(args.thumb_url, "http://example.com/thumb.jpg");
        assert_eq!(args.video_duration, 30);
    }

    #[test]
    fn test_send_file_args_structure() {
        let args = SendFileArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid111".to_string(),
            file_url: "http://example.com/file.pdf".to_string(),
            file_name: "document.pdf".to_string(),
            base_url: None,
        };

        assert_eq!(args.file_url, "http://example.com/file.pdf");
        assert_eq!(args.file_name, "document.pdf");
    }

    #[test]
    fn test_send_link_args_structure() {
        let args = SendLinkArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid222".to_string(),
            title: "Link Title".to_string(),
            desc: "Link Description".to_string(),
            link_url: "http://example.com".to_string(),
            thumb_url: "http://example.com/thumb.jpg".to_string(),
            base_url: None,
        };

        assert_eq!(args.title, "Link Title");
        assert_eq!(args.desc, "Link Description");
        assert_eq!(args.link_url, "http://example.com");
        assert_eq!(args.thumb_url, "http://example.com/thumb.jpg");
    }

    #[test]
    fn test_send_emoji_args_structure() {
        let args = SendEmojiArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid333".to_string(),
            emoji_md5: "abc123def456".to_string(),
            emoji_size: 1024,
            base_url: None,
        };

        assert_eq!(args.emoji_md5, "abc123def456");
        assert_eq!(args.emoji_size, 1024);
    }

    #[test]
    fn test_send_appmsg_args_structure() {
        let args = SendAppmsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid444".to_string(),
            appmsg: "<xml>...</xml>".to_string(),
            base_url: None,
        };

        assert_eq!(args.appmsg, "<xml>...</xml>");
    }

    #[test]
    fn test_send_mini_app_args_structure() {
        let args = SendMiniAppArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid555".to_string(),
            mini_app_id: "mini123".to_string(),
            display_name: "Mini App".to_string(),
            page_path: "pages/index".to_string(),
            cover_img_url: "http://example.com/cover.jpg".to_string(),
            title: "App Title".to_string(),
            user_name: "username".to_string(),
            base_url: None,
        };

        assert_eq!(args.mini_app_id, "mini123");
        assert_eq!(args.display_name, "Mini App");
        assert_eq!(args.page_path, "pages/index");
        assert_eq!(args.title, "App Title");
        assert_eq!(args.user_name, "username");
    }

    #[test]
    fn test_send_name_card_args_structure() {
        let args = SendNameCardArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid666".to_string(),
            nick_name: "Nick Name".to_string(),
            name_card_wxid: "wxid_card".to_string(),
            base_url: None,
        };

        assert_eq!(args.nick_name, "Nick Name");
        assert_eq!(args.name_card_wxid, "wxid_card");
    }

    #[test]
    fn test_resolve_bot_with_alias() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("my_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        let result = resolve_bot(Some("my_bot".to_string()), None, &config).unwrap();
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
        let result = resolve_bot(Some("unknown".to_string()), None, &config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("bot alias not found"));
    }

    #[test]
    fn test_resolve_bot_with_both_alias_and_explicit() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("my_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        // 当同时提供 alias 和 explicit 时，优先使用 alias
        let result = resolve_bot(
            Some("my_bot".to_string()),
            Some("app456".to_string()),
            &config,
        )
        .unwrap();
        assert_eq!(result, Some("app123".to_string()));
    }

    #[test]
    fn test_resolve_bot_none() {
        let config = CliConfig::default();
        let result = resolve_bot(None, None, &config).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_send_text_args_with_bot_alias() {
        let args = SendTextArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("my_bot".to_string()),
            to_wxid: "wxid123".to_string(),
            content: "Hello".to_string(),
            ats: None,
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("my_bot".to_string()));
    }

    #[test]
    fn test_send_text_args_with_bot_app_id() {
        let args = SendTextArgs {
            token: None,
            app_id: None,
            bot_app_id: Some("app789".to_string()),
            bot_alias: None,
            to_wxid: "wxid123".to_string(),
            content: "Hello".to_string(),
            ats: None,
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("app789".to_string()));
    }

    #[test]
    fn test_send_image_args_with_bot_alias() {
        let args = SendImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("img_bot".to_string()),
            to_wxid: "wxid456".to_string(),
            img_url: "http://example.com/image.jpg".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("img_bot".to_string()));
    }
}
