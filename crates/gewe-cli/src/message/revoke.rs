use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct RevokeMsgArgs {
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
    pub msg_id: String,
    #[arg(long)]
    pub new_msg_id: String,
    #[arg(long)]
    pub create_time: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_revoke_msg(
    args: RevokeMsgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let RevokeMsgArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_wxid,
        msg_id,
        new_msg_id,
        create_time,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    client
        .revoke_message(&app_id, &to_wxid, &msg_id, &new_msg_id, &create_time)
        .await?;
    info!(%msg_id, %new_msg_id, "message revoked");
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
