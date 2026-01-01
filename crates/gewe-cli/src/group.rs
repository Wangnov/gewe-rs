use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct CreateChatroomArgs {
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
pub struct DisbandChatroomArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct QuitChatroomArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ModifyChatroomNameArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_name: String,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ModifyChatroomRemarkArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_remark: String,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ModifyChatroomNickNameForSelfArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub nick_name: String,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct InviteMemberArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub reason: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct RemoveMemberArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct JoinRoomUsingQrCodeArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub qr_uuid: String,
    #[arg(long)]
    pub chatroom_name: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct AgreeJoinRoomArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub msg_id: String,
    #[arg(long)]
    pub new_msg_id: String,
    #[arg(long)]
    pub create_time: String,
    #[arg(long)]
    pub from_username: String,
    #[arg(long)]
    pub to_username: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct RoomAccessApplyCheckApproveArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub ticket: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct InviteAddEnterRoomArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub exp_id: String,
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub wxids: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct AddGroupMemberAsFriendArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetChatroomMemberListArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetChatroomMemberDetailArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetChatroomInfoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetChatroomAnnouncementArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub r#type: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetChatroomAnnouncementArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetChatroomQrCodeArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SaveContractListArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub save: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct PinChatArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub add: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetMsgSilenceArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub switch_: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct AdminOperateArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub chatroom_id: String,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub is_admin: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_create_chatroom(
    args: CreateChatroomArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let CreateChatroomArgs {
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
    let client = GeweHttpClient::new(token, base_url)?;
    let resp = client
        .create_chatroom(gewe_core::CreateChatroomRequest {
            app_id: &app_id,
            wxids: wxids.iter().map(|s| s.as_str()).collect(),
        })
        .await?;
    info!(chatroom_id=%resp.chatroom_id, "chatroom created");
    Ok(())
}

pub async fn handle_disband_chatroom(
    args: DisbandChatroomArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DisbandChatroomArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .disband_chatroom(gewe_core::DisbandChatroomRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(%chatroom_id, "chatroom disbanded");
    Ok(())
}

pub async fn handle_quit_chatroom(
    args: QuitChatroomArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let QuitChatroomArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .quit_chatroom(gewe_core::QuitChatroomRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(%chatroom_id, "chatroom quitted");
    Ok(())
}

pub async fn handle_modify_chatroom_name(
    args: ModifyChatroomNameArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ModifyChatroomNameArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_name,
        chatroom_id,
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
        .modify_chatroom_name(gewe_core::ModifyChatroomNameRequest {
            app_id: &app_id,
            chatroom_name: &chatroom_name,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(%chatroom_id, "chatroom name modified");
    Ok(())
}

pub async fn handle_modify_chatroom_remark(
    args: ModifyChatroomRemarkArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ModifyChatroomRemarkArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_remark,
        chatroom_id,
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
        .modify_chatroom_remark(gewe_core::ModifyChatroomRemarkRequest {
            app_id: &app_id,
            chatroom_remark: &chatroom_remark,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(%chatroom_id, "chatroom remark modified");
    Ok(())
}

pub async fn handle_modify_chatroom_nick_name_for_self(
    args: ModifyChatroomNickNameForSelfArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ModifyChatroomNickNameForSelfArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        nick_name,
        chatroom_id,
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
        .modify_chatroom_nick_name_for_self(gewe_core::ModifyChatroomNickNameForSelfRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            nick_name: &nick_name,
        })
        .await?;
    info!(%chatroom_id, "self nickname modified");
    Ok(())
}

pub async fn handle_invite_member(
    args: InviteMemberArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let InviteMemberArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        wxids,
        reason,
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
        .invite_member(gewe_core::InviteMemberRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            reason: &reason,
            wxids: wxids.iter().map(|s| s.as_str()).collect(),
        })
        .await?;
    info!(%chatroom_id, invited=?wxids, %reason, "members invited");
    Ok(())
}

pub async fn handle_remove_member(
    args: RemoveMemberArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let RemoveMemberArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        wxids,
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
        .remove_member(gewe_core::RemoveMemberRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            wxids: wxids.iter().map(|s| s.as_str()).collect(),
        })
        .await?;
    info!(%chatroom_id, removed=?wxids, "members removed");
    Ok(())
}

pub async fn handle_join_room_using_qr_code(
    args: JoinRoomUsingQrCodeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let JoinRoomUsingQrCodeArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        qr_uuid,
        chatroom_name,
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
        .join_room_using_qr_code(gewe_core::JoinRoomUsingQrCodeRequest {
            app_id: &app_id,
            qr_uuid: &qr_uuid,
            chatroom_name: &chatroom_name,
        })
        .await?;
    info!(%chatroom_name, %qr_uuid, "join room using QR requested");
    Ok(())
}

pub async fn handle_agree_join_room(
    args: AgreeJoinRoomArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let AgreeJoinRoomArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        msg_id,
        new_msg_id,
        create_time,
        from_username,
        to_username,
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
        .agree_join_room(gewe_core::AgreeJoinRoomRequest {
            app_id: &app_id,
            msg_id: &msg_id,
            new_msg_id: &new_msg_id,
            create_time: &create_time,
            from_username: &from_username,
            to_username: &to_username,
        })
        .await?;
    info!(%msg_id, "join room agreed");
    Ok(())
}

pub async fn handle_room_access_apply_check_approve(
    args: RoomAccessApplyCheckApproveArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let RoomAccessApplyCheckApproveArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        wxid,
        ticket,
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
        .room_access_apply_check_approve(gewe_core::RoomAccessApplyCheckApproveRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            wxid: &wxid,
            ticket: &ticket,
        })
        .await?;
    info!(%chatroom_id, %wxid, "room access approved");
    Ok(())
}

pub async fn handle_invite_add_enter_room(
    args: InviteAddEnterRoomArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let InviteAddEnterRoomArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        exp_id,
        wxids,
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
        .invite_add_enter_room(gewe_core::InviteAddEnterRoomRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            exp_id: &exp_id,
            wxids: wxids.iter().map(|s| s.as_str()).collect(),
        })
        .await?;
    info!(%chatroom_id, invited=?wxids, "members invited (exp)");
    Ok(())
}

pub async fn handle_add_group_member_as_friend(
    args: AddGroupMemberAsFriendArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let AddGroupMemberAsFriendArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        wxid,
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
        .add_group_member_as_friend(gewe_core::AddGroupMemberAsFriendRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            wxid: &wxid,
            content: &content,
        })
        .await?;
    info!(%chatroom_id, %wxid, "friend request sent");
    Ok(())
}

pub async fn handle_get_chatroom_member_list(
    args: GetChatroomMemberListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetChatroomMemberListArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .get_chatroom_member_list(gewe_core::GetChatroomMemberListRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(owner=%resp.chat_room_owner, members=%resp.chatroom_members.len(), "member list fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_chatroom_member_detail(
    args: GetChatroomMemberDetailArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetChatroomMemberDetailArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
    let resp = client
        .get_chatroom_member_detail(gewe_core::GetChatroomMemberDetailRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            wxid: &wxid,
        })
        .await?;
    info!(member=%resp.wxid, nick=%resp.nick_name, "member detail fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_chatroom_info(
    args: GetChatroomInfoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetChatroomInfoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .get_chatroom_info(gewe_core::GetChatroomInfoRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(chatroom=%resp.chatroom_id, members=%resp.member_list.len(), "chatroom info fetched");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_chatroom_announcement(
    args: GetChatroomAnnouncementArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetChatroomAnnouncementArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        r#type,
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
        .get_chatroom_announcement(gewe_core::GetChatroomAnnouncementRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            r#type,
        })
        .await?;
    info!(%chatroom_id, sender=%resp.sender, "announcement fetched");
    println!("{}", resp.chat_room_announcement);
    Ok(())
}

pub async fn handle_set_chatroom_announcement(
    args: SetChatroomAnnouncementArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetChatroomAnnouncementArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .set_chatroom_announcement(gewe_core::SetChatroomAnnouncementRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            content: &content,
        })
        .await?;
    info!(%chatroom_id, "announcement set");
    Ok(())
}

pub async fn handle_get_chatroom_qr_code(
    args: GetChatroomQrCodeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetChatroomQrCodeArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
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
        .get_chatroom_qr_code(gewe_core::GetChatroomQrCodeRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
        })
        .await?;
    info!(%chatroom_id, "qr code fetched");
    println!("{}", resp.qr_img_base64);
    Ok(())
}

pub async fn handle_save_contract_list(
    args: SaveContractListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SaveContractListArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        save,
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
        .save_contract_list(gewe_core::SaveContractListRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            save,
        })
        .await?;
    info!(%chatroom_id, %save, "save to contacts set");
    Ok(())
}

pub async fn handle_pin_chat(
    args: PinChatArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let PinChatArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        add,
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
        .pin_chat(gewe_core::PinChatRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            add,
        })
        .await?;
    info!(%chatroom_id, %add, "pin chat set");
    Ok(())
}

pub async fn handle_set_msg_silence(
    args: SetMsgSilenceArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetMsgSilenceArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        switch_,
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
        .set_msg_silence(gewe_core::SetMsgSilenceRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            switch_,
        })
        .await?;
    info!(%chatroom_id, %switch_, "msg silence set");
    Ok(())
}

pub async fn handle_admin_operate(
    args: AdminOperateArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let AdminOperateArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        chatroom_id,
        wxid,
        is_admin,
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
        .admin_operate(gewe_core::AdminOperateRequest {
            app_id: &app_id,
            chatroom_id: &chatroom_id,
            wxid: &wxid,
            is_admin,
        })
        .await?;
    info!(%chatroom_id, %wxid, %is_admin, "admin operate done");
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
    fn test_create_chatroom_args() {
        let args = CreateChatroomArgs {
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
    }

    #[test]
    fn test_disband_chatroom_args() {
        let args = DisbandChatroomArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_123@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_123@chatroom");
    }

    #[test]
    fn test_quit_chatroom_args() {
        let args = QuitChatroomArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_456@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_456@chatroom");
    }

    #[test]
    fn test_modify_chatroom_name_args() {
        let args = ModifyChatroomNameArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_name: "新群名称".to_string(),
            chatroom_id: "chatroom_789@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_name, "新群名称");
        assert_eq!(args.chatroom_id, "chatroom_789@chatroom");
    }

    #[test]
    fn test_modify_chatroom_remark_args() {
        let args = ModifyChatroomRemarkArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_remark: "群备注".to_string(),
            chatroom_id: "chatroom_abc@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_remark, "群备注");
    }

    #[test]
    fn test_modify_chatroom_nick_name_for_self_args() {
        let args = ModifyChatroomNickNameForSelfArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            nick_name: "我的群昵称".to_string(),
            chatroom_id: "chatroom_xyz@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.nick_name, "我的群昵称");
    }

    #[test]
    fn test_invite_member_args() {
        let args = InviteMemberArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_invite@chatroom".to_string(),
            wxids: vec!["wxid_a".to_string(), "wxid_b".to_string()],
            reason: "邀请理由".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 2);
        assert_eq!(args.reason, "邀请理由");
    }

    #[test]
    fn test_remove_member_args() {
        let args = RemoveMemberArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_remove@chatroom".to_string(),
            wxids: vec!["wxid_to_remove".to_string()],
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 1);
        assert_eq!(args.wxids[0], "wxid_to_remove");
    }

    #[test]
    fn test_join_room_using_qr_code_args() {
        let args = JoinRoomUsingQrCodeArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            qr_uuid: "qr_uuid_123".to_string(),
            chatroom_name: "群名称".to_string(),
            base_url: None,
        };

        assert_eq!(args.qr_uuid, "qr_uuid_123");
        assert_eq!(args.chatroom_name, "群名称");
    }

    #[test]
    fn test_agree_join_room_args() {
        let args = AgreeJoinRoomArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            msg_id: "msg_123".to_string(),
            new_msg_id: "new_msg_456".to_string(),
            create_time: "1234567890".to_string(),
            from_username: "from_user".to_string(),
            to_username: "to_user".to_string(),
            base_url: None,
        };

        assert_eq!(args.msg_id, "msg_123");
        assert_eq!(args.new_msg_id, "new_msg_456");
        assert_eq!(args.create_time, "1234567890");
    }

    #[test]
    fn test_room_access_apply_check_approve_args() {
        let args = RoomAccessApplyCheckApproveArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_approve@chatroom".to_string(),
            wxid: "wxid_applicant".to_string(),
            ticket: "ticket_abc123".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_applicant");
        assert_eq!(args.ticket, "ticket_abc123");
    }

    #[test]
    fn test_invite_add_enter_room_args() {
        let args = InviteAddEnterRoomArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_exp@chatroom".to_string(),
            exp_id: "exp_123".to_string(),
            wxids: vec!["wxid1".to_string(), "wxid2".to_string()],
            base_url: None,
        };

        assert_eq!(args.exp_id, "exp_123");
        assert_eq!(args.wxids.len(), 2);
    }

    #[test]
    fn test_add_group_member_as_friend_args() {
        let args = AddGroupMemberAsFriendArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_friend@chatroom".to_string(),
            wxid: "wxid_target".to_string(),
            content: "添加好友验证消息".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_target");
        assert_eq!(args.content, "添加好友验证消息");
    }

    #[test]
    fn test_get_chatroom_member_list_args() {
        let args = GetChatroomMemberListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_members@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_members@chatroom");
    }

    #[test]
    fn test_get_chatroom_member_detail_args() {
        let args = GetChatroomMemberDetailArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_detail@chatroom".to_string(),
            wxid: "wxid_member".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_detail@chatroom");
        assert_eq!(args.wxid, "wxid_member");
    }

    #[test]
    fn test_get_chatroom_info_args() {
        let args = GetChatroomInfoArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_info@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_info@chatroom");
    }

    #[test]
    fn test_get_chatroom_announcement_args() {
        let args = GetChatroomAnnouncementArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_announcement@chatroom".to_string(),
            r#type: 1,
            base_url: None,
        };

        assert_eq!(args.r#type, 1);
    }

    #[test]
    fn test_set_chatroom_announcement_args() {
        let args = SetChatroomAnnouncementArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_set_ann@chatroom".to_string(),
            content: "新的群公告内容".to_string(),
            base_url: None,
        };

        assert_eq!(args.content, "新的群公告内容");
    }

    #[test]
    fn test_get_chatroom_qr_code_args() {
        let args = GetChatroomQrCodeArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_qr@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.chatroom_id, "chatroom_qr@chatroom");
    }

    #[test]
    fn test_save_contract_list_args() {
        let args = SaveContractListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_save@chatroom".to_string(),
            save: true,
            base_url: None,
        };

        assert!(args.save);
    }

    #[test]
    fn test_pin_chat_args() {
        let args = PinChatArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_pin@chatroom".to_string(),
            add: true,
            base_url: None,
        };

        assert!(args.add);
    }

    #[test]
    fn test_set_msg_silence_args() {
        let args = SetMsgSilenceArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_silence@chatroom".to_string(),
            switch_: true,
            base_url: None,
        };

        assert!(args.switch_);
    }

    #[test]
    fn test_admin_operate_args() {
        let args = AdminOperateArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom_admin@chatroom".to_string(),
            wxid: "wxid_new_admin".to_string(),
            is_admin: true,
            base_url: None,
        };

        assert_eq!(args.wxid, "wxid_new_admin");
        assert!(args.is_admin);
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
    fn test_create_chatroom_args_empty_wxids() {
        let args = CreateChatroomArgs {
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
    fn test_invite_member_args_single_member() {
        let args = InviteMemberArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom@chatroom".to_string(),
            wxids: vec!["single_wxid".to_string()],
            reason: "reason".to_string(),
            base_url: None,
        };

        assert_eq!(args.wxids.len(), 1);
    }

    #[test]
    fn test_args_with_bot_alias() {
        let args = CreateChatroomArgs {
            token: Some("test_token".to_string()),
            app_id: None,
            bot_app_id: None,
            bot_alias: Some("bot_alias".to_string()),
            wxids: vec!["wxid1".to_string()],
            base_url: None,
        };

        assert_eq!(args.bot_alias, Some("bot_alias".to_string()));
    }

    #[test]
    fn test_args_with_bot_app_id() {
        let args = DisbandChatroomArgs {
            token: Some("test_token".to_string()),
            app_id: None,
            bot_app_id: Some("bot_app_456".to_string()),
            bot_alias: None,
            chatroom_id: "chatroom@chatroom".to_string(),
            base_url: None,
        };

        assert_eq!(args.bot_app_id, Some("bot_app_456".to_string()));
    }

    #[test]
    fn test_save_contract_list_args_false() {
        let args = SaveContractListArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom@chatroom".to_string(),
            save: false,
            base_url: None,
        };

        assert!(!args.save);
    }

    #[test]
    fn test_pin_chat_args_remove() {
        let args = PinChatArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom@chatroom".to_string(),
            add: false,
            base_url: None,
        };

        assert!(!args.add);
    }

    #[test]
    fn test_admin_operate_args_revoke_admin() {
        let args = AdminOperateArgs {
            token: Some("test_token".to_string()),
            app_id: Some("test_app_id".to_string()),
            bot_app_id: None,
            bot_alias: None,
            chatroom_id: "chatroom@chatroom".to_string(),
            wxid: "wxid_admin".to_string(),
            is_admin: false,
            base_url: None,
        };

        assert!(!args.is_admin);
    }
}
