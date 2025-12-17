use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListResponse {
    pub friends: Vec<String>,
    pub chatrooms: Vec<String>,
    pub ghs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListCacheRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

pub type FetchContactsListCacheResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchContactsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "contactsInfo")]
    pub contacts_info: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchContactsResponse {
    pub v3: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub sex: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v4: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactBriefInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactBriefInfo {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "pyInitial")]
    pub py_initial: String,
    #[serde(rename = "quanPin")]
    pub quan_pin: String,
    pub sex: i32,
    pub remark: String,
    #[serde(rename = "remarkPyInitial")]
    pub remark_py_initial: String,
    #[serde(rename = "remarkQuanPin")]
    pub remark_quan_pin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub alias: String,
    #[serde(rename = "snsBgImg", default, skip_serializing_if = "Option::is_none")]
    pub sns_bg_img: Option<String>,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "cardImgUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub card_img_url: Option<String>,
    #[serde(rename = "labelList")]
    pub label_list: String,
    pub province: String,
    pub city: String,
    #[serde(
        rename = "phoneNumList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_num_list: Option<String>,
}

pub type GetContactBriefInfoResponse = Vec<ContactBriefInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactDetailInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetailInfo {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "pyInitial", default, skip_serializing_if = "Option::is_none")]
    pub py_initial: Option<String>,
    #[serde(rename = "quanPin")]
    pub quan_pin: String,
    pub sex: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(
        rename = "remarkPyInitial",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remark_py_initial: Option<String>,
    #[serde(
        rename = "remarkQuanPin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remark_quan_pin: Option<String>,
    pub signature: String,
    pub alias: String,
    #[serde(rename = "snsBgImg")]
    pub sns_bg_img: String,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "cardImgUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub card_img_url: Option<String>,
    #[serde(rename = "labelList", default, skip_serializing_if = "Option::is_none")]
    pub label_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(
        rename = "phoneNumList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_num_list: Option<String>,
}

pub type GetContactDetailInfoResponse = Vec<ContactDetailInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPhoneAddressListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<&'a str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhoneContact {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v4: Option<String>,
    #[serde(rename = "nickName", default, skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    pub sex: i32,
    #[serde(rename = "phoneMd5")]
    pub phone_md5: String,
    pub signature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    pub province: String,
    pub city: String,
    #[serde(rename = "personalCard")]
    pub personal_card: i32,
}

pub type GetPhoneAddressListResponse = Vec<PhoneContact>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckRelationRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactRelationStatus {
    pub wxid: String,
    pub relation: i32,
}

pub type CheckRelationResponse = Vec<ContactRelationStatus>;
