use super::common::{FinderRawData, FinderSearchInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFinderRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "nickName")]
    pub nick_name: &'a str,
    #[serde(rename = "headImg")]
    pub head_img: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateFinderResponse {
    pub username: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "headUrl")]
    pub head_url: String,
    pub signature: String,
    #[serde(rename = "followFlag")]
    pub follow_flag: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFinderProfileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<&'a str>,
    #[serde(rename = "headImg", skip_serializing_if = "Option::is_none")]
    pub head_img: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFinderProfileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderAliasInfo {
    pub nickname: String,
    #[serde(rename = "headImgUrl")]
    pub head_img_url: String,
    #[serde(rename = "roleType")]
    pub role_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderPrivacySetting {
    #[serde(rename = "exportJumpLink")]
    pub export_jump_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinderProfileInfo {
    #[serde(rename = "signatureMaxLength")]
    pub signature_max_length: i32,
    #[serde(rename = "nicknameMinLength")]
    pub nickname_min_length: i32,
    #[serde(rename = "nicknameMaxLength")]
    pub nickname_max_length: i32,
    #[serde(rename = "userNoFinder")]
    pub user_no_finder: i32,
    #[serde(rename = "purchasedTotalCount")]
    pub purchased_total_count: i32,
    #[serde(rename = "privacySetting")]
    pub privacy_setting: FinderPrivacySetting,
    #[serde(rename = "aliasInfo")]
    pub alias_info: Vec<FinderAliasInfo>,
    #[serde(rename = "currentAliasRoleType")]
    pub current_alias_role_type: i32,
    #[serde(rename = "nextAliasModAvailableTime")]
    pub next_alias_mod_available_time: i64,
    #[serde(rename = "actionWording")]
    pub action_wording: serde_json::Value,
    #[serde(rename = "userFlag")]
    pub user_flag: i32,
    #[serde(rename = "mainFinderUsername")]
    pub main_finder_username: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFinderQrCodeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFinderQrCodeResponse {
    #[serde(rename = "qrcodeUrl")]
    pub qrcode_url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPageRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_info: Option<FinderSearchInfo<'a>>,
}

pub type UserPageResponse = FinderRawData;
