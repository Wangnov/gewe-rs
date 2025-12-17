use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct GetProfileArgs {
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

#[derive(Args)]
pub struct UpdateProfileArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub nick_name: Option<String>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub province: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(long)]
    pub sex: Option<i32>,
    #[arg(long)]
    pub signature: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct UpdateHeadImgArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub head_img_url: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetQrCodeArgs {
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

#[derive(Args)]
pub struct PrivacySettingsArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub option: i32,
    #[arg(long)]
    pub open: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetSafetyInfoArgs {
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

pub async fn handle_get_profile(
    args: GetProfileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetProfileArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
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
        .get_profile(gewe_core::GetProfileRequest { app_id: &app_id })
        .await?;
    info!(wxid=%resp.wxid, nick=%resp.nick_name, "profile fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_update_profile(
    args: UpdateProfileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let UpdateProfileArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        nick_name,
        country,
        province,
        city,
        sex,
        signature,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    if nick_name.is_none()
        && country.is_none()
        && province.is_none()
        && city.is_none()
        && sex.is_none()
        && signature.is_none()
    {
        return Err(anyhow!("至少需要提供一项要更新的个人信息"));
    }
    let client = GeweHttpClient::new(token, base_url)?;
    client
        .update_profile(gewe_core::UpdateProfileRequest {
            app_id: &app_id,
            nick_name: nick_name.as_deref(),
            country: country.as_deref(),
            province: province.as_deref(),
            city: city.as_deref(),
            sex,
            signature: signature.as_deref(),
        })
        .await?;
    info!("profile updated");
    Ok(())
}

pub async fn handle_update_head_img(
    args: UpdateHeadImgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let UpdateHeadImgArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        head_img_url,
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
        .update_head_img(gewe_core::UpdateHeadImgRequest {
            app_id: &app_id,
            head_img_url: &head_img_url,
        })
        .await?;
    info!("head image update triggered");
    Ok(())
}

pub async fn handle_get_qr_code(
    args: GetQrCodeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetQrCodeArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
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
        .get_qr_code(gewe_core::GetQrCodeRequest { app_id: &app_id })
        .await?;
    info!("qr code fetched");
    println!("{}", resp.qr_code);
    Ok(())
}

pub async fn handle_privacy_settings(
    args: PrivacySettingsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let PrivacySettingsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        option,
        open,
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
        .privacy_settings(gewe_core::PrivacySettingsRequest {
            app_id: &app_id,
            option,
            open,
        })
        .await?;
    info!(option, open, "privacy settings updated");
    Ok(())
}

pub async fn handle_get_safety_info(
    args: GetSafetyInfoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetSafetyInfoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
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
        .get_safety_info(gewe_core::GetSafetyInfoRequest { app_id: &app_id })
        .await?;
    info!(count = resp.list.len(), "safety info fetched");
    println!("{resp:#?}");
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
