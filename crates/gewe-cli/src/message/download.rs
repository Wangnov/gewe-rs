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
