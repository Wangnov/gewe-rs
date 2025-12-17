use crate::client::GeweHttpClient;
use gewe_core::{
    AddContactsRequest, DeleteFriendRequest, GeweError, SetFriendPermissionsRequest,
    SetFriendRemarkRequest, UploadPhoneAddressListRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn add_contacts(&self, req: AddContactsRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/addContacts", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_friend_remark(
        &self,
        req: SetFriendRemarkRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/setFriendRemark", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn set_friend_permissions(
        &self,
        req: SetFriendPermissionsRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/setFriendPermissions", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn upload_phone_address_list(
        &self,
        req: UploadPhoneAddressListRequest<'_>,
    ) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/uploadPhoneAddressList", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete_friend(&self, req: DeleteFriendRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/contacts/deleteFriend", &req)
            .await?;
        Ok(())
    }
}
