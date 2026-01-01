use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSelfSnsListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypt: Option<bool>,
    #[serde(rename = "firstPageMd5", skip_serializing_if = "Option::is_none")]
    pub first_page_md5: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactsSnsListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypt: Option<bool>,
    #[serde(rename = "firstPageMd5", skip_serializing_if = "Option::is_none")]
    pub first_page_md5: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSnsDetailsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "snsId")]
    pub sns_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnsLikeEntry {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub source: i32,
    #[serde(rename = "type")]
    pub entry_type: i32,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnsCommentEntry {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub source: i32,
    #[serde(rename = "type")]
    pub entry_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "commentId", skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i64>,
    #[serde(rename = "replyCommentId", skip_serializing_if = "Option::is_none")]
    pub reply_comment_id: Option<i64>,
    #[serde(rename = "isNotRichText", skip_serializing_if = "Option::is_none")]
    pub is_not_rich_text: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnsTimelineItem {
    pub id: i64,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "snsXml")]
    pub sns_xml: String,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "likeList", default, skip_serializing_if = "Option::is_none")]
    pub like_list: Option<Vec<SnsLikeEntry>>,
    #[serde(rename = "commentCount")]
    pub comment_count: i32,
    #[serde(
        rename = "commentList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comment_list: Option<Vec<SnsCommentEntry>>,
    #[serde(rename = "withUserCount")]
    pub with_user_count: i32,
    #[serde(
        rename = "withUserList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub with_user_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnsListResponse {
    #[serde(rename = "firstPageMd5")]
    pub first_page_md5: String,
    #[serde(rename = "maxId")]
    pub max_id: i64,
    #[serde(rename = "snsCount")]
    pub sns_count: i32,
    #[serde(rename = "requestTime")]
    pub request_time: i64,
    #[serde(rename = "snsList")]
    pub sns_list: Vec<SnsTimelineItem>,
}

pub type GetSelfSnsListResponse = SnsListResponse;
pub type GetContactsSnsListResponse = SnsListResponse;
pub type GetSnsDetailsResponse = SnsTimelineItem;
