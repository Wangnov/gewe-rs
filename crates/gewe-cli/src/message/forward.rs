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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BotRecord;

    #[test]
    fn test_forward_image_args_structure() {
        let args = ForwardImageArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid123".to_string(),
            xml: "<img>...</img>".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid123");
        assert_eq!(args.xml, "<img>...</img>");
        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_forward_image_args_with_custom_base_url() {
        let args = ForwardImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid456".to_string(),
            xml: "<img>...</img>".to_string(),
            base_url: Some("http://custom.api.com".to_string()),
        };

        assert_eq!(args.base_url, Some("http://custom.api.com".to_string()));
    }

    #[test]
    fn test_forward_video_args_structure() {
        let args = ForwardVideoArgs {
            token: Some("token".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid789".to_string(),
            xml: "<video>...</video>".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid789");
        assert_eq!(args.xml, "<video>...</video>");
    }

    #[test]
    fn test_forward_file_args_structure() {
        let args = ForwardFileArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid000".to_string(),
            xml: "<file>...</file>".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid000");
        assert_eq!(args.xml, "<file>...</file>");
    }

    #[test]
    fn test_forward_mini_app_args_structure() {
        let args = ForwardMiniAppArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid111".to_string(),
            xml: "<miniapp>...</miniapp>".to_string(),
            cover_img_url: "http://example.com/cover.jpg".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid111");
        assert_eq!(args.xml, "<miniapp>...</miniapp>");
        assert_eq!(args.cover_img_url, "http://example.com/cover.jpg");
    }

    #[test]
    fn test_forward_url_args_structure() {
        let args = ForwardUrlArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid222".to_string(),
            xml: "<url>...</url>".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid222");
        assert_eq!(args.xml, "<url>...</url>");
    }

    #[test]
    fn test_forward_image_args_with_bot_alias() {
        let args = ForwardImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("fwd_bot".to_string()),
            to_wxid: "wxid333".to_string(),
            xml: "<img>...</img>".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("fwd_bot".to_string()));
    }

    #[test]
    fn test_forward_video_args_with_bot_app_id() {
        let args = ForwardVideoArgs {
            token: None,
            app_id: None,
            bot_app_id: Some("app456".to_string()),
            bot_alias: None,
            to_wxid: "wxid444".to_string(),
            xml: "<video>...</video>".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("app456".to_string()));
    }

    #[test]
    fn test_forward_file_args_with_all_optional_fields() {
        let args = ForwardFileArgs {
            token: Some("token123".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: Some("bot123".to_string()),
            bot_alias: Some("alias123".to_string()),
            to_wxid: "wxid555".to_string(),
            xml: "<file>test</file>".to_string(),
            base_url: Some("http://test.com".to_string()),
        };

        assert!(args.token.is_some());
        assert!(args.app_id.is_some());
        assert!(args.bot_app_id.is_some());
        assert!(args.bot_alias.is_some());
        assert!(args.base_url.is_some());
    }

    #[test]
    fn test_forward_mini_app_args_with_empty_cover() {
        let args = ForwardMiniAppArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid666".to_string(),
            xml: "<miniapp>...</miniapp>".to_string(),
            cover_img_url: "".to_string(),
            base_url: None,
        };

        assert_eq!(args.cover_img_url, "");
    }

    #[test]
    fn test_resolve_bot_with_alias() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("fwd_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        let result = resolve_bot(Some("fwd_bot".to_string()), None, &config).unwrap();
        assert_eq!(result, Some("app123".to_string()));
    }

    #[test]
    fn test_resolve_bot_with_explicit_app_id() {
        let config = CliConfig::default();
        let result = resolve_bot(None, Some("app789".to_string()), &config).unwrap();
        assert_eq!(result, Some("app789".to_string()));
    }

    #[test]
    fn test_resolve_bot_alias_not_found() {
        let config = CliConfig::default();
        let result = resolve_bot(Some("nonexistent".to_string()), None, &config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("bot alias not found"));
    }

    #[test]
    fn test_resolve_bot_none() {
        let config = CliConfig::default();
        let result = resolve_bot(None, None, &config).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_forward_url_args_with_long_xml() {
        let long_xml = "<url>".to_string() + &"a".repeat(1000) + "</url>";
        let args = ForwardUrlArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid777".to_string(),
            xml: long_xml.clone(),
            base_url: None,
        };

        assert_eq!(args.xml, long_xml);
        assert!(args.xml.len() > 1000);
    }

    #[test]
    fn test_forward_image_args_multiple_recipients() {
        let args1 = ForwardImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid1".to_string(),
            xml: "<img>...</img>".to_string(),
            base_url: None,
        };

        let args2 = ForwardImageArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid2".to_string(),
            xml: "<img>...</img>".to_string(),
            base_url: None,
        };

        assert_ne!(args1.to_wxid, args2.to_wxid);
        assert_eq!(args1.xml, args2.xml);
    }
}
