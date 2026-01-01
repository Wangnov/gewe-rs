use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use gewe_core::{
    BrowseFinderRequest, CommentFinderRequest, CommentListRequest, ContactListRequest,
    CreateFinderRequest, FinderOptRequest, FinderProfileInfo, FinderSearchInfo, FinderVideoCdn,
    FollowFinderRequest, FollowListRequest, GetFinderProfileRequest, GetFinderQrCodeRequest,
    GetFinderQrCodeResponse, IdFavRequest, IdLikeRequest, LikeFavListRequest, MentionListRequest,
    PostPrivateLetterImgRequest, PostPrivateLetterRequest, PublishFinderCdnRequest,
    PublishFinderWebRequest, ScanBrowseRequest, ScanCommentRequest, ScanFavRequest,
    ScanFollowRequest, ScanLikeRequest, ScanLoginChannelsRequest, ScanQrCodeRequest,
    SearchFinderRequest, SearchFollowRequest, SendFinderMsgRequest, SendFinderSnsRequest,
    SyncPrivateLetterMsgRequest, UpdateFinderProfileRequest, UploadFinderVideoRequest,
    UserPageRequest, UserPageResponse,
};
use gewe_http::GeweHttpClient;
use serde_json::to_string_pretty;
use std::path::Path;
use tracing::info;

#[derive(Args, Clone)]
pub struct VideoAccountBaseArgs {
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

#[derive(Subcommand)]
pub enum VideoAccountCommands {
    UploadFinderVideo(UploadFinderVideoArgs),
    PublishFinderCdn(PublishFinderCdnArgs),
    PublishFinderWeb(PublishFinderWebArgs),
    SendFinderSns(SendFinderSnsArgs),
    SendFinderMsg(SendFinderMsgArgs),
    FollowFinder(FollowFinderArgs),
    FollowList(FollowListArgs),
    SearchFollow(SearchFollowArgs),
    SearchFinder(SearchFinderArgs),
    ScanFollow(ScanFollowArgs),
    ScanBrowse(ScanBrowseArgs),
    ScanLike(ScanLikeArgs),
    ScanFav(ScanFavArgs),
    ScanComment(ScanCommentArgs),
    ScanQrCode(ScanQrCodeArgs),
    ScanLoginChannels(ScanLoginChannelsArgs),
    IdFav(IdFavArgs),
    IdLike(IdLikeArgs),
    FinderOpt(FinderOptArgs),
    BrowseFinder(BrowseFinderArgs),
    LikeFavList(LikeFavListArgs),
    CommentFinder(CommentFinderArgs),
    CommentList(CommentListArgs),
    MentionList(MentionListArgs),
    ContactList(ContactListArgs),
    PostPrivateLetter(PostPrivateLetterArgs),
    PostPrivateLetterImg(PostPrivateLetterImgArgs),
    SyncPrivateLetterMsg(SyncPrivateLetterMsgArgs),
    CreateFinder(CreateFinderArgs),
    UpdateFinderProfile(UpdateFinderProfileArgs),
    GetFinderProfile(GetFinderProfileArgs),
    GetFinderQrCode(GetFinderQrCodeArgs),
    UserPage(UserPageArgs),
}

pub async fn handle_video_account_command(
    command: VideoAccountCommands,
    config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    match command {
        VideoAccountCommands::UploadFinderVideo(args) => {
            handle_upload_finder_video(args, config_path, config).await
        }
        VideoAccountCommands::PublishFinderCdn(args) => {
            handle_publish_finder_cdn(args, config_path, config).await
        }
        VideoAccountCommands::PublishFinderWeb(args) => {
            handle_publish_finder_web(args, config_path, config).await
        }
        VideoAccountCommands::SendFinderSns(args) => {
            handle_send_finder_sns(args, config_path, config).await
        }
        VideoAccountCommands::SendFinderMsg(args) => {
            handle_send_finder_msg(args, config_path, config).await
        }
        VideoAccountCommands::FollowFinder(args) => {
            handle_follow_finder(args, config_path, config).await
        }
        VideoAccountCommands::FollowList(args) => {
            handle_follow_list(args, config_path, config).await
        }
        VideoAccountCommands::SearchFollow(args) => {
            handle_search_follow(args, config_path, config).await
        }
        VideoAccountCommands::SearchFinder(args) => {
            handle_search_finder(args, config_path, config).await
        }
        VideoAccountCommands::ScanFollow(args) => {
            handle_scan_follow(args, config_path, config).await
        }
        VideoAccountCommands::ScanBrowse(args) => {
            handle_scan_browse(args, config_path, config).await
        }
        VideoAccountCommands::ScanLike(args) => handle_scan_like(args, config_path, config).await,
        VideoAccountCommands::ScanFav(args) => handle_scan_fav(args, config_path, config).await,
        VideoAccountCommands::ScanComment(args) => {
            handle_scan_comment(args, config_path, config).await
        }
        VideoAccountCommands::ScanQrCode(args) => {
            handle_scan_qr_code(args, config_path, config).await
        }
        VideoAccountCommands::ScanLoginChannels(args) => {
            handle_scan_login_channels(args, config_path, config).await
        }
        VideoAccountCommands::IdFav(args) => handle_id_fav(args, config_path, config).await,
        VideoAccountCommands::IdLike(args) => handle_id_like(args, config_path, config).await,
        VideoAccountCommands::FinderOpt(args) => handle_finder_opt(args, config_path, config).await,
        VideoAccountCommands::BrowseFinder(args) => {
            handle_browse_finder(args, config_path, config).await
        }
        VideoAccountCommands::LikeFavList(args) => {
            handle_like_fav_list(args, config_path, config).await
        }
        VideoAccountCommands::CommentFinder(args) => {
            handle_comment_finder(args, config_path, config).await
        }
        VideoAccountCommands::CommentList(args) => {
            handle_comment_list(args, config_path, config).await
        }
        VideoAccountCommands::MentionList(args) => {
            handle_mention_list(args, config_path, config).await
        }
        VideoAccountCommands::ContactList(args) => {
            handle_contact_list(args, config_path, config).await
        }
        VideoAccountCommands::PostPrivateLetter(args) => {
            handle_post_private_letter(args, config_path, config).await
        }
        VideoAccountCommands::PostPrivateLetterImg(args) => {
            handle_post_private_letter_img(args, config_path, config).await
        }
        VideoAccountCommands::SyncPrivateLetterMsg(args) => {
            handle_sync_private_letter_msg(args, config_path, config).await
        }
        VideoAccountCommands::CreateFinder(args) => {
            handle_create_finder(args, config_path, config).await
        }
        VideoAccountCommands::UpdateFinderProfile(args) => {
            handle_update_finder_profile(args, config_path, config).await
        }
        VideoAccountCommands::GetFinderProfile(args) => {
            handle_get_finder_profile(args, config_path, config).await
        }
        VideoAccountCommands::GetFinderQrCode(args) => {
            handle_get_finder_qr_code(args, config_path, config).await
        }
        VideoAccountCommands::UserPage(args) => handle_user_page(args, config_path, config).await,
    }
}

#[derive(Args, Clone)]
pub struct UploadFinderVideoArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub video_url: String,
    #[arg(long)]
    pub cover_img_url: String,
}

#[derive(Args, Clone)]
pub struct PublishFinderCdnArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long, value_delimiter = ',')]
    pub topic: Vec<String>,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub file_url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub mp4_identify: String,
    #[arg(long)]
    pub file_size: i64,
    #[arg(long)]
    pub thumb_md5: String,
    #[arg(long)]
    pub file_key: String,
}

#[derive(Args, Clone)]
pub struct PublishFinderWebArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub video_url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub description: String,
}

#[derive(Args, Clone)]
pub struct SendFinderSnsArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long, value_delimiter = ',')]
    pub allow_wx_ids: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub at_wx_ids: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub disable_wx_ids: Vec<String>,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub nickname: String,
    #[arg(long)]
    pub head_url: String,
    #[arg(long)]
    pub nonce_id: String,
    #[arg(long)]
    pub media_type: String,
    #[arg(long)]
    pub width: String,
    #[arg(long)]
    pub height: String,
    #[arg(long)]
    pub url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub thumb_url_token: String,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub video_play_len: String,
}

#[derive(Args, Clone)]
pub struct SendFinderMsgArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub to_wxid: String,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub nickname: String,
    #[arg(long)]
    pub head_url: String,
    #[arg(long)]
    pub nonce_id: String,
    #[arg(long)]
    pub media_type: String,
    #[arg(long)]
    pub width: String,
    #[arg(long)]
    pub height: String,
    #[arg(long)]
    pub url: String,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub thumb_url_token: String,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub video_play_len: String,
}

#[derive(Args, Clone)]
pub struct FollowFinderArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub op_type: i32,
    #[arg(long)]
    pub search_cookies: Option<String>,
    #[arg(long)]
    pub search_doc_id: Option<String>,
    #[arg(long)]
    pub search_id: Option<String>,
}

#[derive(Args, Clone)]
pub struct FollowListArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub last_buffer: Option<String>,
}

#[derive(Args, Clone)]
pub struct SearchFollowArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub keyword: String,
}

#[derive(Args, Clone)]
pub struct SearchFinderArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub category: Option<i32>,
    #[arg(long)]
    pub filter: Option<i32>,
    #[arg(long)]
    pub page: Option<i32>,
    #[arg(long)]
    pub cookie: Option<String>,
    #[arg(long)]
    pub search_id: Option<String>,
    #[arg(long)]
    pub offset: Option<i32>,
}

#[derive(Args, Clone)]
pub struct ScanFollowArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub proxy_ip: String,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub qr_content: String,
    #[arg(long)]
    pub object_id: String,
    #[arg(long)]
    pub object_nonce_id: String,
}

#[derive(Args, Clone)]
pub struct ScanBrowseArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub qr_content: String,
    #[arg(long)]
    pub object_id: i64,
}

#[derive(Args, Clone)]
pub struct ScanLikeArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub qr_content: String,
    #[arg(long)]
    pub object_id: i64,
}

pub type ScanFavArgs = ScanLikeArgs;

#[derive(Args, Clone)]
pub struct ScanCommentArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub qr_content: String,
    #[arg(long)]
    pub object_id: i64,
    #[arg(long)]
    pub comment_content: String,
    #[arg(long)]
    pub reply_username: Option<String>,
    #[arg(long)]
    pub ref_comment_id: Option<i64>,
    #[arg(long)]
    pub root_comment_id: Option<i64>,
}

#[derive(Args, Clone)]
pub struct ScanQrCodeArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub qr_content: String,
}

#[derive(Args, Clone)]
pub struct ScanLoginChannelsArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub qr_content: String,
}

#[derive(Args, Clone)]
pub struct IdFavArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub op_type: i32,
    #[arg(long)]
    pub object_nonce_id: String,
    #[arg(long)]
    pub session_buffer: String,
    #[arg(long)]
    pub object_id: i64,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
}

pub type IdLikeArgs = IdFavArgs;

#[derive(Args, Clone)]
pub struct FinderOptArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub op_type: i32,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub remain: i32,
}

#[derive(Args, Clone)]
pub struct BrowseFinderArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub object_id: i64,
    #[arg(long)]
    pub session_buffer: Option<String>,
    #[arg(long)]
    pub object_nonce_id: String,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
}

#[derive(Args, Clone)]
pub struct LikeFavListArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub last_buffer: Option<String>,
    #[arg(long)]
    pub flag: i32,
}

#[derive(Args, Clone)]
pub struct CommentFinderArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub proxy_ip: String,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub op_type: i32,
    #[arg(long)]
    pub object_nonce_id: String,
    #[arg(long)]
    pub session_buffer: String,
    #[arg(long)]
    pub object_id: i64,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub comment_id: String,
    #[arg(long)]
    pub reply_user_name: String,
    #[arg(long)]
    pub ref_comment_id: i64,
    #[arg(long)]
    pub root_comment_id: i64,
}

#[derive(Args, Clone)]
pub struct CommentListArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub object_id: i64,
    #[arg(long)]
    pub last_buffer: Option<String>,
    #[arg(long)]
    pub session_buffer: String,
    #[arg(long)]
    pub object_nonce_id: Option<String>,
    #[arg(long)]
    pub ref_comment_id: Option<i64>,
    #[arg(long)]
    pub root_comment_id: Option<i64>,
}

#[derive(Args, Clone)]
pub struct MentionListArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub req_scene: i32,
    #[arg(long)]
    pub last_buff: String,
}

#[derive(Args, Clone)]
pub struct ContactListArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
    #[arg(long)]
    pub query_info: String,
}

#[derive(Args, Clone)]
pub struct PostPrivateLetterArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub content: String,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub msg_session_id: String,
}

#[derive(Args, Clone)]
pub struct PostPrivateLetterImgArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub msg_session_id: String,
    #[arg(long)]
    pub img_url: String,
}

#[derive(Args, Clone)]
pub struct SyncPrivateLetterMsgArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub key_buff: Option<String>,
}

#[derive(Args, Clone)]
pub struct CreateFinderArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub nick_name: String,
    #[arg(long)]
    pub head_img: String,
    #[arg(long)]
    pub signature: Option<String>,
    #[arg(long)]
    pub sex: Option<i32>,
}

#[derive(Args, Clone)]
pub struct UpdateFinderProfileArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub nick_name: Option<String>,
    #[arg(long)]
    pub head_img: Option<String>,
    #[arg(long)]
    pub signature: Option<String>,
    #[arg(long)]
    pub sex: Option<i32>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub province: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
}

#[derive(Args, Clone)]
pub struct GetFinderProfileArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
}

#[derive(Args, Clone)]
pub struct GetFinderQrCodeArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub my_user_name: String,
    #[arg(long)]
    pub my_role_type: i32,
}

#[derive(Args, Clone)]
pub struct UserPageArgs {
    #[command(flatten)]
    pub base: VideoAccountBaseArgs,
    #[arg(long)]
    pub to_user_name: String,
    #[arg(long)]
    pub last_buffer: Option<String>,
    #[arg(long)]
    pub max_id: Option<i64>,
    #[arg(long)]
    pub search_cookies: Option<String>,
    #[arg(long)]
    pub search_id: Option<String>,
}

async fn resolve_client(
    base: &VideoAccountBaseArgs,
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

async fn handle_upload_finder_video(
    args: UploadFinderVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .upload_finder_video(UploadFinderVideoRequest {
            app_id: &app_id,
            video_url: &args.video_url,
            cover_img_url: &args.cover_img_url,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_publish_finder_cdn(
    args: PublishFinderCdnArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let video_cdn = FinderVideoCdn {
        file_url: args.file_url,
        thumb_url: args.thumb_url,
        mp4_identify: args.mp4_identify,
        file_size: args.file_size,
        thumb_md5: args.thumb_md5,
        file_key: args.file_key,
    };
    let resp = client
        .publish_finder_cdn(PublishFinderCdnRequest {
            app_id: &app_id,
            topic: args.topic.iter().map(|s| s.as_str()).collect(),
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            description: &args.description,
            video_cdn,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_publish_finder_web(
    args: PublishFinderWebArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .publish_finder_web(PublishFinderWebRequest {
            app_id: &app_id,
            title: &args.title,
            video_url: &args.video_url,
            thumb_url: &args.thumb_url,
            description: &args.description,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_send_finder_sns(
    args: SendFinderSnsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .send_finder_sns(SendFinderSnsRequest {
            app_id: &app_id,
            allow_wx_ids: args.allow_wx_ids.iter().map(|s| s.as_str()).collect(),
            at_wx_ids: args.at_wx_ids.iter().map(|s| s.as_str()).collect(),
            disable_wx_ids: args.disable_wx_ids.iter().map(|s| s.as_str()).collect(),
            id: args.id,
            username: &args.username,
            nickname: &args.nickname,
            head_url: &args.head_url,
            nonce_id: &args.nonce_id,
            media_type: &args.media_type,
            width: &args.width,
            height: &args.height,
            url: &args.url,
            thumb_url: &args.thumb_url,
            thumb_url_token: &args.thumb_url_token,
            description: &args.description,
            video_play_len: &args.video_play_len,
        })
        .await?;
    info!("send finder sns triggered");
    Ok(())
}

async fn handle_send_finder_msg(
    args: SendFinderMsgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .send_finder_msg(SendFinderMsgRequest {
            app_id: &app_id,
            to_wxid: &args.to_wxid,
            id: args.id,
            username: &args.username,
            nickname: &args.nickname,
            head_url: &args.head_url,
            nonce_id: &args.nonce_id,
            media_type: &args.media_type,
            width: &args.width,
            height: &args.height,
            url: &args.url,
            thumb_url: &args.thumb_url,
            thumb_url_token: &args.thumb_url_token,
            description: &args.description,
            video_play_len: &args.video_play_len,
        })
        .await?;
    info!("finder message sent");
    Ok(())
}

async fn handle_follow_finder(
    args: FollowFinderArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let search_info = if args.search_cookies.is_some()
        || args.search_doc_id.is_some()
        || args.search_id.is_some()
    {
        Some(FinderSearchInfo {
            cookies: args.search_cookies.as_deref(),
            search_id: args.search_id.as_deref(),
            doc_id: args.search_doc_id.as_deref(),
        })
    } else {
        None
    };
    let resp = client
        .follow_finder(FollowFinderRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            to_user_name: &args.to_user_name,
            op_type: args.op_type,
            search_info,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_follow_list(
    args: FollowListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .follow_list(FollowListRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            last_buffer: args.last_buffer.as_deref(),
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_search_follow(
    args: SearchFollowArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .search_follow(SearchFollowRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            to_user_name: &args.to_user_name,
            keyword: &args.keyword,
        })
        .await?;
    info!("search follow finished");
    Ok(())
}

async fn handle_search_finder(
    args: SearchFinderArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .search_finder(SearchFinderRequest {
            app_id: &app_id,
            content: &args.content,
            category: args.category,
            filter: args.filter,
            page: args.page,
            cookie: args.cookie.as_deref(),
            search_id: args.search_id.as_deref(),
            offset: args.offset,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_scan_follow(
    args: ScanFollowArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .scan_follow(ScanFollowRequest {
            app_id: &app_id,
            proxy_ip: &args.proxy_ip,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
            object_id: &args.object_id,
            object_nonce_id: &args.object_nonce_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_scan_browse(
    args: ScanBrowseArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .scan_browse(ScanBrowseRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
            object_id: args.object_id,
        })
        .await?;
    info!("scan browse done");
    Ok(())
}

async fn handle_scan_like(
    args: ScanLikeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .scan_like(ScanLikeRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
            object_id: args.object_id,
        })
        .await?;
    info!("scan like done");
    Ok(())
}

async fn handle_scan_fav(
    args: ScanFavArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .scan_fav(ScanFavRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
            object_id: args.object_id,
        })
        .await?;
    info!("scan fav done");
    Ok(())
}

async fn handle_scan_comment(
    args: ScanCommentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .scan_comment(ScanCommentRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
            object_id: args.object_id,
            comment_content: &args.comment_content,
            reply_username: args.reply_username.as_deref(),
            ref_comment_id: args.ref_comment_id,
            root_comment_id: args.root_comment_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_scan_qr_code(
    args: ScanQrCodeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .scan_qr_code(ScanQrCodeRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            qr_content: &args.qr_content,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_scan_login_channels(
    args: ScanLoginChannelsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .scan_login_channels(ScanLoginChannelsRequest {
            app_id: &app_id,
            qr_content: &args.qr_content,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_id_fav(args: IdFavArgs, _config_path: &Path, config: &mut CliConfig) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .id_fav(IdFavRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            op_type: args.op_type,
            object_nonce_id: &args.object_nonce_id,
            session_buffer: &args.session_buffer,
            object_id: args.object_id,
            to_user_name: &args.to_user_name,
            my_role_type: args.my_role_type,
        })
        .await?;
    info!("id fav done");
    Ok(())
}

async fn handle_id_like(
    args: IdLikeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .id_like(IdLikeRequest {
            app_id: &app_id,
            object_id: args.object_id,
            session_buffer: Some(&args.session_buffer),
            object_nonce_id: &args.object_nonce_id,
            op_type: args.op_type,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            to_user_name: &args.to_user_name,
        })
        .await?;
    info!("id like done");
    Ok(())
}

async fn handle_finder_opt(
    args: FinderOptArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .finder_opt(FinderOptRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            to_user_name: &args.to_user_name,
            op_type: args.op_type,
            id: &args.id,
            remain: args.remain,
        })
        .await?;
    info!("finder opt done");
    Ok(())
}

async fn handle_browse_finder(
    args: BrowseFinderArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .browse_finder(BrowseFinderRequest {
            app_id: &app_id,
            object_id: args.object_id,
            session_buffer: args.session_buffer.as_deref(),
            object_nonce_id: &args.object_nonce_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
        })
        .await?;
    info!("browse finder done");
    Ok(())
}

async fn handle_like_fav_list(
    args: LikeFavListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .like_fav_list(LikeFavListRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            last_buffer: args.last_buffer.as_deref(),
            flag: args.flag,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_comment_finder(
    args: CommentFinderArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .comment_finder(CommentFinderRequest {
            app_id: &app_id,
            proxy_ip: &args.proxy_ip,
            my_user_name: &args.my_user_name,
            op_type: args.op_type,
            object_nonce_id: &args.object_nonce_id,
            session_buffer: &args.session_buffer,
            object_id: args.object_id,
            my_role_type: args.my_role_type,
            content: &args.content,
            comment_id: &args.comment_id,
            reply_user_name: &args.reply_user_name,
            ref_comment_id: args.ref_comment_id,
            root_comment_id: args.root_comment_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_comment_list(
    args: CommentListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .comment_list(CommentListRequest {
            app_id: &app_id,
            object_id: args.object_id,
            last_buffer: args.last_buffer.as_deref(),
            session_buffer: &args.session_buffer,
            object_nonce_id: args.object_nonce_id.as_deref(),
            ref_comment_id: args.ref_comment_id,
            root_comment_id: args.root_comment_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_mention_list(
    args: MentionListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .mention_list(MentionListRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            req_scene: args.req_scene,
            last_buff: &args.last_buff,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_contact_list(
    args: ContactListArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .contact_list(ContactListRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
            query_info: &args.query_info,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_post_private_letter(
    args: PostPrivateLetterArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .post_private_letter(PostPrivateLetterRequest {
            app_id: &app_id,
            content: &args.content,
            to_user_name: &args.to_user_name,
            my_user_name: &args.my_user_name,
            msg_session_id: &args.msg_session_id,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_post_private_letter_img(
    args: PostPrivateLetterImgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .post_private_letter_img(PostPrivateLetterImgRequest {
            app_id: &app_id,
            to_user_name: &args.to_user_name,
            my_user_name: &args.my_user_name,
            msg_session_id: &args.msg_session_id,
            img_url: &args.img_url,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_sync_private_letter_msg(
    args: SyncPrivateLetterMsgArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .sync_private_letter_msg(SyncPrivateLetterMsgRequest {
            app_id: &app_id,
            key_buff: args.key_buff.as_deref(),
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_create_finder(
    args: CreateFinderArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp = client
        .create_finder(CreateFinderRequest {
            app_id: &app_id,
            nick_name: &args.nick_name,
            head_img: &args.head_img,
            signature: args.signature.as_deref(),
            sex: args.sex,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_update_finder_profile(
    args: UpdateFinderProfileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    client
        .update_finder_profile(UpdateFinderProfileRequest {
            app_id: &app_id,
            nick_name: args.nick_name.as_deref(),
            head_img: args.head_img.as_deref(),
            signature: args.signature.as_deref(),
            sex: args.sex,
            country: args.country.as_deref(),
            province: args.province.as_deref(),
            city: args.city.as_deref(),
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
        })
        .await?;
    info!("finder profile updated");
    Ok(())
}

async fn handle_get_finder_profile(
    args: GetFinderProfileArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp: FinderProfileInfo = client
        .get_finder_profile(GetFinderProfileRequest { app_id: &app_id })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}

async fn handle_get_finder_qr_code(
    args: GetFinderQrCodeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let resp: GetFinderQrCodeResponse = client
        .get_finder_qr_code(GetFinderQrCodeRequest {
            app_id: &app_id,
            my_user_name: &args.my_user_name,
            my_role_type: args.my_role_type,
        })
        .await?;
    println!("{}", resp.qrcode_url);
    Ok(())
}

async fn handle_user_page(
    args: UserPageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let (client, app_id) = resolve_client(&args.base, config).await?;
    let search_info = if args.search_cookies.is_some() || args.search_id.is_some() {
        Some(FinderSearchInfo {
            cookies: args.search_cookies.as_deref(),
            search_id: args.search_id.as_deref(),
            doc_id: None,
        })
    } else {
        None
    };
    let resp: UserPageResponse = client
        .user_page(UserPageRequest {
            app_id: &app_id,
            to_user_name: &args.to_user_name,
            last_buffer: args.last_buffer.as_deref(),
            max_id: args.max_id,
            search_info,
        })
        .await?;
    println!("{}", to_string_pretty(&resp)?);
    Ok(())
}
