use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddContactsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub scene: i32,
    pub option: i32,
    pub v3: &'a str,
    pub v4: &'a str,
    pub content: &'a str,
}

pub type AddContactsResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFriendRemarkRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
    pub remark: &'a str,
}

pub type SetFriendRemarkResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFriendPermissionsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
    #[serde(rename = "onlyChat")]
    pub only_chat: bool,
}

pub type SetFriendPermissionsResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPhoneAddressListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub phones: Vec<&'a str>,
    #[serde(rename = "opType")]
    pub op_type: i32,
}

pub type UploadPhoneAddressListResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFriendRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
}

pub type DeleteFriendResponse = ();
