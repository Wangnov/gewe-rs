use super::common::{FinderContactProfile, FinderRawData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFollowRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "proxyIp")]
    pub proxy_ip: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: &'a str,
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBrowseRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFavRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
}

pub type ScanLikeRequest<'a> = ScanFavRequest<'a>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanCommentRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(rename = "commentContent")]
    pub comment_content: &'a str,
    #[serde(rename = "replyUsername", skip_serializing_if = "Option::is_none")]
    pub reply_username: Option<&'a str>,
    #[serde(rename = "refCommentId", skip_serializing_if = "Option::is_none")]
    pub ref_comment_id: Option<i64>,
    #[serde(rename = "rootCommentId", skip_serializing_if = "Option::is_none")]
    pub root_comment_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanQrCodeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanLoginChannelsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "qrContent")]
    pub qr_content: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanLoginFinderInfo {
    #[serde(rename = "finderUsername", skip_serializing_if = "Option::is_none")]
    pub finder_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "headImgUrl", skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(rename = "coverImgUrl", skip_serializing_if = "Option::is_none")]
    pub cover_img_url: Option<String>,
    #[serde(rename = "spamFlag", skip_serializing_if = "Option::is_none")]
    pub spam_flag: Option<i32>,
    #[serde(rename = "acctType", skip_serializing_if = "Option::is_none")]
    pub acct_type: Option<i32>,
    #[serde(rename = "authIconType", skip_serializing_if = "Option::is_none")]
    pub auth_icon_type: Option<i32>,
    #[serde(rename = "ownerWxUin", skip_serializing_if = "Option::is_none")]
    pub owner_wx_uin: Option<i64>,
    #[serde(rename = "adminNickname", skip_serializing_if = "Option::is_none")]
    pub admin_nickname: Option<String>,
    #[serde(rename = "categoryFlag", skip_serializing_if = "Option::is_none")]
    pub category_flag: Option<String>,
    #[serde(rename = "uniqId", skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    #[serde(rename = "isMasterFinder", skip_serializing_if = "Option::is_none")]
    pub is_master_finder: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScanLoginChannelsResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "finderList")]
    pub finder_list: Vec<ScanLoginFinderInfo>,
    #[serde(rename = "acctStatus")]
    pub acct_status: i32,
}

pub type ScanFollowResponse = FinderContactProfile;
pub type ScanQrCodeResponse = FinderRawData;
pub type ScanCommentResponse = FinderRawData;
