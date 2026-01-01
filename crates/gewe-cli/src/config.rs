use anyhow::{anyhow, Result};
use clap::Args;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub token: Option<String>,
    pub base_url: Option<String>,
    pub app_id: Option<String>,
    pub region_id: Option<String>,
    pub device_type: Option<String>,
    #[serde(default)]
    pub bots: Vec<BotRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BotRecord {
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub region_id: Option<String>,
    #[arg(long)]
    pub device_type: Option<String>,
    #[arg(long)]
    pub list_bots: bool,
    #[arg(long)]
    pub alias: Option<String>,
    #[arg(long)]
    pub alias_target_app_id: Option<String>,
    #[arg(long)]
    pub alias_target_wxid: Option<String>,
}

pub fn resolve_config_path(custom: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = custom {
        return Ok(path.to_path_buf());
    }
    if let Some(proj) = ProjectDirs::from("com", "gewe", "gewe-cli") {
        let dir = proj.config_dir();
        fs::create_dir_all(dir)?;
        return Ok(dir.join("config.toml"));
    }
    let home = dirs::home_dir().ok_or_else(|| anyhow!("failed to resolve home directory"))?;
    let dir = home.join(".config/gewe");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("config.toml"))
}

pub fn load_config(path: &Path) -> Result<CliConfig> {
    if !path.exists() {
        return Ok(CliConfig::default());
    }
    let contents = fs::read_to_string(path)?;
    let cfg: CliConfig = toml::from_str(&contents)?;
    Ok(cfg)
}

pub fn save_config(path: &Path, cfg: &CliConfig) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let toml = toml::to_string_pretty(cfg)?;
    let mut file = fs::File::create(path)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}

pub fn upsert_bot(config: &mut CliConfig, app_id: &str, wxid: Option<String>) {
    if let Some(entry) = config.bots.iter_mut().find(|b| b.app_id == app_id) {
        entry.wxid = wxid;
    } else {
        config.bots.push(BotRecord {
            app_id: app_id.to_string(),
            wxid,
            alias: None,
            token: None,
            webhook_secret: None,
        });
    }
}

pub fn lookup_bot(config: &CliConfig, alias: &str) -> Option<String> {
    config
        .bots
        .iter()
        .find(|b| {
            b.app_id == alias
                || b.wxid.as_deref() == Some(alias)
                || b.alias.as_deref() == Some(alias)
        })
        .map(|b| b.app_id.clone())
}

pub fn set_alias(config: &mut CliConfig, target: &str, alias: String) -> bool {
    if let Some(entry) = config
        .bots
        .iter_mut()
        .find(|b| b.app_id == target || b.wxid.as_deref() == Some(target))
    {
        entry.alias = Some(alias);
        true
    } else {
        false
    }
}

pub fn resolve_value(
    value: Option<String>,
    fallback: Option<String>,
    field: &str,
) -> Result<String> {
    value
        .or(fallback)
        .ok_or_else(|| anyhow!("{field} required; set via flag or config command"))
}

pub fn default_base_url() -> String {
    "http://api.geweapi.com".to_string()
}

pub fn handle_config(args: ConfigArgs, config_path: &Path, config: &mut CliConfig) -> Result<()> {
    let ConfigArgs {
        token,
        base_url,
        app_id,
        region_id,
        device_type,
        list_bots,
        alias,
        alias_target_app_id,
        alias_target_wxid,
    } = args;

    let mut updated = false;
    if let Some(token) = token {
        config.token = Some(token);
        updated = true;
    }
    if let Some(base_url) = base_url {
        config.base_url = Some(base_url);
        updated = true;
    }
    if let Some(app_id) = app_id {
        config.app_id = Some(app_id);
        updated = true;
    }
    if let Some(region_id) = region_id {
        config.region_id = Some(region_id);
        updated = true;
    }
    if let Some(device_type) = device_type {
        config.device_type = Some(device_type);
        updated = true;
    }
    if let Some(alias) = alias {
        if let Some(target) = alias_target_app_id.or(alias_target_wxid.clone()) {
            if set_alias(config, &target, alias) {
                updated = true;
            } else {
                eprintln!("Alias target not found; use --list-bots to inspect");
            }
        } else {
            eprintln!("--alias requires --alias-target-app-id or --alias-target-wxid");
        }
    }
    if updated {
        save_config(config_path, config)?;
        println!("Config updated at {}", config_path.display());
    } else if list_bots {
        if config.bots.is_empty() {
            println!("No bots stored");
        } else {
            for bot in &config.bots {
                println!(
                    "appId={} wxid={} alias={}",
                    bot.app_id,
                    bot.wxid.as_deref().unwrap_or("<unknown>"),
                    bot.alias.as_deref().unwrap_or("<none>")
                );
            }
        }
    } else {
        println!("{}", toml::to_string_pretty(config)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_config_path_with_custom_path() {
        let custom_path = Path::new("/tmp/custom_config.toml");
        let result = resolve_config_path(Some(custom_path)).unwrap();
        assert_eq!(result, custom_path);
    }

    #[test]
    fn test_default_base_url() {
        assert_eq!(default_base_url(), "http://api.geweapi.com");
    }

    #[test]
    fn test_cli_config_default() {
        let config = CliConfig::default();
        assert!(config.token.is_none());
        assert!(config.base_url.is_none());
        assert!(config.app_id.is_none());
        assert!(config.region_id.is_none());
        assert!(config.device_type.is_none());
        assert!(config.bots.is_empty());
    }

    #[test]
    fn test_upsert_bot_new_bot() {
        let mut config = CliConfig::default();
        upsert_bot(&mut config, "app123", Some("wxid123".to_string()));

        assert_eq!(config.bots.len(), 1);
        assert_eq!(config.bots[0].app_id, "app123");
        assert_eq!(config.bots[0].wxid, Some("wxid123".to_string()));
        assert_eq!(config.bots[0].alias, None);
    }

    #[test]
    fn test_upsert_bot_update_existing() {
        let mut config = CliConfig::default();
        upsert_bot(&mut config, "app123", Some("wxid123".to_string()));
        upsert_bot(&mut config, "app123", Some("wxid456".to_string()));

        assert_eq!(config.bots.len(), 1);
        assert_eq!(config.bots[0].app_id, "app123");
        assert_eq!(config.bots[0].wxid, Some("wxid456".to_string()));
    }

    #[test]
    fn test_upsert_bot_with_none_wxid() {
        let mut config = CliConfig::default();
        upsert_bot(&mut config, "app123", Some("wxid123".to_string()));
        upsert_bot(&mut config, "app123", None);

        assert_eq!(config.bots.len(), 1);
        assert_eq!(config.bots[0].app_id, "app123");
        assert_eq!(config.bots[0].wxid, None);
    }

    #[test]
    fn test_lookup_bot_by_app_id() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        });

        assert_eq!(lookup_bot(&config, "app123"), Some("app123".to_string()));
    }

    #[test]
    fn test_lookup_bot_by_wxid() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        });

        assert_eq!(lookup_bot(&config, "wxid123"), Some("app123".to_string()));
    }

    #[test]
    fn test_lookup_bot_by_alias() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        });

        assert_eq!(lookup_bot(&config, "bot1"), Some("app123".to_string()));
    }

    #[test]
    fn test_lookup_bot_not_found() {
        let config = CliConfig::default();
        assert_eq!(lookup_bot(&config, "nonexistent"), None);
    }

    #[test]
    fn test_set_alias_by_app_id() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: None,
            token: None,
            webhook_secret: None,
        });

        let result = set_alias(&mut config, "app123", "my_bot".to_string());
        assert!(result);
        assert_eq!(config.bots[0].alias, Some("my_bot".to_string()));
    }

    #[test]
    fn test_set_alias_by_wxid() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: None,
            token: None,
            webhook_secret: None,
        });

        let result = set_alias(&mut config, "wxid123", "my_bot".to_string());
        assert!(result);
        assert_eq!(config.bots[0].alias, Some("my_bot".to_string()));
    }

    #[test]
    fn test_set_alias_not_found() {
        let mut config = CliConfig::default();
        let result = set_alias(&mut config, "nonexistent", "my_bot".to_string());
        assert!(!result);
    }

    #[test]
    fn test_resolve_value_with_value() {
        let result = resolve_value(
            Some("value1".to_string()),
            Some("fallback".to_string()),
            "test_field",
        )
        .unwrap();
        assert_eq!(result, "value1");
    }

    #[test]
    fn test_resolve_value_with_fallback() {
        let result = resolve_value(None, Some("fallback".to_string()), "test_field").unwrap();
        assert_eq!(result, "fallback");
    }

    #[test]
    fn test_resolve_value_error() {
        let result = resolve_value(None, None, "test_field");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("test_field required"));
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let mut config = CliConfig::default();
        config.token = Some("test_token".to_string());
        config.base_url = Some("http://test.com".to_string());
        config.bots.push(BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        });

        save_config(&config_path, &config).unwrap();
        assert!(config_path.exists());

        let loaded_config = load_config(&config_path).unwrap();
        assert_eq!(loaded_config.token, Some("test_token".to_string()));
        assert_eq!(loaded_config.base_url, Some("http://test.com".to_string()));
        assert_eq!(loaded_config.bots.len(), 1);
        assert_eq!(loaded_config.bots[0].app_id, "app123");
        assert_eq!(loaded_config.bots[0].wxid, Some("wxid123".to_string()));
        assert_eq!(loaded_config.bots[0].alias, Some("bot1".to_string()));
    }

    #[test]
    fn test_load_config_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");

        let config = load_config(&config_path).unwrap();
        assert!(config.token.is_none());
        assert!(config.bots.is_empty());
    }

    #[test]
    fn test_save_config_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir
            .path()
            .join("nested")
            .join("dir")
            .join("config.toml");

        let config = CliConfig::default();
        save_config(&config_path, &config).unwrap();
        assert!(config_path.exists());
    }

    #[test]
    fn test_bot_record_serialization() {
        let bot = BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        };

        let serialized = toml::to_string(&bot).unwrap();
        let deserialized: BotRecord = toml::from_str(&serialized).unwrap();
        assert_eq!(bot, deserialized);
    }

    #[test]
    fn test_bot_record_serialization_skip_none() {
        let bot = BotRecord {
            app_id: "app123".to_string(),
            wxid: None,
            alias: None,
            token: None,
            webhook_secret: None,
        };

        let serialized = toml::to_string(&bot).unwrap();
        assert!(!serialized.contains("wxid"));
        assert!(!serialized.contains("alias"));
    }

    #[test]
    fn test_cli_config_serialization_with_bots() {
        let mut config = CliConfig::default();
        config.token = Some("token123".to_string());
        config.bots.push(BotRecord {
            app_id: "app1".to_string(),
            wxid: Some("wxid1".to_string()),
            alias: None,
            token: None,
            webhook_secret: None,
        });
        config.bots.push(BotRecord {
            app_id: "app2".to_string(),
            wxid: None,
            alias: Some("bot2".to_string()),
            token: None,
            webhook_secret: None,
        });

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: CliConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.token, config.token);
        assert_eq!(deserialized.bots.len(), 2);
        assert_eq!(deserialized.bots[0].app_id, "app1");
        assert_eq!(deserialized.bots[1].alias, Some("bot2".to_string()));
    }

    #[test]
    fn test_lookup_bot_multiple_bots() {
        let mut config = CliConfig::default();
        config.bots.push(BotRecord {
            app_id: "app1".to_string(),
            wxid: Some("wxid1".to_string()),
            alias: Some("bot1".to_string()),
            token: None,
            webhook_secret: None,
        });
        config.bots.push(BotRecord {
            app_id: "app2".to_string(),
            wxid: Some("wxid2".to_string()),
            alias: Some("bot2".to_string()),
            token: None,
            webhook_secret: None,
        });

        assert_eq!(lookup_bot(&config, "app1"), Some("app1".to_string()));
        assert_eq!(lookup_bot(&config, "wxid2"), Some("app2".to_string()));
        assert_eq!(lookup_bot(&config, "bot1"), Some("app1".to_string()));
        assert_eq!(lookup_bot(&config, "bot2"), Some("app2".to_string()));
    }

    #[test]
    fn test_upsert_bot_multiple_bots() {
        let mut config = CliConfig::default();
        upsert_bot(&mut config, "app1", Some("wxid1".to_string()));
        upsert_bot(&mut config, "app2", Some("wxid2".to_string()));
        upsert_bot(&mut config, "app1", Some("wxid1_updated".to_string()));

        assert_eq!(config.bots.len(), 2);
        assert_eq!(config.bots[0].app_id, "app1");
        assert_eq!(config.bots[0].wxid, Some("wxid1_updated".to_string()));
        assert_eq!(config.bots[1].app_id, "app2");
        assert_eq!(config.bots[1].wxid, Some("wxid2".to_string()));
    }
}
