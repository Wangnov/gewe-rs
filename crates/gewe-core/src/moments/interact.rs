use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "snsId")]
    pub sns_id: i64,
    #[serde(rename = "operType")]
    pub oper_type: i32,
    pub wxid: &'a str,
}

pub type LikeSnsResponse = ();

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "snsId")]
    pub sns_id: i64,
    #[serde(rename = "operType")]
    pub oper_type: i32,
    pub wxid: &'a str,
    #[serde(rename = "commentId", skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
}

pub type CommentSnsResponse = ();
