use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct ForwardImageArgs {
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
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ForwardVideoArgs {
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
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ForwardFileArgs {
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
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ForwardMiniAppArgs {
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
    pub xml: String,
    #[arg(long)]
    pub cover_img_url: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ForwardUrlArgs {
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
    pub xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_forward_image(
    args: ForwardImageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardImageArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
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
    let resp = client.forward_image(&app_id, &to_wxid, &xml).await?;
    info!(?resp, "image forwarded");
    Ok(())
}

pub async fn handle_forward_video(
    args: ForwardVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
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
    let resp = client.forward_video(&app_id, &to_wxid, &xml).await?;
    info!(?resp, "video forwarded");
    Ok(())
}

pub async fn handle_forward_file(
    args: ForwardFileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardFileArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
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
    let resp = client.forward_file(&app_id, &to_wxid, &xml).await?;
    info!(?resp, "file forwarded");
    Ok(())
}

pub async fn handle_forward_mini_app(
    args: ForwardMiniAppArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardMiniAppArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        xml,
        cover_img_url,
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
        .forward_mini_app(&app_id, &to_wxid, &xml, &cover_img_url)
        .await?;
    info!(?resp, "mini app forwarded");
    Ok(())
}

pub async fn handle_forward_url(
    args: ForwardUrlArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardUrlArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
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
    let resp = client.forward_url(&app_id, &to_wxid, &xml).await?;
    info!(?resp, "url forwarded");
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
