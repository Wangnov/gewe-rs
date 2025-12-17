use crate::client::GeweHttpClient;
use gewe_core::{
    AddWecomContactRequest, GetWecomContactDetailRequest, GetWecomContactDetailResponse, GeweError,
    SearchWecomRequest, SearchWecomResponse, SyncWecomContactsRequest, SyncWecomContactsResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn search_wecom_contact(
        &self,
        req: SearchWecomRequest<'_>,
    ) -> Result<SearchWecomResponse, GeweError> {
        let env = self
            .post_api::<_, SearchWecomResponse>("gewe/v2/api/im/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn sync_wecom_contacts(
        &self,
        req: SyncWecomContactsRequest<'_>,
    ) -> Result<SyncWecomContactsResponse, GeweError> {
        let env = self
            .post_api::<_, SyncWecomContactsResponse>("gewe/v2/api/im/sync", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn add_wecom_contact(
        &self,
        req: AddWecomContactRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self.post_api::<_, ()>("gewe/v2/api/im/add", &req).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_wecom_contact_detail(
        &self,
        req: GetWecomContactDetailRequest<'_>,
    ) -> Result<GetWecomContactDetailResponse, GeweError> {
        let env = self
            .post_api::<_, GetWecomContactDetailResponse>("gewe/v2/api/im/detail", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
