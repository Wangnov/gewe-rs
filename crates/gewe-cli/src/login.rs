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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_login_qr_args_default_values() {
        let args = GetLoginQrArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            device_type: Some("ipad".to_string()),
            region_id: Some("320000".to_string()),
            base_url: None,
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
        assert_eq!(args.device_type, Some("ipad".to_string()));
        assert_eq!(args.region_id, Some("320000".to_string()));
        assert!(args.base_url.is_none());
    }

    #[test]
    fn test_check_login_args_auto_sliding_default() {
        let args = CheckLoginArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            uuid: "test_uuid".to_string(),
            base_url: None,
            auto_sliding: true,
        };

        assert_eq!(args.uuid, "test_uuid");
        assert!(args.auto_sliding);
    }

    #[test]
    fn test_dialog_login_args_structure() {
        let args = DialogLoginArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            region_id: Some("320000".to_string()),
            proxy_ip: Some("127.0.0.1".to_string()),
            aid: Some("aid123".to_string()),
            base_url: None,
        };

        assert_eq!(args.proxy_ip, Some("127.0.0.1".to_string()));
        assert_eq!(args.aid, Some("aid123".to_string()));
    }

    #[test]
    fn test_login_by_account_args_required_fields() {
        let args = LoginByAccountArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            proxy_ip: "192.168.1.1".to_string(),
            region_id: "320000".to_string(),
            account: "test_account".to_string(),
            password: "test_password".to_string(),
            step: 1,
            base_url: None,
        };

        assert_eq!(args.proxy_ip, "192.168.1.1");
        assert_eq!(args.region_id, "320000");
        assert_eq!(args.account, "test_account");
        assert_eq!(args.password, "test_password");
        assert_eq!(args.step, 1);
    }

    #[test]
    fn test_set_callback_args_token_usage() {
        let args = SetCallbackArgs {
            token: Some("test_token".to_string()),
            callback_url: "http://example.com/callback".to_string(),
            callback_token: Some("callback_token".to_string()),
            base_url: None,
        };

        assert_eq!(args.callback_url, "http://example.com/callback");
        assert_eq!(args.callback_token, Some("callback_token".to_string()));
    }

    #[test]
    fn test_set_callback_args_no_callback_token() {
        let args = SetCallbackArgs {
            token: Some("test_token".to_string()),
            callback_url: "http://example.com/callback".to_string(),
            callback_token: None,
            base_url: None,
        };

        assert!(args.callback_token.is_none());
    }

    #[test]
    fn test_change_mac_to_ipad_args() {
        let args = ChangeMacToIpadArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: Some("http://test.com".to_string()),
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
        assert_eq!(args.base_url, Some("http://test.com".to_string()));
    }

    #[test]
    fn test_check_online_args() {
        let args = CheckOnlineArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_reconnection_args() {
        let args = ReconnectionArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_logout_args() {
        let args = LogoutArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_get_login_qr_args_with_all_fields() {
        let args = GetLoginQrArgs {
            token: Some("token123".to_string()),
            app_id: Some("app456".to_string()),
            device_type: Some("mac".to_string()),
            region_id: Some("110000".to_string()),
            base_url: Some("http://custom.com".to_string()),
        };

        assert_eq!(args.token, Some("token123".to_string()));
        assert_eq!(args.app_id, Some("app456".to_string()));
        assert_eq!(args.device_type, Some("mac".to_string()));
        assert_eq!(args.region_id, Some("110000".to_string()));
        assert_eq!(args.base_url, Some("http://custom.com".to_string()));
    }

    #[test]
    fn test_check_login_args_with_custom_auto_sliding() {
        let args = CheckLoginArgs {
            token: Some("token123".to_string()),
            app_id: Some("app456".to_string()),
            uuid: "uuid789".to_string(),
            base_url: Some("http://custom.com".to_string()),
            auto_sliding: false,
        };

        assert_eq!(args.uuid, "uuid789");
        assert!(!args.auto_sliding);
        assert_eq!(args.base_url, Some("http://custom.com".to_string()));
    }

    #[test]
    fn test_login_by_account_args_with_different_steps() {
        let args1 = LoginByAccountArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            proxy_ip: "192.168.1.1".to_string(),
            region_id: "320000".to_string(),
            account: "account1".to_string(),
            password: "password1".to_string(),
            step: 1,
            base_url: None,
        };

        let args2 = LoginByAccountArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            proxy_ip: "192.168.1.1".to_string(),
            region_id: "320000".to_string(),
            account: "account2".to_string(),
            password: "password2".to_string(),
            step: 2,
            base_url: None,
        };

        assert_eq!(args1.step, 1);
        assert_eq!(args2.step, 2);
    }

    #[test]
    fn test_args_with_optional_token() {
        let args_with_token = CheckOnlineArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        let args_without_token = CheckOnlineArgs {
            token: None,
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        assert!(args_with_token.token.is_some());
        assert!(args_without_token.token.is_none());
    }

    #[test]
    fn test_args_with_optional_app_id() {
        let args_with_app_id = ReconnectionArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            base_url: None,
        };

        let args_without_app_id = ReconnectionArgs {
            token: Some("test_token".to_string()),
            app_id: None,
            base_url: None,
        };

        assert!(args_with_app_id.app_id.is_some());
        assert!(args_without_app_id.app_id.is_none());
    }

    #[test]
    fn test_dialog_login_args_with_optional_fields() {
        let args_full = DialogLoginArgs {
            token: Some("token".to_string()),
            app_id: Some("app".to_string()),
            region_id: Some("region".to_string()),
            proxy_ip: Some("proxy".to_string()),
            aid: Some("aid".to_string()),
            base_url: Some("base".to_string()),
        };

        let args_minimal = DialogLoginArgs {
            token: Some("token".to_string()),
            app_id: Some("app".to_string()),
            region_id: None,
            proxy_ip: None,
            aid: None,
            base_url: None,
        };

        assert!(args_full.proxy_ip.is_some());
        assert!(args_full.aid.is_some());
        assert!(args_minimal.proxy_ip.is_none());
        assert!(args_minimal.aid.is_none());
    }
}
