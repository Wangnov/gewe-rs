use super::common::{FinderContactProfile, FinderSearchInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowFinderRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    #[serde(rename = "opType")]
    pub op_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_info: Option<FinderSearchInfo<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FollowFinderResponse {
    pub username: String,
    pub nickname: String,
    #[serde(rename = "headUrl")]
    pub head_url: String,
    pub signature: String,
    #[serde(rename = "followFlag")]
    pub follow_flag: i32,
    #[serde(default)]
    pub auth_info: serde_json::Value,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    #[serde(rename = "spamStatus")]
    pub spam_status: i32,
    #[serde(rename = "extFlag")]
    pub ext_flag: i32,
    #[serde(rename = "extInfo")]
    pub ext_info: Option<super::common::FinderExtInfo>,
    #[serde(rename = "liveStatus")]
    pub live_status: i32,
    #[serde(rename = "liveCoverImgUrl")]
    pub live_cover_img_url: String,
    #[serde(rename = "liveInfo")]
    pub live_info: Option<super::common::FinderLiveInfo>,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FollowListData {
    #[serde(rename = "contactList")]
    pub contact_list: Vec<FinderContactProfile>,
    #[serde(rename = "lastBuffer")]
    pub last_buffer: String,
    #[serde(rename = "continueFlag")]
    pub continue_flag: i32,
    #[serde(rename = "followCount")]
    pub follow_count: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFollowRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    pub keyword: &'a str,
}
