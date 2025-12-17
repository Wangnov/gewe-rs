use crate::config::{default_base_url, resolve_value, save_config, upsert_bot, CliConfig};
use anyhow::Result;
use clap::Args;
use gewe_core::{
    ChangeMacToIpadRequest, CheckLoginRequest, CheckOnlineRequest, DialogLoginRequest,
    GetLoginQrCodeRequest, LoginByAccountRequest, LogoutRequest, ReconnectionRequest,
    SetCallbackRequest,
};
use gewe_http::GeweHttpClient;
use serde_json::to_string_pretty;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct GetLoginQrArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub device_type: Option<String>,
    #[arg(long)]
    pub region_id: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct CheckLoginArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub uuid: String,
    #[arg(long)]
    pub base_url: Option<String>,
    #[arg(long, default_value_t = true)]
    pub auto_sliding: bool,
}

#[derive(Args)]
pub struct DialogLoginArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub region_id: Option<String>,
    #[arg(long)]
    pub proxy_ip: Option<String>,
    #[arg(long)]
    pub aid: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct LoginByAccountArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub proxy_ip: String,
    #[arg(long)]
    pub region_id: String,
    #[arg(long)]
    pub account: String,
    #[arg(long)]
    pub password: String,
    #[arg(long)]
    pub step: i32,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetCallbackArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub callback_url: String,
    /// 不设置时默认与 --token 一致
    #[arg(long)]
    pub callback_token: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ChangeMacToIpadArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct CheckOnlineArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ReconnectionArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct LogoutArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_get_login_qr(
    args: GetLoginQrArgs,
    config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetLoginQrArgs {
        token,
        app_id,
        device_type,
        region_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = app_id.or_else(|| config.app_id.clone()).unwrap_or_default();
    let device_type = device_type
        .or_else(|| config.device_type.clone())
        .unwrap_or_else(|| "ipad".to_string());
    let region_id = region_id
        .or_else(|| config.region_id.clone())
        .unwrap_or_else(|| "320000".to_string());
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .get_login_qr_code(GetLoginQrCodeRequest {
            app_id: &app_id,
            r#type: &device_type,
            region_id: &region_id,
            proxy_ip: None,
            ttuid: None,
            aid: None,
        })
        .await?;
    info!(app_id=%resp.app_id, uuid=%resp.uuid, "received login QR");
    config.app_id = Some(resp.app_id.clone());
    upsert_bot(config, &resp.app_id, None);
    save_config(config_path, config)?;
    println!("{}", resp.qr_img_base64);
    Ok(())
}

pub async fn handle_check_login(
    args: CheckLoginArgs,
    config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let CheckLoginArgs {
        token,
        app_id,
        uuid,
        base_url,
        auto_sliding,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .check_login(CheckLoginRequest {
            app_id: &app_id,
            uuid: &uuid,
            proxy_ip: None,
            captch_code: None,
            auto_sliding: Some(auto_sliding),
        })
        .await?;
    let wxid = resp.login_info.as_ref().and_then(|info| info.wxid.clone());
    info!(status=%resp.status, uuid=%resp.uuid, ?wxid, "check login result");
    if resp.status == 2 {
        config.app_id = Some(app_id.clone());
        upsert_bot(config, &app_id, wxid);
        save_config(config_path, config)?;
    }
    Ok(())
}

pub async fn handle_dialog_login(
    args: DialogLoginArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DialogLoginArgs {
        token,
        app_id,
        region_id,
        proxy_ip,
        aid,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let region_id = region_id
        .or_else(|| config.region_id.clone())
        .unwrap_or_else(|| "320000".to_string());
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .dialog_login(DialogLoginRequest {
            app_id: &app_id,
            region_id: &region_id,
            proxy_ip: proxy_ip.as_deref(),
            aid: aid.as_deref(),
        })
        .await?;
    info!(uuid = %resp.uuid, "dialog login triggered");
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_login_by_account(
    args: LoginByAccountArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let LoginByAccountArgs {
        token,
        app_id,
        proxy_ip,
        region_id,
        account,
        password,
        step,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = app_id.or_else(|| config.app_id.clone()).unwrap_or_default();
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .login_by_account(LoginByAccountRequest {
            app_id: &app_id,
            proxy_ip: &proxy_ip,
            region_id: &region_id,
            account: &account,
            password: &password,
            step,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_set_callback(
    args: SetCallbackArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetCallbackArgs {
        token,
        callback_url,
        callback_token,
        base_url,
    } = args;
    let token_value = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let client = GeweHttpClient::new(token_value.clone(), base_url)?;
    let body_token = callback_token.unwrap_or(token_value);
    client
        .set_callback(SetCallbackRequest {
            token: &body_token,
            callback_url: &callback_url,
        })
        .await?;
    info!("callback url updated");
    Ok(())
}

pub async fn handle_change_mac_to_ipad(
    args: ChangeMacToIpadArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ChangeMacToIpadArgs {
        token,
        app_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .change_mac_to_ipad(ChangeMacToIpadRequest { app_id: &app_id })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_check_online(
    args: CheckOnlineArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let CheckOnlineArgs {
        token,
        app_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let online = client
        .check_online(CheckOnlineRequest { app_id: &app_id })
        .await?;
    println!("{}", online);
    Ok(())
}

pub async fn handle_reconnection(
    args: ReconnectionArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ReconnectionArgs {
        token,
        app_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .reconnection(ReconnectionRequest { app_id: &app_id })
        .await?;
    if let Some(data) = resp {
        println!("{}", to_string_pretty(&data)?);
    } else {
        info!("reconnection request accepted, waiting for confirmation");
    }
    Ok(())
}

pub async fn handle_logout(
    args: LogoutArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let LogoutArgs {
        token,
        app_id,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let app_id = resolve_value(app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    client.logout(LogoutRequest { app_id: &app_id }).await?;
    info!("logout command sent");
    Ok(())
}
