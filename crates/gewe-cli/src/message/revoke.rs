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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BotRecord;

    #[test]
    fn test_revoke_msg_args_structure() {
        let args = RevokeMsgArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid123".to_string(),
            msg_id: "769533801".to_string(),
            new_msg_id: "5271007655758710001".to_string(),
            create_time: "1704163145".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "wxid123");
        assert_eq!(args.msg_id, "769533801");
        assert_eq!(args.new_msg_id, "5271007655758710001");
        assert_eq!(args.create_time, "1704163145");
        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
    }

    #[test]
    fn test_revoke_msg_args_with_custom_base_url() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid456".to_string(),
            msg_id: "123".to_string(),
            new_msg_id: "456".to_string(),
            create_time: "1234567890".to_string(),
            base_url: Some("http://custom.api.com".to_string()),
        };

        assert_eq!(args.base_url, Some("http://custom.api.com".to_string()));
    }

    #[test]
    fn test_revoke_msg_args_with_bot_alias() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("revoke_bot".to_string()),
            to_wxid: "wxid789".to_string(),
            msg_id: "111".to_string(),
            new_msg_id: "222".to_string(),
            create_time: "9999999999".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("revoke_bot".to_string()));
    }

    #[test]
    fn test_revoke_msg_args_with_bot_app_id() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: Some("app456".to_string()),
            bot_alias: None,
            to_wxid: "wxid000".to_string(),
            msg_id: "333".to_string(),
            new_msg_id: "444".to_string(),
            create_time: "1111111111".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("app456".to_string()));
    }

    #[test]
    fn test_revoke_msg_args_with_group_chat() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "34757816141@chatroom".to_string(),
            msg_id: "769533801".to_string(),
            new_msg_id: "5271007655758710001".to_string(),
            create_time: "1704163145".to_string(),
            base_url: None,
        };

        assert!(args.to_wxid.contains("@chatroom"));
    }

    #[test]
    fn test_revoke_msg_args_all_fields_populated() {
        let args = RevokeMsgArgs {
            token: Some("token123".to_string()),
            app_id: Some("app123".to_string()),
            bot_app_id: Some("bot123".to_string()),
            bot_alias: Some("alias123".to_string()),
            to_wxid: "wxid111".to_string(),
            msg_id: "msg123".to_string(),
            new_msg_id: "newmsg456".to_string(),
            create_time: "1234567890".to_string(),
            base_url: Some("http://test.com".to_string()),
        };

        assert!(args.token.is_some());
        assert!(args.app_id.is_some());
        assert!(args.bot_app_id.is_some());
        assert!(args.bot_alias.is_some());
        assert!(args.base_url.is_some());
    }

    #[test]
    fn test_revoke_msg_args_large_msg_ids() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid222".to_string(),
            msg_id: "999999999999999999".to_string(),
            new_msg_id: "888888888888888888".to_string(),
            create_time: "2147483647".to_string(),
            base_url: None,
        };

        assert_eq!(args.msg_id, "999999999999999999");
        assert_eq!(args.new_msg_id, "888888888888888888");
    }

    #[test]
    fn test_resolve_bot_with_alias() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("revoke_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        let result = resolve_bot(Some("revoke_bot".to_string()), None, &config).unwrap();
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
        let result = resolve_bot(Some("unknown_bot".to_string()), None, &config);
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
    fn test_resolve_bot_with_both_alias_and_explicit() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("revoke_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        // 当同时提供 alias 和 explicit 时，优先使用 alias
        let result = resolve_bot(
            Some("revoke_bot".to_string()),
            Some("app456".to_string()),
            &config,
        )
        .unwrap();
        assert_eq!(result, Some("app123".to_string()));
    }

    #[test]
    fn test_revoke_msg_args_empty_strings() {
        let args = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "".to_string(),
            msg_id: "".to_string(),
            new_msg_id: "".to_string(),
            create_time: "".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_wxid, "");
        assert_eq!(args.msg_id, "");
        assert_eq!(args.new_msg_id, "");
        assert_eq!(args.create_time, "");
    }

    #[test]
    fn test_revoke_msg_args_timestamp_formats() {
        // 测试不同的时间戳格式
        let args1 = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid333".to_string(),
            msg_id: "1".to_string(),
            new_msg_id: "2".to_string(),
            create_time: "1704163145".to_string(), // 10位时间戳
            base_url: None,
        };

        let args2 = RevokeMsgArgs {
            token: None,
            app_id: None,
            bot_app_id: None,
            bot_alias: None,
            to_wxid: "wxid444".to_string(),
            msg_id: "3".to_string(),
            new_msg_id: "4".to_string(),
            create_time: "1704163145000".to_string(), // 13位时间戳
            base_url: None,
        };

        assert_eq!(args1.create_time.len(), 10);
        assert_eq!(args2.create_time.len(), 13);
    }
}
