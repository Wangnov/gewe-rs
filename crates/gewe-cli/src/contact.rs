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
