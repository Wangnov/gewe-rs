mod config;
mod contact;
mod favorite;
mod group;
mod login;
mod message;
mod moments;
mod personal;
mod tag;
mod video_account;
mod wait_reply;
mod webhook;

use anyhow::Result;
use clap::{ArgAction, Parser, Subcommand};
use config::{load_config, resolve_config_path};
use contact::{
    handle_add_contacts, handle_add_wecom_contact, handle_check_contact_relation,
    handle_delete_friend, handle_fetch_contacts_list, handle_fetch_contacts_list_cache,
    handle_get_contact_brief_info, handle_get_contact_detail_info, handle_get_phone_address_list,
    handle_get_wecom_contact_detail, handle_search_contacts, handle_search_wecom_contact,
    handle_set_friend_permissions, handle_set_friend_remark, handle_sync_wecom_contacts,
    handle_upload_phone_address_list, AddContactsArgs, AddWecomContactArgs,
    CheckContactRelationArgs, DeleteFriendArgs, FetchContactsListArgs, FetchContactsListCacheArgs,
    GetContactBriefInfoArgs, GetContactDetailInfoArgs, GetPhoneAddressListArgs,
    GetWecomContactDetailArgs, SearchContactsArgs, SearchWecomContactArgs,
    SetFriendPermissionsArgs, SetFriendRemarkArgs, SyncWecomContactsArgs,
    UploadPhoneAddressListArgs,
};
use favorite::{
    handle_delete_favorite, handle_get_favorite_content, handle_sync_favorites, DeleteFavoriteArgs,
    GetFavoriteContentArgs, SyncFavoriteArgs,
};
use group::{
    handle_add_group_member_as_friend, handle_admin_operate, handle_agree_join_room,
    handle_create_chatroom, handle_disband_chatroom, handle_get_chatroom_announcement,
    handle_get_chatroom_info, handle_get_chatroom_member_detail, handle_get_chatroom_member_list,
    handle_get_chatroom_qr_code, handle_invite_add_enter_room, handle_invite_member,
    handle_join_room_using_qr_code, handle_modify_chatroom_name,
    handle_modify_chatroom_nick_name_for_self, handle_modify_chatroom_remark, handle_pin_chat,
    handle_quit_chatroom, handle_remove_member, handle_room_access_apply_check_approve,
    handle_save_contract_list, handle_set_chatroom_announcement, handle_set_msg_silence,
};
use login::{
    handle_change_mac_to_ipad, handle_check_login, handle_check_online, handle_dialog_login,
    handle_get_login_qr, handle_login_by_account, handle_logout, handle_reconnection,
    handle_set_callback,
};
use message::{
    handle_download_cdn, handle_download_emoji, handle_download_file, handle_download_image,
    handle_download_video, handle_download_voice, handle_forward_file, handle_forward_image,
    handle_forward_mini_app, handle_forward_url, handle_forward_video, handle_revoke_msg,
    handle_send_appmsg, handle_send_emoji, handle_send_file, handle_send_image, handle_send_link,
    handle_send_mini_app, handle_send_name_card, handle_send_text, handle_send_video,
    handle_send_voice,
};
use personal::{
    handle_get_profile, handle_get_qr_code, handle_get_safety_info, handle_privacy_settings,
    handle_update_head_img, handle_update_profile, GetProfileArgs, GetQrCodeArgs,
    GetSafetyInfoArgs, PrivacySettingsArgs, UpdateHeadImgArgs, UpdateProfileArgs,
};
use tag::{
    handle_add_label, handle_delete_label, handle_list_labels, handle_modify_label_members,
    AddLabelArgs, DeleteLabelArgs, ListLabelArgs, ModifyLabelMembersArgs,
};
use video_account::{handle_video_account_command, VideoAccountCommands};

#[derive(Parser)]
#[command(name = "gewe", version, about = "GeWe CLI")]
struct Cli {
    /// 增加输出详细度，可重复：-v 为 debug，-vv 为 trace
    #[arg(long, short = 'v', global = true, action = ArgAction::Count)]
    verbose: u8,
    #[arg(long, global = true)]
    config: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 发送文字消息
    SendText(message::SendTextArgs),
    /// 发送图片消息
    SendImage(message::SendImageArgs),
    /// 发送语音消息
    SendVoice(message::SendVoiceArgs),
    /// 发送视频消息
    SendVideo(message::SendVideoArgs),
    /// 发送文件消息
    SendFile(message::SendFileArgs),
    /// 发送链接消息
    SendLink(message::SendLinkArgs),
    /// 发送 emoji 消息
    SendEmoji(message::SendEmojiArgs),
    /// 发送 appmsg 消息
    SendAppmsg(message::SendAppmsgArgs),
    /// 发送小程序消息
    SendMiniApp(message::SendMiniAppArgs),
    /// 发送名片消息
    SendNameCard(message::SendNameCardArgs),
    /// 转发图片
    ForwardImage(message::ForwardImageArgs),
    /// 转发视频
    ForwardVideo(message::ForwardVideoArgs),
    /// 转发文件
    ForwardFile(message::ForwardFileArgs),
    /// 转发小程序
    ForwardMiniApp(message::ForwardMiniAppArgs),
    /// 转发链接
    ForwardUrl(message::ForwardUrlArgs),
    /// 下载图片
    DownloadImage(message::DownloadImageArgs),
    /// 下载视频
    DownloadVideo(message::DownloadVideoArgs),
    /// 下载文件
    DownloadFile(message::DownloadFileArgs),
    /// 下载语音
    DownloadVoice(message::DownloadVoiceArgs),
    /// 下载 emoji
    DownloadEmoji(message::DownloadEmojiArgs),
    /// cdn 下载
    DownloadCdn(message::DownloadCdnArgs),
    /// 撤回消息
    RevokeMsg(message::RevokeMsgArgs),
    /// 获取通讯录列表
    FetchContactsList(FetchContactsListArgs),
    /// 获取通讯录列表缓存
    FetchContactsListCache(FetchContactsListCacheArgs),
    /// 搜索好友
    SearchContacts(SearchContactsArgs),
    /// 添加联系人/同意好友
    AddContacts(AddContactsArgs),
    /// 设置好友备注
    SetFriendRemark(SetFriendRemarkArgs),
    /// 设置好友仅聊天
    SetFriendPermissions(SetFriendPermissionsArgs),
    /// 删除好友
    DeleteFriend(DeleteFriendArgs),
    /// 检测好友关系
    CheckContactRelation(CheckContactRelationArgs),
    /// 获取好友/群简要信息
    GetContactBriefInfo(GetContactBriefInfoArgs),
    /// 获取好友/群详细信息
    GetContactDetailInfo(GetContactDetailInfoArgs),
    /// 获取手机通讯录
    GetPhoneAddressList(GetPhoneAddressListArgs),
    /// 上传手机通讯录
    UploadPhoneAddressList(UploadPhoneAddressListArgs),
    /// 搜索企微联系人
    SearchWecomContact(SearchWecomContactArgs),
    /// 同步企微好友
    SyncWecomContacts(SyncWecomContactsArgs),
    /// 添加企微好友
    AddWecomContact(AddWecomContactArgs),
    /// 获取企微好友详情
    GetWecomContactDetail(GetWecomContactDetailArgs),
    /// 发送文字朋友圈
    SendMomentText(SendMomentTextArgs),
    /// 发送图片朋友圈
    SendMomentImage(SendMomentImageArgs),
    /// 发送视频朋友圈
    SendMomentVideo(SendMomentVideoArgs),
    /// 发送链接朋友圈
    SendMomentLink(SendMomentLinkArgs),
    /// 转发朋友圈
    ForwardMoment(ForwardMomentArgs),
    /// 上传朋友圈图片
    UploadMomentImage(UploadMomentImageArgs),
    /// 上传朋友圈视频
    UploadMomentVideo(UploadMomentVideoArgs),
    /// 下载朋友圈视频
    DownloadMomentVideo(DownloadMomentVideoArgs),
    /// 删除朋友圈
    DeleteMoment(DeleteMomentArgs),
    /// 设置陌生人可见
    SetStrangerVisibility(SetStrangerVisibilityArgs),
    /// 获取朋友圈详情
    GetMomentDetail(GetMomentDetailArgs),
    /// 点赞/取消点赞
    LikeMoment(LikeMomentArgs),
    /// 评论/删除评论
    CommentMoment(CommentMomentArgs),
    /// 联系人的朋友圈列表
    GetContactMoments(GetContactMomentsArgs),
    /// 自己的朋友圈列表
    GetSelfMoments(GetSelfMomentsArgs),
    /// 设置朋友圈可见范围
    SetMomentVisibleScope(SetMomentVisibleScopeArgs),
    /// 设置某条朋友圈隐私
    SetMomentPrivacy(SetMomentPrivacyArgs),
    /// 获取个人资料
    GetProfile(GetProfileArgs),
    /// 更新个人资料
    UpdateProfile(UpdateProfileArgs),
    /// 更新头像
    UpdateHeadImg(UpdateHeadImgArgs),
    /// 获取自己的二维码
    GetQrCode(GetQrCodeArgs),
    /// 隐私设置
    PrivacySettings(PrivacySettingsArgs),
    /// 获取设备记录
    GetSafetyInfo(GetSafetyInfoArgs),
    /// 添加/查询标签
    AddLabel(AddLabelArgs),
    /// 删除标签
    DeleteLabel(DeleteLabelArgs),
    /// 标签列表
    ListLabels(ListLabelArgs),
    /// 修改好友标签
    ModifyLabelMembers(ModifyLabelMembersArgs),
    /// 同步收藏夹列表
    SyncFavorites(SyncFavoriteArgs),
    /// 获取收藏夹详情
    GetFavoriteContent(GetFavoriteContentArgs),
    /// 删除收藏夹
    DeleteFavorite(DeleteFavoriteArgs),
    /// 视频号模块命令
    VideoAccount {
        #[command(subcommand)]
        command: VideoAccountCommands,
    },
    /// 创建微信群
    CreateChatroom(group::CreateChatroomArgs),
    /// 解散群聊
    DisbandChatroom(group::DisbandChatroomArgs),
    /// 退出群聊
    QuitChatroom(group::QuitChatroomArgs),
    /// 修改群名称
    ModifyChatroomName(group::ModifyChatroomNameArgs),
    /// 修改群备注
    ModifyChatroomRemark(group::ModifyChatroomRemarkArgs),
    /// 修改我在群内的昵称
    ModifyChatroomNickNameForSelf(group::ModifyChatroomNickNameForSelfArgs),
    /// 邀请-添加进群
    InviteMember(group::InviteMemberArgs),
    /// 删除群成员
    RemoveMember(group::RemoveMemberArgs),
    /// 扫码进群
    JoinRoomUsingQrCode(group::JoinRoomUsingQrCodeArgs),
    /// 同意进群
    AgreeJoinRoom(group::AgreeJoinRoomArgs),
    /// 确认进群申请
    RoomAccessApplyCheckApprove(group::RoomAccessApplyCheckApproveArgs),
    /// 邀请-添加进群（exp 版）
    InviteAddEnterRoom(group::InviteAddEnterRoomArgs),
    /// 添加群成员为好友
    AddGroupMemberAsFriend(group::AddGroupMemberAsFriendArgs),
    /// 获取群成员列表
    GetChatroomMemberList(group::GetChatroomMemberListArgs),
    /// 获取群成员详情
    GetChatroomMemberDetail(group::GetChatroomMemberDetailArgs),
    /// 获取群信息
    GetChatroomInfo(group::GetChatroomInfoArgs),
    /// 获取群公告
    GetChatroomAnnouncement(group::GetChatroomAnnouncementArgs),
    /// 设置群公告
    SetChatroomAnnouncement(group::SetChatroomAnnouncementArgs),
    /// 获取群二维码
    GetChatroomQrCode(group::GetChatroomQrCodeArgs),
    /// 群保存到通讯录
    SaveContractList(group::SaveContractListArgs),
    /// 聊天置顶
    PinChat(group::PinChatArgs),
    /// 设置消息免打扰
    SetMsgSilence(group::SetMsgSilenceArgs),
    /// 管理员操作
    AdminOperate(group::AdminOperateArgs),
    /// 获取登录二维码
    GetLoginQr(login::GetLoginQrArgs),
    /// 执行登录检查
    CheckLogin(login::CheckLoginArgs),
    /// 弹框登录
    DialogLogin(login::DialogLoginArgs),
    /// 账号密码登录
    LoginByAccount(login::LoginByAccountArgs),
    /// 设置回调地址
    SetCallback(login::SetCallbackArgs),
    /// Mac 设备转 iPad 登录
    ChangeMacToIpad(login::ChangeMacToIpadArgs),
    /// 检查是否在线
    CheckOnline(login::CheckOnlineArgs),
    /// 断线重连
    Reconnection(login::ReconnectionArgs),
    /// 退出登录
    Logout(login::LogoutArgs),
    /// 查看或更新配置
    Config(config::ConfigArgs),
    /// 启动 webhook 服务器，接收并处理消息事件
    ServeWebhook(webhook::ServeWebhookArgs),
    /// 发送消息后等待特定用户回复
    WaitReply(wait_reply::WaitReplyArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let filter = get_log_filter(cli.verbose);
    tracing_subscriber::fmt().with_env_filter(filter).init();
    let config_path = resolve_config_path(cli.config.as_deref())?;
    let mut cfg = load_config(&config_path)?;

    match cli.command {
        Commands::SendText(args) => handle_send_text(args, &config_path, &mut cfg).await?,
        Commands::SendImage(args) => handle_send_image(args, &config_path, &mut cfg).await?,
        Commands::SendVoice(args) => handle_send_voice(args, &config_path, &mut cfg).await?,
        Commands::SendVideo(args) => handle_send_video(args, &config_path, &mut cfg).await?,
        Commands::SendFile(args) => handle_send_file(args, &config_path, &mut cfg).await?,
        Commands::SendLink(args) => handle_send_link(args, &config_path, &mut cfg).await?,
        Commands::SendEmoji(args) => handle_send_emoji(args, &config_path, &mut cfg).await?,
        Commands::SendAppmsg(args) => handle_send_appmsg(args, &config_path, &mut cfg).await?,
        Commands::SendMiniApp(args) => handle_send_mini_app(args, &config_path, &mut cfg).await?,
        Commands::SendNameCard(args) => handle_send_name_card(args, &config_path, &mut cfg).await?,
        Commands::ForwardImage(args) => handle_forward_image(args, &config_path, &mut cfg).await?,
        Commands::ForwardVideo(args) => handle_forward_video(args, &config_path, &mut cfg).await?,
        Commands::ForwardFile(args) => handle_forward_file(args, &config_path, &mut cfg).await?,
        Commands::ForwardMiniApp(args) => {
            handle_forward_mini_app(args, &config_path, &mut cfg).await?
        }
        Commands::ForwardUrl(args) => handle_forward_url(args, &config_path, &mut cfg).await?,
        Commands::DownloadImage(args) => {
            handle_download_image(args, &config_path, &mut cfg).await?
        }
        Commands::DownloadVideo(args) => {
            handle_download_video(args, &config_path, &mut cfg).await?
        }
        Commands::DownloadFile(args) => handle_download_file(args, &config_path, &mut cfg).await?,
        Commands::DownloadVoice(args) => {
            handle_download_voice(args, &config_path, &mut cfg).await?
        }
        Commands::DownloadEmoji(args) => {
            handle_download_emoji(args, &config_path, &mut cfg).await?
        }
        Commands::DownloadCdn(args) => handle_download_cdn(args, &config_path, &mut cfg).await?,
        Commands::RevokeMsg(args) => handle_revoke_msg(args, &config_path, &mut cfg).await?,
        Commands::FetchContactsList(args) => {
            handle_fetch_contacts_list(args, &config_path, &mut cfg).await?
        }
        Commands::FetchContactsListCache(args) => {
            handle_fetch_contacts_list_cache(args, &config_path, &mut cfg).await?
        }
        Commands::SearchContacts(args) => {
            handle_search_contacts(args, &config_path, &mut cfg).await?
        }
        Commands::AddContacts(args) => handle_add_contacts(args, &config_path, &mut cfg).await?,
        Commands::SetFriendRemark(args) => {
            handle_set_friend_remark(args, &config_path, &mut cfg).await?
        }
        Commands::SetFriendPermissions(args) => {
            handle_set_friend_permissions(args, &config_path, &mut cfg).await?
        }
        Commands::DeleteFriend(args) => handle_delete_friend(args, &config_path, &mut cfg).await?,
        Commands::CheckContactRelation(args) => {
            handle_check_contact_relation(args, &config_path, &mut cfg).await?
        }
        Commands::GetContactBriefInfo(args) => {
            handle_get_contact_brief_info(args, &config_path, &mut cfg).await?
        }
        Commands::GetContactDetailInfo(args) => {
            handle_get_contact_detail_info(args, &config_path, &mut cfg).await?
        }
        Commands::GetPhoneAddressList(args) => {
            handle_get_phone_address_list(args, &config_path, &mut cfg).await?
        }
        Commands::UploadPhoneAddressList(args) => {
            handle_upload_phone_address_list(args, &config_path, &mut cfg).await?
        }
        Commands::SearchWecomContact(args) => {
            handle_search_wecom_contact(args, &config_path, &mut cfg).await?
        }
        Commands::SyncWecomContacts(args) => {
            handle_sync_wecom_contacts(args, &config_path, &mut cfg).await?
        }
        Commands::AddWecomContact(args) => {
            handle_add_wecom_contact(args, &config_path, &mut cfg).await?
        }
        Commands::GetWecomContactDetail(args) => {
            handle_get_wecom_contact_detail(args, &config_path, &mut cfg).await?
        }
        Commands::SendMomentText(args) => {
            handle_send_moment_text(args, &config_path, &mut cfg).await?
        }
        Commands::SendMomentImage(args) => {
            handle_send_moment_image(args, &config_path, &mut cfg).await?
        }
        Commands::SendMomentVideo(args) => {
            handle_send_moment_video(args, &config_path, &mut cfg).await?
        }
        Commands::SendMomentLink(args) => {
            handle_send_moment_link(args, &config_path, &mut cfg).await?
        }
        Commands::ForwardMoment(args) => {
            handle_forward_moment(args, &config_path, &mut cfg).await?
        }
        Commands::UploadMomentImage(args) => {
            handle_upload_moment_image(args, &config_path, &mut cfg).await?
        }
        Commands::UploadMomentVideo(args) => {
            handle_upload_moment_video(args, &config_path, &mut cfg).await?
        }
        Commands::DownloadMomentVideo(args) => {
            handle_download_moment_video(args, &config_path, &mut cfg).await?
        }
        Commands::DeleteMoment(args) => handle_delete_moment(args, &config_path, &mut cfg).await?,
        Commands::SetStrangerVisibility(args) => {
            handle_set_stranger_visibility(args, &config_path, &mut cfg).await?
        }
        Commands::GetMomentDetail(args) => {
            handle_get_moment_detail(args, &config_path, &mut cfg).await?
        }
        Commands::LikeMoment(args) => handle_like_moment(args, &config_path, &mut cfg).await?,
        Commands::CommentMoment(args) => {
            handle_comment_moment(args, &config_path, &mut cfg).await?
        }
        Commands::GetContactMoments(args) => {
            handle_get_contact_moments(args, &config_path, &mut cfg).await?
        }
        Commands::GetSelfMoments(args) => {
            handle_get_self_moments(args, &config_path, &mut cfg).await?
        }
        Commands::SetMomentVisibleScope(args) => {
            handle_set_moment_visible_scope(args, &config_path, &mut cfg).await?
        }
        Commands::SetMomentPrivacy(args) => {
            handle_set_moment_privacy(args, &config_path, &mut cfg).await?
        }
        Commands::GetProfile(args) => handle_get_profile(args, &config_path, &mut cfg).await?,
        Commands::UpdateProfile(args) => {
            handle_update_profile(args, &config_path, &mut cfg).await?
        }
        Commands::UpdateHeadImg(args) => {
            handle_update_head_img(args, &config_path, &mut cfg).await?
        }
        Commands::GetQrCode(args) => handle_get_qr_code(args, &config_path, &mut cfg).await?,
        Commands::PrivacySettings(args) => {
            handle_privacy_settings(args, &config_path, &mut cfg).await?
        }
        Commands::GetSafetyInfo(args) => {
            handle_get_safety_info(args, &config_path, &mut cfg).await?
        }
        Commands::GetLoginQr(args) => handle_get_login_qr(args, &config_path, &mut cfg).await?,
        Commands::CheckLogin(args) => handle_check_login(args, &config_path, &mut cfg).await?,
        Commands::DialogLogin(args) => handle_dialog_login(args, &config_path, &mut cfg).await?,
        Commands::LoginByAccount(args) => {
            handle_login_by_account(args, &config_path, &mut cfg).await?
        }
        Commands::SetCallback(args) => handle_set_callback(args, &config_path, &mut cfg).await?,
        Commands::ChangeMacToIpad(args) => {
            handle_change_mac_to_ipad(args, &config_path, &mut cfg).await?
        }
        Commands::CheckOnline(args) => handle_check_online(args, &config_path, &mut cfg).await?,
        Commands::Reconnection(args) => handle_reconnection(args, &config_path, &mut cfg).await?,
        Commands::Logout(args) => handle_logout(args, &config_path, &mut cfg).await?,
        Commands::AddLabel(args) => handle_add_label(args, &config_path, &mut cfg).await?,
        Commands::DeleteLabel(args) => handle_delete_label(args, &config_path, &mut cfg).await?,
        Commands::ListLabels(args) => handle_list_labels(args, &config_path, &mut cfg).await?,
        Commands::ModifyLabelMembers(args) => {
            handle_modify_label_members(args, &config_path, &mut cfg).await?
        }
        Commands::SyncFavorites(args) => {
            handle_sync_favorites(args, &config_path, &mut cfg).await?
        }
        Commands::GetFavoriteContent(args) => {
            handle_get_favorite_content(args, &config_path, &mut cfg).await?
        }
        Commands::DeleteFavorite(args) => {
            handle_delete_favorite(args, &config_path, &mut cfg).await?
        }
        Commands::VideoAccount { command } => {
            handle_video_account_command(command, &config_path, &mut cfg).await?
        }
        Commands::CreateChatroom(args) => {
            handle_create_chatroom(args, &config_path, &mut cfg).await?
        }
        Commands::DisbandChatroom(args) => {
            handle_disband_chatroom(args, &config_path, &mut cfg).await?
        }
        Commands::QuitChatroom(args) => handle_quit_chatroom(args, &config_path, &mut cfg).await?,
        Commands::ModifyChatroomName(args) => {
            handle_modify_chatroom_name(args, &config_path, &mut cfg).await?
        }
        Commands::ModifyChatroomRemark(args) => {
            handle_modify_chatroom_remark(args, &config_path, &mut cfg).await?
        }
        Commands::ModifyChatroomNickNameForSelf(args) => {
            handle_modify_chatroom_nick_name_for_self(args, &config_path, &mut cfg).await?
        }
        Commands::InviteMember(args) => handle_invite_member(args, &config_path, &mut cfg).await?,
        Commands::RemoveMember(args) => handle_remove_member(args, &config_path, &mut cfg).await?,
        Commands::JoinRoomUsingQrCode(args) => {
            handle_join_room_using_qr_code(args, &config_path, &mut cfg).await?
        }
        Commands::AgreeJoinRoom(args) => {
            handle_agree_join_room(args, &config_path, &mut cfg).await?
        }
        Commands::RoomAccessApplyCheckApprove(args) => {
            handle_room_access_apply_check_approve(args, &config_path, &mut cfg).await?
        }
        Commands::InviteAddEnterRoom(args) => {
            handle_invite_add_enter_room(args, &config_path, &mut cfg).await?
        }
        Commands::AddGroupMemberAsFriend(args) => {
            handle_add_group_member_as_friend(args, &config_path, &mut cfg).await?
        }
        Commands::GetChatroomMemberList(args) => {
            handle_get_chatroom_member_list(args, &config_path, &mut cfg).await?
        }
        Commands::GetChatroomMemberDetail(args) => {
            handle_get_chatroom_member_detail(args, &config_path, &mut cfg).await?
        }
        Commands::GetChatroomInfo(args) => {
            handle_get_chatroom_info(args, &config_path, &mut cfg).await?
        }
        Commands::GetChatroomAnnouncement(args) => {
            handle_get_chatroom_announcement(args, &config_path, &mut cfg).await?
        }
        Commands::SetChatroomAnnouncement(args) => {
            handle_set_chatroom_announcement(args, &config_path, &mut cfg).await?
        }
        Commands::GetChatroomQrCode(args) => {
            handle_get_chatroom_qr_code(args, &config_path, &mut cfg).await?
        }
        Commands::SaveContractList(args) => {
            handle_save_contract_list(args, &config_path, &mut cfg).await?
        }
        Commands::PinChat(args) => handle_pin_chat(args, &config_path, &mut cfg).await?,
        Commands::SetMsgSilence(args) => {
            handle_set_msg_silence(args, &config_path, &mut cfg).await?
        }
        Commands::AdminOperate(args) => handle_admin_operate(args, &config_path, &mut cfg).await?,
        Commands::Config(args) => config::handle_config(args, &config_path, &mut cfg)?,
        Commands::ServeWebhook(args) => {
            webhook::handle_serve_webhook(args, &config_path, &cfg).await?
        }
        Commands::WaitReply(args) => {
            wait_reply::handle_wait_reply(args, &config_path, &cfg).await?;
        }
    }
    Ok(())
}

fn get_log_filter(verbose: u8) -> &'static str {
    match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_filter_info() {
        assert_eq!(get_log_filter(0), "info");
    }

    #[test]
    fn test_get_log_filter_debug() {
        assert_eq!(get_log_filter(1), "debug");
    }

    #[test]
    fn test_get_log_filter_trace() {
        assert_eq!(get_log_filter(2), "trace");
        assert_eq!(get_log_filter(3), "trace");
        assert_eq!(get_log_filter(255), "trace");
    }
}

use moments::{
    handle_comment_moment, handle_delete_moment, handle_download_moment_video,
    handle_forward_moment, handle_get_contact_moments, handle_get_moment_detail,
    handle_get_self_moments, handle_like_moment, handle_send_moment_image, handle_send_moment_link,
    handle_send_moment_text, handle_send_moment_video, handle_set_moment_privacy,
    handle_set_moment_visible_scope, handle_set_stranger_visibility, handle_upload_moment_image,
    handle_upload_moment_video, CommentMomentArgs, DeleteMomentArgs, DownloadMomentVideoArgs,
    ForwardMomentArgs, GetContactMomentsArgs, GetMomentDetailArgs, GetSelfMomentsArgs,
    LikeMomentArgs, SendMomentImageArgs, SendMomentLinkArgs, SendMomentTextArgs,
    SendMomentVideoArgs, SetMomentPrivacyArgs, SetMomentVisibleScopeArgs,
    SetStrangerVisibilityArgs, UploadMomentImageArgs, UploadMomentVideoArgs,
};
