use super::common::FinderRawData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinderOptRequest<'a> {
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
    pub id: &'a str,
    pub remain: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseFinderRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_buffer: Option<&'a str>,
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdFavRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "opType")]
    pub op_type: i32,
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: &'a str,
    #[serde(rename = "sessionBuffer")]
    pub session_buffer: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdLikeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_buffer: Option<&'a str>,
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: &'a str,
    #[serde(rename = "opType")]
    pub op_type: i32,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeFavListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<&'a str>,
    pub flag: i32,
}

pub type LikeFavListResponse = FinderRawData;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentFinderRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "proxyIp")]
    pub proxy_ip: &'a str,
    #[serde(rename = "myUserName")]
    pub my_user_name: &'a str,
    #[serde(rename = "opType")]
    pub op_type: i32,
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: &'a str,
    #[serde(rename = "sessionBuffer")]
    pub session_buffer: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(rename = "myRoleType")]
    pub my_role_type: i32,
    pub content: &'a str,
    #[serde(rename = "commentId")]
    pub comment_id: &'a str,
    #[serde(rename = "replyUserName")]
    pub reply_user_name: &'a str,
    #[serde(rename = "refCommentId")]
    pub ref_comment_id: i64,
    #[serde(rename = "rootCommentId")]
    pub root_comment_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentFinderResponse {
    #[serde(rename = "commentId")]
    pub comment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "objectId")]
    pub object_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<&'a str>,
    #[serde(rename = "sessionBuffer")]
    pub session_buffer: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectNonceId")]
    pub object_nonce_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refCommentId")]
    pub ref_comment_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rootCommentId")]
    pub root_comment_id: Option<i64>,
}

pub type CommentListResponse = FinderRawData;
