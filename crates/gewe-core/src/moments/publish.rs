use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnsAudience<'a> {
    #[serde(rename = "allowWxIds", skip_serializing_if = "Option::is_none")]
    pub allow_wxids: Option<Vec<&'a str>>,
    #[serde(rename = "atWxIds", skip_serializing_if = "Option::is_none")]
    pub at_wxids: Option<Vec<&'a str>>,
    #[serde(rename = "disableWxIds", skip_serializing_if = "Option::is_none")]
    pub disable_wxids: Option<Vec<&'a str>>,
    #[serde(rename = "allowTagIds", skip_serializing_if = "Option::is_none")]
    pub allow_tag_ids: Option<Vec<&'a str>>,
    #[serde(rename = "disableTagIds", skip_serializing_if = "Option::is_none")]
    pub disable_tag_ids: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTextSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(flatten)]
    pub audience: SnsAudience<'a>,
    pub content: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnsImageInfo {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "fileMd5")]
    pub file_md5: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendImgSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(flatten)]
    pub audience: SnsAudience<'a>,
    #[serde(rename = "imgInfos")]
    pub img_infos: Vec<SnsImageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnsVideoInfo {
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: String,
    #[serde(rename = "fileMd5")]
    pub file_md5: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendVideoSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(flatten)]
    pub audience: SnsAudience<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
    #[serde(rename = "videoInfo")]
    pub video_info: SnsVideoInfo,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendUrlSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(flatten)]
    pub audience: SnsAudience<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
    #[serde(rename = "thumbUrl")]
    pub thumb_url: &'a str,
    #[serde(rename = "linkUrl")]
    pub link_url: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardSnsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(flatten)]
    pub audience: SnsAudience<'a>,
    #[serde(rename = "snsXml")]
    pub sns_xml: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendSnsResponse {
    pub id: i64,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}
