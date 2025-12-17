use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginQrCodeRequest<'a> {
    /// 设备ID，首次登录可空字符串
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    /// 设备类型：ipad（推荐），mac
    pub r#type: &'a str,
    /// 地区ID
    pub region_id: &'a str,
    /// 可选代理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttuid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginQrCodeResponse {
    pub qr_data: String,
    pub qr_img_base64: String,
    pub uuid: String,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckLoginRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub uuid: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captch_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sliding: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CheckLoginResponse {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<i64>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_info: Option<LoginInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogLoginRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "regionId")]
    pub region_id: &'a str,
    #[serde(rename = "proxyIp", skip_serializing_if = "Option::is_none")]
    pub proxy_ip: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DialogLoginResponse {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginByAccountRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "proxyIp")]
    pub proxy_ip: &'a str,
    #[serde(rename = "regionId")]
    pub region_id: &'a str,
    pub account: &'a str,
    pub password: &'a str,
    pub step: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginByAccountResponse {
    #[serde(rename = "appId", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "base64Img", skip_serializing_if = "Option::is_none")]
    pub base64_img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "headImgUrl", skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "expiredTime", skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_info: Option<LoginInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCallbackRequest<'a> {
    pub token: &'a str,
    #[serde(rename = "callbackUrl")]
    pub callback_url: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMacToIpadRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckOnlineRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

pub type CheckOnlineResponse = bool;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectionRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectionResponse {
    pub uuid: String,
    #[serde(rename = "headImgUrl", skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "expiredTime", skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<i64>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_info: Option<LoginInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}
