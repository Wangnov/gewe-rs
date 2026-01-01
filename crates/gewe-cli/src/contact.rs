use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct FetchContactsListArgs {
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
pub struct FetchContactsListCacheArgs {
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
pub struct SearchContactsArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub contacts_info: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct AddContactsArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub scene: i32,
    #[arg(long)]
    pub option: i32,
    #[arg(long)]
    pub v3: String,
    #[arg(long)]
    pub v4: String,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetFriendRemarkArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub remark: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetFriendPermissionsArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub only_chat: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DeleteFriendArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct CheckContactRelationArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetContactBriefInfoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetContactDetailInfoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetPhoneAddressListArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long, value_delimiter = ',', num_args = 0..)]
    pub phones: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct UploadPhoneAddressListArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub phones: Vec<String>,
    #[arg(long)]
    pub op_type: i32,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SearchWecomContactArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub scene: i32,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SyncWecomContactsArgs {
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
pub struct AddWecomContactArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub v3: String,
    #[arg(long)]
    pub v4: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetWecomContactDetailArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_fetch_contacts_list(
    args: FetchContactsListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let FetchContactsListArgs {
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
        .fetch_contacts_list(gewe_core::FetchContactsListRequest { app_id: &app_id })
        .await?;
    info!(
        friends = resp.friends.len(),
        chatrooms = resp.chatrooms.len(),
        ghs = resp.ghs.len(),
        "contacts list fetched"
    );
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_fetch_contacts_list_cache(
    args: FetchContactsListCacheArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let FetchContactsListCacheArgs {
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
    client
        .fetch_contacts_list_cache(gewe_core::FetchContactsListCacheRequest { app_id: &app_id })
        .await?;
    info!("contact list cache fetched");
    println!("通讯录列表缓存获取成功");
    Ok(())
}

pub async fn handle_search_contacts(
    args: SearchContactsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SearchContactsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        contacts_info,
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
        .search_contacts(gewe_core::SearchContactsRequest {
            app_id: &app_id,
            contacts_info: &contacts_info,
        })
        .await?;
    info!(nick=%resp.nick_name, "contact search completed");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_add_contacts(
    args: AddContactsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let AddContactsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        scene,
        option,
        v3,
        v4,
        content,
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
        .add_contacts(gewe_core::AddContactsRequest {
            app_id: &app_id,
            scene,
            option,
            v3: &v3,
            v4: &v4,
            content: &content,
        })
        .await?;
    info!(option, "contact add/accept triggered");
    Ok(())
}

pub async fn handle_set_friend_remark(
    args: SetFriendRemarkArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetFriendRemarkArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxid,
        remark,
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
        .set_friend_remark(gewe_core::SetFriendRemarkRequest {
            app_id: &app_id,
            wxid: &wxid,
            remark: &remark,
        })
        .await?;
    info!(%wxid, "friend remark set");
    Ok(())
}

pub async fn handle_set_friend_permissions(
    args: SetFriendPermissionsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetFriendPermissionsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxid,
        only_chat,
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
        .set_friend_permissions(gewe_core::SetFriendPermissionsRequest {
            app_id: &app_id,
            wxid: &wxid,
            only_chat,
        })
        .await?;
    info!(%wxid, only_chat, "friend permission updated");
    Ok(())
}

pub async fn handle_delete_friend(
    args: DeleteFriendArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DeleteFriendArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxid,
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
        .delete_friend(gewe_core::DeleteFriendRequest {
            app_id: &app_id,
            wxid: &wxid,
        })
        .await?;
    info!(%wxid, "friend deleted");
    Ok(())
}

pub async fn handle_check_contact_relation(
    args: CheckContactRelationArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let CheckContactRelationArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxids,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let wxid_refs: Vec<&str> = wxids.iter().map(|s| s.as_str()).collect();
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .check_contact_relation(gewe_core::CheckRelationRequest {
            app_id: &app_id,
            wxids: wxid_refs,
        })
        .await?;
    info!(count = resp.len(), "contact relations checked");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_contact_brief_info(
    args: GetContactBriefInfoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetContactBriefInfoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxids,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let wxid_refs: Vec<&str> = wxids.iter().map(|s| s.as_str()).collect();
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .get_contact_brief_info(gewe_core::GetContactBriefInfoRequest {
            app_id: &app_id,
            wxids: wxid_refs,
        })
        .await?;
    info!(count = resp.len(), "contact brief info fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_contact_detail_info(
    args: GetContactDetailInfoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetContactDetailInfoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxids,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let wxid_refs: Vec<&str> = wxids.iter().map(|s| s.as_str()).collect();
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .get_contact_detail_info(gewe_core::GetContactDetailInfoRequest {
            app_id: &app_id,
            wxids: wxid_refs,
        })
        .await?;
    info!(count = resp.len(), "contact detail info fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_phone_address_list(
    args: GetPhoneAddressListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetPhoneAddressListArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        phones,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let phone_refs = if phones.is_empty() {
        None
    } else {
        Some(phones.iter().map(|s| s.as_str()).collect())
    };
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .get_phone_address_list(gewe_core::GetPhoneAddressListRequest {
            app_id: &app_id,
            phones: phone_refs,
        })
        .await?;
    info!(count = resp.len(), "phone address list fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_upload_phone_address_list(
    args: UploadPhoneAddressListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let UploadPhoneAddressListArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        phones,
        op_type,
        base_url,
    } = args;
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let phone_refs: Vec<&str> = phones.iter().map(|s| s.as_str()).collect();
    let client = GeweHttpClient::new(token, base_url)?;
    client
        .upload_phone_address_list(gewe_core::UploadPhoneAddressListRequest {
            app_id: &app_id,
            phones: phone_refs,
            op_type,
        })
        .await?;
    info!(count = phones.len(), op_type, "phone address list uploaded");
    Ok(())
}

pub async fn handle_search_wecom_contact(
    args: SearchWecomContactArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SearchWecomContactArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        scene,
        content,
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
        .search_wecom_contact(gewe_core::SearchWecomRequest {
            app_id: &app_id,
            scene,
            content: &content,
        })
        .await?;
    info!(nick=%resp.nick_name, "wecom contact searched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_sync_wecom_contacts(
    args: SyncWecomContactsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SyncWecomContactsArgs {
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
        .sync_wecom_contacts(gewe_core::SyncWecomContactsRequest { app_id: &app_id })
        .await?;
    info!(count = resp.len(), "wecom contacts synced");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_add_wecom_contact(
    args: AddWecomContactArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let AddWecomContactArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        v3,
        v4,
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
        .add_wecom_contact(gewe_core::AddWecomContactRequest {
            app_id: &app_id,
            v3: &v3,
            v4: &v4,
        })
        .await?;
    info!(%v3, "wecom contact add triggered");
    Ok(())
}

pub async fn handle_get_wecom_contact_detail(
    args: GetWecomContactDetailArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetWecomContactDetailArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        to_user_name,
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
        .get_wecom_contact_detail(gewe_core::GetWecomContactDetailRequest {
            app_id: &app_id,
            to_user_name: &to_user_name,
        })
        .await?;
    info!(nick=%resp.nick_name, "wecom contact detail fetched");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contacts_list_args() {
        let args = FetchContactsListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            base_url: None,
        };

        assert_eq!(args.token, Some("test_token".to_string()));
        assert_eq!(args.app_id, Some("test_app_id".to_string()));
        assert!(args.bot_app_id.is_none());
        assert!(args.bot_alias.is_none());
    }

    #[test]
    fn test_search_contacts_args() {
        let args = SearchContactsArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            contacts_info: "wxid_12345".to_string(),
            base_url: None,
        };

        assert_eq!(args.contacts_info, "wxid_12345");
    }

    #[test]
    fn test_add_contacts_args() {
        let args = AddContactsArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            scene: 1,
            option: 2,
            v3: "v3_value".to_string(),
            v4: "v4_value".to_string(),
            content: "hello".to_string(),
            base_url: None,
        };

        assert_eq!(args.scene, 1);
        assert_eq!(args.option, 2);
        assert_eq!(args.v3, "v3_value");
        assert_eq!(args.v4, "v4_value");
        assert_eq!(args.content, "hello");
    }

    #[test]
    fn test_set_friend_remark_args() {
        let args = SetFriendRemarkArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxid: "wxid_test".to_string(),
            remark: "好友备注".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_test");
        assert_eq!(args.remark, "好友备注");
    }

    #[test]
    fn test_set_friend_permissions_args() {
        let args = SetFriendPermissionsArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxid: "wxid_test".to_string(),
            only_chat: true,
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_test");
        assert!(args.only_chat);
    }

    #[test]
    fn test_delete_friend_args() {
        let args = DeleteFriendArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxid: "wxid_to_delete".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_to_delete");
    }

    #[test]
    fn test_check_contact_relation_args() {
        let args = CheckContactRelationArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxids: vec![
                "wxid1".to_string(),
                "wxid2".to_string(),
                "wxid3".to_string(),
            ],
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 3);
        assert_eq!(args.wxids[0], "wxid1");
        assert_eq!(args.wxids[1], "wxid2");
    }

    #[test]
    fn test_get_contact_brief_info_args() {
        let args = GetContactBriefInfoArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxids: vec!["wxid_a".to_string(), "wxid_b".to_string()],
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 2);
        assert!(args.wxids.contains(&"wxid_a".to_string()));
    }

    #[test]
    fn test_get_contact_detail_info_args() {
        let args = GetContactDetailInfoArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxids: vec!["wxid_detail".to_string()],
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 1);
        assert_eq!(args.wxids[0], "wxid_detail");
    }

    #[test]
    fn test_get_phone_address_list_args_empty_phones() {
        let args = GetPhoneAddressListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            phones: vec![],
            base_url: None,
        };

        assert!(args.phones.is_empty());
    }

    #[test]
    fn test_get_phone_address_list_args_with_phones() {
        let args = GetPhoneAddressListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            phones: vec!["13800138000".to_string(), "13900139000".to_string()],
            base_url: None,
        };

        assert_eq!(args.phones.len(), 2);
        assert_eq!(args.phones[0], "13800138000");
    }

    #[test]
    fn test_upload_phone_address_list_args() {
        let args = UploadPhoneAddressListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            phones: vec!["13800138000".to_string()],
            op_type: 1,
            base_url: None,
        };

        assert_eq!(args.phones.len(), 1);
        assert_eq!(args.op_type, 1);
    }

    #[test]
    fn test_search_wecom_contact_args() {
        let args = SearchWecomContactArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            scene: 3,
            content: "wecom_user".to_string(),
            base_url: None,
        };

        assert_eq!(args.scene, 3);
        assert_eq!(args.content, "wecom_user");
    }

    #[test]
    fn test_add_wecom_contact_args() {
        let args = AddWecomContactArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            v3: "wecom_v3".to_string(),
            v4: "wecom_v4".to_string(),
            base_url: None,
        };

        assert_eq!(args.v3, "wecom_v3");
        assert_eq!(args.v4, "wecom_v4");
    }

    #[test]
    fn test_get_wecom_contact_detail_args() {
        let args = GetWecomContactDetailArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            to_user_name: "wecom_username".to_string(),
            base_url: None,
        };

        assert_eq!(args.to_user_name, "wecom_username");
    }

    #[test]
    fn test_resolve_bot_with_explicit() {
        let config = CliConfig::default();
        let result = resolve_bot(None, Some("explicit_app_id".to_string()), &config).unwrap();

        assert_eq!(result, Some("explicit_app_id".to_string()));
    }

    #[test]
    fn test_resolve_bot_with_alias_found() {
        let mut config = CliConfig::default();
        config.bots.push(crate::config::BotRecord {
            app_id: "app123".to_string(),
            wxid: Some("wxid123".to_string()),
            alias: Some("my_bot".to_string()),
            token: None,
            webhook_secret: None,
        });

        let result = resolve_bot(Some("my_bot".to_string()), None, &config).unwrap();

        assert_eq!(result, Some("app123".to_string()));
    }

    #[test]
    fn test_resolve_bot_with_alias_not_found() {
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

        assert!(result.is_none());
    }

    #[test]
    fn test_args_with_bot_alias() {
        let args = FetchContactsListArgs {
            token: Some("test_token".to_string()),
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("bot1".to_string()),
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("bot1".to_string()));
    }

    #[test]
    fn test_args_with_bot_app_id() {
        let args = FetchContactsListArgs {
            token: Some("test_token".to_string()),
            app_id: None,
            bot_app_id: Some("bot_app_123".to_string()),
            bot_alias: None,
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("bot_app_123".to_string()));
    }

    #[test]
    fn test_check_contact_relation_args_empty_wxids() {
        let args = CheckContactRelationArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            wxids: vec![],
            base_url: None,
        };

        assert!(args.wxids.is_empty());
    }

    #[test]
    fn test_upload_phone_address_list_args_different_op_types() {
        let args1 = UploadPhoneAddressListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            phones: vec!["13800138000".to_string()],
            op_type: 1,
            base_url: None,
        };

        let args2 = UploadPhoneAddressListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            phones: vec!["13800138000".to_string()],
            op_type: 2,
            base_url: None,
        };

        assert_eq!(args1.op_type, 1);
        assert_eq!(args2.op_type, 2);
    }
}
