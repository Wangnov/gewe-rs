use crate::client::GeweHttpClient;
use gewe_core::{
    ChangeMacToIpadRequest, CheckLoginRequest, CheckLoginResponse, CheckOnlineRequest,
    CheckOnlineResponse, DialogLoginRequest, DialogLoginResponse, GetLoginQrCodeRequest,
    GetLoginQrCodeResponse, GeweError, LoginByAccountRequest, LoginByAccountResponse,
    LogoutRequest, ReconnectionRequest, ReconnectionResponse, SetCallbackRequest,
};
use serde_json::Value;
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn get_login_qr_code(
        &self,
        req: GetLoginQrCodeRequest<'_>,
    ) -> Result<GetLoginQrCodeResponse, GeweError> {
        let env = self
            .post_api::<_, GetLoginQrCodeResponse>("gewe/v2/api/login/getLoginQrCode", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn check_login(
        &self,
        req: CheckLoginRequest<'_>,
    ) -> Result<CheckLoginResponse, GeweError> {
        let env = self
            .post_api::<_, CheckLoginResponse>("gewe/v2/api/login/checkLogin", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn dialog_login(
        &self,
        req: DialogLoginRequest<'_>,
    ) -> Result<DialogLoginResponse, GeweError> {
        let env = self
            .post_api::<_, DialogLoginResponse>("gewe/v2/api/login/dialogLogin", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn login_by_account(
        &self,
        req: LoginByAccountRequest<'_>,
    ) -> Result<LoginByAccountResponse, GeweError> {
        let env = self
            .post_api::<_, LoginByAccountResponse>("gewe/v2/api/login/loginByAccount", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn set_callback(&self, req: SetCallbackRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, Value>("gewe/v2/api/login/setCallback", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn change_mac_to_ipad(
        &self,
        req: ChangeMacToIpadRequest<'_>,
    ) -> Result<GetLoginQrCodeResponse, GeweError> {
        let env = self
            .post_api::<_, GetLoginQrCodeResponse>("gewe/v2/api/login/changeMacToIpad", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn check_online(
        &self,
        req: CheckOnlineRequest<'_>,
    ) -> Result<CheckOnlineResponse, GeweError> {
        let env = self
            .post_api::<_, CheckOnlineResponse>("gewe/v2/api/login/checkOnline", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn reconnection(
        &self,
        req: ReconnectionRequest<'_>,
    ) -> Result<Option<ReconnectionResponse>, GeweError> {
        let env = self
            .post_api::<_, ReconnectionResponse>("gewe/v2/api/login/reconnection", &req)
            .await?;
        Ok(env.data)
    }

    #[instrument(skip(self))]
    pub async fn logout(&self, req: LogoutRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, Value>("gewe/v2/api/login/logout", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("ipad"));
        assert!(json.contains("cn"));
    }

    #[test]
    fn test_check_login_request_serialization() {
        let req = CheckLoginRequest {
            app_id: "test_app",
            uuid: "uuid-1234",
            proxy_ip: None,
            captch_code: None,
            auto_sliding: Some(true),
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("uuid"));
        assert!(json.contains("uuid-1234"));
    }

    #[test]
    fn test_dialog_login_request_serialization() {
        let req = DialogLoginRequest {
            app_id: "test_app",
            region_id: "cn",
            proxy_ip: Some("127.0.0.1"),
            aid: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("regionId"));
    }

    #[test]
    fn test_login_by_account_request_serialization() {
        let req = LoginByAccountRequest {
            app_id: "test_app",
            proxy_ip: "127.0.0.1",
            region_id: "cn",
            account: "test_account",
            password: "password123",
            step: 1,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_account"));
        assert!(json.contains("password123"));
        assert!(json.contains("proxyIp"));
    }

    #[test]
    fn test_set_callback_request_serialization() {
        let req = SetCallbackRequest {
            token: "test_token",
            callback_url: "https://example.com/callback",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("token"));
        assert!(json.contains("https://example.com/callback"));
    }

    #[test]
    fn test_change_mac_to_ipad_request_serialization() {
        let req = ChangeMacToIpadRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_check_online_request_serialization() {
        let req = CheckOnlineRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_reconnection_request_serialization() {
        let req = ReconnectionRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_logout_request_serialization() {
        let req = LogoutRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_get_login_qr_code_without_optional_fields() {
        let req = GetLoginQrCodeRequest {
            app_id: "test_app",
            r#type: "mac",
            region_id: "us",
            proxy_ip: None,
            ttuid: None,
            aid: None,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("mac"));
    }
}
