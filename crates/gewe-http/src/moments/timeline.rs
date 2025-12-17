use crate::client::GeweHttpClient;
use gewe_core::{
    GetContactsSnsListRequest, GetContactsSnsListResponse, GetSelfSnsListRequest,
    GetSelfSnsListResponse, GetSnsDetailsRequest, GetSnsDetailsResponse, GeweError,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn get_self_sns_list(
        &self,
        req: GetSelfSnsListRequest<'_>,
    ) -> Result<GetSelfSnsListResponse, GeweError> {
        let env = self
            .post_api::<_, GetSelfSnsListResponse>("gewe/v2/api/sns/snsList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contacts_sns_list(
        &self,
        req: GetContactsSnsListRequest<'_>,
    ) -> Result<GetContactsSnsListResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactsSnsListResponse>("gewe/v2/api/sns/contactsSnsList", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_sns_details(
        &self,
        req: GetSnsDetailsRequest<'_>,
    ) -> Result<GetSnsDetailsResponse, GeweError> {
        let env = self
            .post_api::<_, GetSnsDetailsResponse>("gewe/v2/api/sns/snsDetails", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
