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
