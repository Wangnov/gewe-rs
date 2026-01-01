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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_login_qr_code_response_default() {
        let resp = GetLoginQrCodeResponse::default();
        assert_eq!(resp.qr_data, "");
        assert_eq!(resp.qr_img_base64, "");
        assert_eq!(resp.uuid, "");
        assert_eq!(resp.app_id, "");
    }

    #[test]
    fn test_get_login_qr_code_response_deserialization() {
        let json = r#"{
            "qrData": "test_qr_data",
            "qrImgBase64": "base64_image",
            "uuid": "test_uuid",
            "appId": "test_app_id"
        }"#;
        let resp: GetLoginQrCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.qr_data, "test_qr_data");
        assert_eq!(resp.qr_img_base64, "base64_image");
        assert_eq!(resp.uuid, "test_uuid");
        assert_eq!(resp.app_id, "test_app_id");
    }

    #[test]
    fn test_check_login_response_default() {
        let resp = CheckLoginResponse::default();
        assert_eq!(resp.uuid, "");
        assert_eq!(resp.status, 0);
        assert_eq!(resp.head_img_url, None);
        assert_eq!(resp.nick_name, None);
    }

    #[test]
    fn test_check_login_response_with_login_info() {
        let json = r#"{
            "uuid": "test_uuid",
            "status": 1,
            "headImgUrl": "http://example.com/avatar.jpg",
            "nickName": "TestUser",
            "expiredTime": 1234567890,
            "loginInfo": {
                "uin": 123456,
                "wxid": "test_wxid",
                "nickName": "TestUser",
                "mobile": "1234567890",
                "alias": "test_alias"
            }
        }"#;
        let resp: CheckLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.uuid, "test_uuid");
        assert_eq!(resp.status, 1);
        assert_eq!(
            resp.head_img_url,
            Some("http://example.com/avatar.jpg".to_string())
        );
        assert_eq!(resp.nick_name, Some("TestUser".to_string()));
        assert_eq!(resp.expired_time, Some(1234567890));
        assert!(resp.login_info.is_some());

        let login_info = resp.login_info.unwrap();
        assert_eq!(login_info.uin, Some(123456));
        assert_eq!(login_info.wxid, Some("test_wxid".to_string()));
        assert_eq!(login_info.nick_name, Some("TestUser".to_string()));
        assert_eq!(login_info.mobile, Some("1234567890".to_string()));
        assert_eq!(login_info.alias, Some("test_alias".to_string()));
    }

    #[test]
    fn test_login_info_default() {
        let info = LoginInfo::default();
        assert_eq!(info.uin, None);
        assert_eq!(info.wxid, None);
        assert_eq!(info.nick_name, None);
        assert_eq!(info.mobile, None);
        assert_eq!(info.alias, None);
    }

    #[test]
    fn test_dialog_login_response_default() {
        let resp = DialogLoginResponse::default();
        assert_eq!(resp.app_id, "");
        assert_eq!(resp.uuid, "");
    }

    #[test]
    fn test_dialog_login_response_deserialization() {
        let json = r#"{
            "appId": "test_app",
            "uuid": "test_uuid"
        }"#;
        let resp: DialogLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.app_id, "test_app");
        assert_eq!(resp.uuid, "test_uuid");
    }

    #[test]
    fn test_login_by_account_response_default() {
        let resp = LoginByAccountResponse::default();
        assert_eq!(resp.app_id, None);
        assert_eq!(resp.base64_img, None);
        assert_eq!(resp.uuid, None);
        assert_eq!(resp.status, None);
    }

    #[test]
    fn test_login_by_account_response_deserialization() {
        let json = r#"{
            "appId": "test_app",
            "base64Img": "base64_data",
            "uuid": "test_uuid",
            "headImgUrl": "http://example.com/avatar.jpg",
            "nickName": "TestUser",
            "expiredTime": 1234567890,
            "status": 1
        }"#;
        let resp: LoginByAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.app_id, Some("test_app".to_string()));
        assert_eq!(resp.base64_img, Some("base64_data".to_string()));
        assert_eq!(resp.uuid, Some("test_uuid".to_string()));
        assert_eq!(
            resp.head_img_url,
            Some("http://example.com/avatar.jpg".to_string())
        );
        assert_eq!(resp.nick_name, Some("TestUser".to_string()));
        assert_eq!(resp.expired_time, Some(1234567890));
        assert_eq!(resp.status, Some(1));
    }

    #[test]
    fn test_reconnection_response_default() {
        let resp = ReconnectionResponse::default();
        assert_eq!(resp.uuid, "");
        assert_eq!(resp.status, 0);
        assert_eq!(resp.head_img_url, None);
    }

    #[test]
    fn test_reconnection_response_deserialization() {
        let json = r#"{
            "uuid": "test_uuid",
            "status": 1,
            "headImgUrl": "http://example.com/avatar.jpg",
            "nickName": "TestUser",
            "expiredTime": 1234567890
        }"#;
        let resp: ReconnectionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.uuid, "test_uuid");
        assert_eq!(resp.status, 1);
        assert_eq!(
            resp.head_img_url,
            Some("http://example.com/avatar.jpg".to_string())
        );
        assert_eq!(resp.nick_name, Some("TestUser".to_string()));
        assert_eq!(resp.expired_time, Some(1234567890));
    }

    #[test]
    fn test_get_login_qr_code_request_serialization() {
        let req = GetLoginQrCodeRequest {
            app_id: "test_app",
            r#type: "ipad",
            region_id: "cn",
            proxy_ip: Some("127.0.0.1"),
            ttuid: None,
            aid: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("ipad"));
        assert!(json.contains("cn"));
        assert!(json.contains("127.0.0.1"));
    }

    #[test]
    fn test_check_login_request_serialization() {
        let req = CheckLoginRequest {
            app_id: "test_app",
            uuid: "test_uuid",
            proxy_ip: None,
            captch_code: None,
            auto_sliding: Some(true),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("test_uuid"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_dialog_login_request_serialization() {
        let req = DialogLoginRequest {
            app_id: "test_app",
            region_id: "cn",
            proxy_ip: Some("127.0.0.1"),
            aid: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("cn"));
        assert!(json.contains("127.0.0.1"));
    }

    #[test]
    fn test_login_by_account_request_serialization() {
        let req = LoginByAccountRequest {
            app_id: "test_app",
            proxy_ip: "127.0.0.1",
            region_id: "cn",
            account: "test_account",
            password: "test_password",
            step: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("127.0.0.1"));
        assert!(json.contains("test_account"));
        assert!(json.contains("test_password"));
    }

    #[test]
    fn test_set_callback_request_serialization() {
        let req = SetCallbackRequest {
            token: "test_token",
            callback_url: "http://example.com/callback",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("http://example.com/callback"));
    }

    #[test]
    fn test_change_mac_to_ipad_request_serialization() {
        let req = ChangeMacToIpadRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_check_online_request_serialization() {
        let req = CheckOnlineRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_reconnection_request_serialization() {
        let req = ReconnectionRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_logout_request_serialization() {
        let req = LogoutRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }
}
