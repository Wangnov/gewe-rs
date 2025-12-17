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
