use crate::client::GeweHttpClient;
use gewe_core::{
    CheckRelationRequest, CheckRelationResponse, FetchContactsListCacheRequest,
    FetchContactsListRequest, FetchContactsListResponse, GetContactBriefInfoRequest,
    GetContactBriefInfoResponse, GetContactDetailInfoRequest, GetContactDetailInfoResponse,
    GetPhoneAddressListRequest, GetPhoneAddressListResponse, GeweError, SearchContactsRequest,
    SearchContactsResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn fetch_contacts_list(
        &self,
        req: FetchContactsListRequest<'_>,
    ) -> Result<FetchContactsListResponse, GeweError> {
        let env = self
            .post_api::<_, FetchContactsListResponse>(
                "gewe/v2/api/contacts/fetchContactsList",
                &req,
            )
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn fetch_contacts_list_cache(
        &self,
        req: FetchContactsListCacheRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/fetchContactsListCache", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn search_contacts(
        &self,
        req: SearchContactsRequest<'_>,
    ) -> Result<SearchContactsResponse, GeweError> {
        let env = self
            .post_api::<_, SearchContactsResponse>("gewe/v2/api/contacts/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contact_brief_info(
        &self,
        req: GetContactBriefInfoRequest<'_>,
    ) -> Result<GetContactBriefInfoResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactBriefInfoResponse>("gewe/v2/api/contacts/getBriefInfo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_contact_detail_info(
        &self,
        req: GetContactDetailInfoRequest<'_>,
    ) -> Result<GetContactDetailInfoResponse, GeweError> {
        let env = self
            .post_api::<_, GetContactDetailInfoResponse>("gewe/v2/api/contacts/getDetailInfo", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_phone_address_list(
        &self,
        req: GetPhoneAddressListRequest<'_>,
    ) -> Result<GetPhoneAddressListResponse, GeweError> {
        let env = self
            .post_api::<_, GetPhoneAddressListResponse>(
                "gewe/v2/api/contacts/getPhoneAddressList",
                &req,
            )
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn check_contact_relation(
        &self,
        req: CheckRelationRequest<'_>,
    ) -> Result<CheckRelationResponse, GeweError> {
        let env = self
            .post_api::<_, CheckRelationResponse>("gewe/v2/api/contacts/checkRelation", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
