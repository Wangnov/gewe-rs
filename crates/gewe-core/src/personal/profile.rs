use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProfileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetProfileResponse {
    pub alias: String,
    pub wxid: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub mobile: String,
    pub uin: i64,
    pub sex: i32,
    pub province: String,
    pub city: String,
    pub signature: String,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(rename = "regCountry")]
    pub reg_country: String,
    #[serde(rename = "snsBgImg")]
    pub sns_bg_img: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<&'a str>,
}

pub type UpdateProfileResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHeadImgRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "headImgUrl")]
    pub head_img_url: &'a str,
}

pub type UpdateHeadImgResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQrCodeRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetQrCodeResponse {
    #[serde(rename = "qrCode")]
    pub qr_code: String,
}
