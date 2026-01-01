use super::common::FinderRawData;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFinderRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<&'a str>,
    #[serde(rename = "searchId", skip_serializing_if = "Option::is_none")]
    pub search_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

pub type SearchFinderResponse = FinderRawData;
