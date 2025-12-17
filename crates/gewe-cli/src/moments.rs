use crate::config::{default_base_url, lookup_bot, resolve_value, CliConfig};
use anyhow::{anyhow, Result};
use clap::Args;
use gewe_core::{
    CommentSnsRequest, DeleteSnsRequest, DownloadSnsVideoRequest, ForwardSnsRequest,
    GetContactsSnsListRequest, GetSelfSnsListRequest, GetSnsDetailsRequest, LikeSnsRequest,
    SendImgSnsRequest, SendTextSnsRequest, SendUrlSnsRequest, SendVideoSnsRequest,
    SetSnsPrivacyRequest, SetSnsVisibleScopeRequest, SnsAudience, SnsImageInfo, SnsVideoInfo,
    StrangerVisibilityRequest, UploadSnsImageRequest, UploadSnsVideoRequest,
};
use gewe_http::GeweHttpClient;
use std::path::Path;
use tracing::info;

#[derive(Args)]
pub struct AudienceArgs {
    #[arg(long, value_delimiter = ',')]
    pub allow_wxids: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub at_wxids: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub disable_wxids: Vec<String>,
    #[arg(long)]
    pub privacy: Option<bool>,
    #[arg(long, value_delimiter = ',')]
    pub allow_tag_ids: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    pub disable_tag_ids: Vec<String>,
}

#[derive(Args)]
pub struct SendMomentTextArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub content: String,
    #[command(flatten)]
    pub audience: AudienceArgs,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendMomentImageArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[command(flatten)]
    pub audience: AudienceArgs,
    /// 通过 upload-moment-image 获取的信息，格式：fileUrl,thumbUrl,fileMd5,width,height[,length]，可重复
    #[arg(long = "img-info")]
    pub img_infos: Vec<String>,
    #[arg(long)]
    pub content: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendMomentVideoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[command(flatten)]
    pub audience: AudienceArgs,
    #[arg(long)]
    pub content: Option<String>,
    #[arg(long)]
    pub video_file_url: String,
    #[arg(long)]
    pub video_thumb_url: String,
    #[arg(long)]
    pub video_file_md5: String,
    #[arg(long)]
    pub video_length: Option<i64>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SendMomentLinkArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[command(flatten)]
    pub audience: AudienceArgs,
    #[arg(long)]
    pub content: Option<String>,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub link_url: String,
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct ForwardMomentArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[command(flatten)]
    pub audience: AudienceArgs,
    #[arg(long)]
    pub sns_xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct UploadMomentImageArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long = "img-url", value_delimiter = ',')]
    pub img_urls: Vec<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct UploadMomentVideoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub thumb_url: String,
    #[arg(long)]
    pub video_url: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DownloadMomentVideoArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_xml: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct DeleteMomentArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_id: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetStrangerVisibilityArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub enabled: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetMomentDetailArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_id: i64,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct LikeMomentArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_id: i64,
    #[arg(long)]
    pub oper_type: i32,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct CommentMomentArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_id: i64,
    #[arg(long)]
    pub oper_type: i32,
    #[arg(long)]
    pub wxid: String,
    #[arg(long)]
    pub comment_id: Option<String>,
    #[arg(long)]
    pub content: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetContactMomentsArgs {
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
    pub max_id: Option<i64>,
    #[arg(long)]
    pub decrypt: Option<bool>,
    #[arg(long)]
    pub first_page_md5: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct GetSelfMomentsArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub max_id: Option<i64>,
    #[arg(long)]
    pub decrypt: Option<bool>,
    #[arg(long)]
    pub first_page_md5: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetMomentVisibleScopeArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub option: i32,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args)]
pub struct SetMomentPrivacyArgs {
    #[arg(long)]
    pub token: Option<String>,
    #[arg(long)]
    pub app_id: Option<String>,
    #[arg(long)]
    pub bot_app_id: Option<String>,
    #[arg(long)]
    pub bot_alias: Option<String>,
    #[arg(long)]
    pub sns_id: i64,
    #[arg(long)]
    pub open: bool,
    #[arg(long)]
    pub base_url: Option<String>,
}

pub async fn handle_send_moment_text(
    args: SendMomentTextArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendMomentTextArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        content,
        audience,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let req = SendTextSnsRequest {
        app_id: &app_id,
        audience: build_audience(&audience),
        content: &content,
    };
    let resp = client.send_text_sns(req).await?;
    info!(id = resp.id, "moment text sent");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_send_moment_image(
    args: SendMomentImageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendMomentImageArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        audience,
        img_infos,
        content,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    if img_infos.is_empty() {
        return Err(anyhow!("至少提供一个 --img-info"));
    }
    let images = parse_img_infos(&img_infos)?;
    let req = SendImgSnsRequest {
        app_id: &app_id,
        audience: build_audience(&audience),
        img_infos: images,
        content: content.as_deref(),
    };
    let resp = client.send_img_sns(req).await?;
    info!(id = resp.id, "moment image sent");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_send_moment_video(
    args: SendMomentVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendMomentVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        audience,
        content,
        video_file_url,
        video_thumb_url,
        video_file_md5,
        video_length,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let video_info = SnsVideoInfo {
        file_url: video_file_url,
        thumb_url: video_thumb_url,
        file_md5: video_file_md5,
        length: video_length,
    };
    let req = SendVideoSnsRequest {
        app_id: &app_id,
        audience: build_audience(&audience),
        content: content.as_deref(),
        video_info,
    };
    let resp = client.send_video_sns(req).await?;
    info!(id = resp.id, "moment video sent");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_send_moment_link(
    args: SendMomentLinkArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SendMomentLinkArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        audience,
        content,
        thumb_url,
        link_url,
        title,
        description,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let req = SendUrlSnsRequest {
        app_id: &app_id,
        audience: build_audience(&audience),
        content: content.as_deref(),
        thumb_url: &thumb_url,
        link_url: &link_url,
        title: &title,
        description: &description,
    };
    let resp = client.send_url_sns(req).await?;
    info!(id = resp.id, "moment link sent");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_forward_moment(
    args: ForwardMomentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let ForwardMomentArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        audience,
        sns_xml,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let req = ForwardSnsRequest {
        app_id: &app_id,
        audience: build_audience(&audience),
        sns_xml: &sns_xml,
    };
    let resp = client.forward_sns(req).await?;
    info!(id = resp.id, "moment forwarded");
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_upload_moment_image(
    args: UploadMomentImageArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let UploadMomentImageArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        img_urls,
        base_url,
    } = args;
    if img_urls.is_empty() {
        return Err(anyhow!("至少提供一个 --img-url"));
    }
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let img_refs: Vec<&str> = img_urls.iter().map(|s| s.as_str()).collect();
    let resp = client
        .upload_sns_image(UploadSnsImageRequest {
            app_id: &app_id,
            img_urls: img_refs,
        })
        .await?;
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_upload_moment_video(
    args: UploadMomentVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let UploadMomentVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        thumb_url,
        video_url,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let resp = client
        .upload_sns_video(UploadSnsVideoRequest {
            app_id: &app_id,
            thumb_url: &thumb_url,
            video_url: &video_url,
        })
        .await?;
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_download_moment_video(
    args: DownloadMomentVideoArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DownloadMomentVideoArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_xml,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let resp = client
        .download_sns_video(DownloadSnsVideoRequest {
            app_id: &app_id,
            sns_xml: &sns_xml,
        })
        .await?;
    println!("{}", resp.file_url);
    Ok(())
}

pub async fn handle_delete_moment(
    args: DeleteMomentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let DeleteMomentArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_id,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .delete_sns(DeleteSnsRequest {
            app_id: &app_id,
            sns_id,
        })
        .await?;
    info!(sns_id, "moment deleted");
    Ok(())
}

pub async fn handle_set_stranger_visibility(
    args: SetStrangerVisibilityArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetStrangerVisibilityArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        enabled,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .set_stranger_visibility(StrangerVisibilityRequest {
            app_id: &app_id,
            enabled,
        })
        .await?;
    info!(enabled, "stranger visibility updated");
    Ok(())
}

pub async fn handle_get_moment_detail(
    args: GetMomentDetailArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetMomentDetailArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_id,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let resp = client
        .get_sns_details(GetSnsDetailsRequest {
            app_id: &app_id,
            sns_id,
        })
        .await?;
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_like_moment(
    args: LikeMomentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let LikeMomentArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_id,
        oper_type,
        wxid,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .like_sns(LikeSnsRequest {
            app_id: &app_id,
            sns_id,
            oper_type,
            wxid: &wxid,
        })
        .await?;
    info!(sns_id, oper_type, "like action sent");
    Ok(())
}

pub async fn handle_comment_moment(
    args: CommentMomentArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let CommentMomentArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_id,
        oper_type,
        wxid,
        comment_id,
        content,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .comment_sns(CommentSnsRequest {
            app_id: &app_id,
            sns_id,
            oper_type,
            wxid: &wxid,
            comment_id: comment_id.as_deref(),
            content: content.as_deref(),
        })
        .await?;
    info!(sns_id, oper_type, "comment action sent");
    Ok(())
}

pub async fn handle_get_contact_moments(
    args: GetContactMomentsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetContactMomentsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        wxid,
        max_id,
        decrypt,
        first_page_md5,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let resp = client
        .get_contacts_sns_list(GetContactsSnsListRequest {
            app_id: &app_id,
            wxid: &wxid,
            max_id,
            decrypt,
            first_page_md5: first_page_md5.as_deref(),
        })
        .await?;
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_get_self_moments(
    args: GetSelfMomentsArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let GetSelfMomentsArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        max_id,
        decrypt,
        first_page_md5,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    let resp = client
        .get_self_sns_list(GetSelfSnsListRequest {
            app_id: &app_id,
            max_id,
            decrypt,
            first_page_md5: first_page_md5.as_deref(),
        })
        .await?;
    println!("{resp:#?}");
    Ok(())
}

pub async fn handle_set_moment_visible_scope(
    args: SetMomentVisibleScopeArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetMomentVisibleScopeArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        option,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .set_sns_visible_scope(SetSnsVisibleScopeRequest {
            app_id: &app_id,
            option,
        })
        .await?;
    info!(option, "moment visible scope updated");
    Ok(())
}

pub async fn handle_set_moment_privacy(
    args: SetMomentPrivacyArgs,
    _config_path: &Path,
    config: &mut CliConfig,
) -> Result<()> {
    let SetMomentPrivacyArgs {
        token,
        app_id,
        bot_app_id,
        bot_alias,
        sns_id,
        open,
        base_url,
    } = args;
    let (client, app_id) = prepare_client(token, app_id, bot_app_id, bot_alias, base_url, config)?;
    client
        .set_sns_privacy(SetSnsPrivacyRequest {
            app_id: &app_id,
            sns_id,
            open,
        })
        .await?;
    info!(sns_id, open, "moment privacy updated");
    Ok(())
}

fn prepare_client(
    token: Option<String>,
    app_id: Option<String>,
    bot_app_id: Option<String>,
    bot_alias: Option<String>,
    base_url: Option<String>,
    config: &mut CliConfig,
) -> Result<(GeweHttpClient, String)> {
    let token = resolve_value(token, config.token.clone(), "token")?;
    let base_url = base_url
        .or_else(|| config.base_url.clone())
        .unwrap_or_else(default_base_url);
    let effective_app_id = resolve_bot(bot_alias, bot_app_id.or(app_id), config)?;
    let app_id = resolve_value(effective_app_id, config.app_id.clone(), "app_id")?;
    let client = GeweHttpClient::new(token, base_url)?;
    Ok((client, app_id))
}

fn build_audience<'a>(args: &'a AudienceArgs) -> SnsAudience<'a> {
    SnsAudience {
        allow_wxids: vec_opt(&args.allow_wxids),
        at_wxids: vec_opt(&args.at_wxids),
        disable_wxids: vec_opt(&args.disable_wxids),
        allow_tag_ids: vec_opt(&args.allow_tag_ids),
        disable_tag_ids: vec_opt(&args.disable_tag_ids),
        privacy: args.privacy,
    }
}

fn vec_opt(values: &[String]) -> Option<Vec<&str>> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().map(|s| s.as_str()).collect())
    }
}

fn parse_img_infos(specs: &[String]) -> Result<Vec<SnsImageInfo>> {
    specs.iter().map(|spec| parse_img_info(spec)).collect()
}

fn parse_img_info(spec: &str) -> Result<SnsImageInfo> {
    let parts: Vec<&str> = spec.split(',').collect();
    let (file_url, thumb_url, file_md5, width_idx, height_idx, length) = match parts.len() {
        5 => (parts[0], parts[1], parts[2], 3, 4, None),
        6 => (
            parts[0],
            parts[1],
            parts[2],
            4,
            5,
            Some(parts[3].parse::<i64>()?),
        ),
        _ => {
            return Err(anyhow!(
                "img-info 需要 5 或 6 个字段：fileUrl,thumbUrl,fileMd5,width,height[,length]"
            ))
        }
    };
    let width = parts[width_idx].parse::<i64>()?;
    let height = parts[height_idx].parse::<i64>()?;
    Ok(SnsImageInfo {
        file_url: file_url.to_string(),
        thumb_url: thumb_url.to_string(),
        file_md5: file_md5.to_string(),
        length,
        width,
        height,
    })
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
