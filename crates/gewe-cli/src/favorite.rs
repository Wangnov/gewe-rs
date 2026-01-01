use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_core::{DeleteFavorRequest, GetFavorContentRequest, SyncFavorRequest};
use gewe_http::GeweHttpClient;
use serde_json::to_string_pretty;
use std::path::Path;
use tracing::info;

#[derive(Args, Clone)]
pub struct FavoriteBaseArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args, Clone)]
pub struct SyncFavoriteArgs {
    #[command(flatten)]
    pub base: FavoriteBaseArgs,
    #[arg(long)]
    pub sync_key: Option<String>,
}

#[derive(Args, Clone)]
pub struct GetFavoriteContentArgs {
    #[command(flatten)]
    pub base: FavoriteBaseArgs,
    #[arg(long)]
    pub fav_id: i64,
}

#[derive(Args, Clone)]
pub struct DeleteFavoriteArgs {
    #[command(flatten)]
    pub base: FavoriteBaseArgs,
    #[arg(long)]
    pub fav_id: i64,
}

pub async fn handle_sync_favorites(
    args: SyncFavoriteArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .sync_favorites(SyncFavorRequest {
            app_id: &app_id,
            sync_key: args.sync_key.as_deref(),
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_get_favorite_content(
    args: GetFavoriteContentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .get_favor_content(GetFavorContentRequest {
            app_id: &app_id,
            fav_id: args.fav_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_delete_favorite(
    args: DeleteFavoriteArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .delete_favor(DeleteFavorRequest {
            app_id: &app_id,
            fav_id: args.fav_id,
        })
        .await?;
    info!(fav_id = args.fav_id, "favorite deleted");
    Ok(())
}

async fn resolve_client(
    base: &FavoriteBaseArgs,
    config: &CliConfig,
) -> Result<(GeweHttpClient, String)> {
    let token = resolve_value(base.token.clone(), config.token.clone(), "token")?;
    let base_url = base
        .base_url
        .clone()
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(
        base.bot_alias.clone(),
        base.bot_app_id.clone().or(base.app_id.clone()),
        config,
    )?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    Ok((client, app_id))
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
