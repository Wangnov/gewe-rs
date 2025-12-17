use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StrangerVisibilityRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub enabled: bool,
}

pub type StrangerVisibilityResponse = ();

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnsVisibleScopeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub option: i32,
}

pub type SetSnsVisibleScopeResponse = ();

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnsPrivacyRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "snsId")]
    pub sns_id: i64,
    pub open: bool,
}

pub type SetSnsPrivacyResponse = ();
