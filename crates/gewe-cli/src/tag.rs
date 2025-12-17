use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_core::{AddLabelRequest, DeleteLabelRequest, ListLabelRequest, ModifyLabelMemberRequest};
use gewe_http::GeweHttpClient;
use serde_json::to_string_pretty;
use std::path::Path;
use tracing::info;

#[derive(Args, Clone)]
pub struct LabelBaseArgs {
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
pub struct AddLabelArgs {
    #[command(flatten)]
    pub base: LabelBaseArgs,
    #[arg(long)]
    pub label_name: String,
}

#[derive(Args, Clone)]
pub struct DeleteLabelArgs {
    #[command(flatten)]
    pub base: LabelBaseArgs,
    /// 标签 ID，多个用逗号拼接
    #[arg(long)]
    pub label_ids: String,
}

#[derive(Args, Clone)]
pub struct ListLabelArgs {
    #[command(flatten)]
    pub base: LabelBaseArgs,
}

#[derive(Args, Clone)]
pub struct ModifyLabelMembersArgs {
    #[command(flatten)]
    pub base: LabelBaseArgs,
    /// 目标好友 wxid，多个用逗号分隔
    #[arg(long = "wx-id", value_delimiter = ',')]
    pub wx_ids: Vec<String>,
    /// 标签 ID 列表，多个逗号分隔，需全量覆盖
    #[arg(long)]
    pub label_ids: String,
}

pub async fn handle_add_label(
    args: AddLabelArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .add_label(AddLabelRequest {
            app_id: &app_id,
            label_name: &args.label_name,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_delete_label(
    args: DeleteLabelArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .delete_label(DeleteLabelRequest {
            app_id: &app_id,
            label_ids: &args.label_ids,
        })
        .await?;
    info!(ids = args.label_ids, "labels deleted");
    Ok(())
}

pub async fn handle_list_labels(
    args: ListLabelArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .list_labels(ListLabelRequest { app_id: &app_id })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

pub async fn handle_modify_label_members(
    args: ModifyLabelMembersArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    if args.wx_ids.is_empty() {
        return Err(anyhow!("至少传入一个 wx-id"));
    }
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let wx_refs: Vec<&str> = args.wx_ids.iter().map(|s| s.as_str()).collect();
    client
        .modify_label_members(ModifyLabelMemberRequest {
            app_id: &app_id,
            label_ids: &args.label_ids,
            wx_ids: wx_refs,
        })
        .await?;
    info!(
        label_ids = args.label_ids,
        count = args.wx_ids.len(),
        "label members updated"
    );
    Ok(())
}

async fn resolve_client(
    base: &LabelBaseArgs,
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
