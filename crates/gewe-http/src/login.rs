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
