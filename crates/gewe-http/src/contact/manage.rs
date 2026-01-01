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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contacts_request_serialization() {
        let req = AddContactsRequest {
            app_id: "test_app",
            scene: 3,
            option: 1,
            v3: "stranger_v3_value",
            v4: "stranger_v4_value",
            content: "Hello",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("stranger_v3_value"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_add_contacts_request_with_scene() {
        let req = AddContactsRequest {
            app_id: "test_app",
            scene: 1,
            option: 0,
            v3: "v3_data",
            v4: "v4_data",
            content: "content_data",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("v3_data"));
    }

    #[test]
    fn test_set_friend_remark_request_serialization() {
        let req = SetFriendRemarkRequest {
            app_id: "test_app",
            wxid: "friend_wxid",
            remark: "NewRemark",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid"));
        assert!(json.contains("friend_wxid"));
        assert!(json.contains("remark"));
        assert!(json.contains("NewRemark"));
    }

    #[test]
    fn test_set_friend_remark_with_unicode() {
        let req = SetFriendRemarkRequest {
            app_id: "测试应用",
            wxid: "wxid_123",
            remark: "好友备注",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("测试应用"));
        assert!(json.contains("好友备注"));
    }

    #[test]
    fn test_set_friend_permissions_request_serialization() {
        let req = SetFriendPermissionsRequest {
            app_id: "test_app",
            wxid: "friend_wxid",
            only_chat: true,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid"));
        assert!(json.contains("friend_wxid"));
        assert!(json.contains("onlyChat"));
    }

    #[test]
    fn test_upload_phone_address_list_request_serialization() {
        let req = UploadPhoneAddressListRequest {
            app_id: "test_app",
            phones: vec!["+1234567890", "+9876543210"],
            op_type: 1,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("+1234567890"));
        assert!(json.contains("+9876543210"));
    }

    #[test]
    fn test_upload_phone_address_list_empty() {
        let req = UploadPhoneAddressListRequest {
            app_id: "test_app",
            phones: vec![],
            op_type: 0,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("[]"));
    }

    #[test]
    fn test_delete_friend_request_serialization() {
        let req = DeleteFriendRequest {
            app_id: "test_app",
            wxid: "friend_to_delete",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid"));
        assert!(json.contains("friend_to_delete"));
    }

    #[test]
    fn test_set_friend_remark_with_empty_remark() {
        let req = SetFriendRemarkRequest {
            app_id: "test_app",
            wxid: "friend_wxid",
            remark: "",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("remark"));
    }

    #[test]
    fn test_set_friend_permissions_false() {
        let req = SetFriendPermissionsRequest {
            app_id: "test_app",
            wxid: "friend_wxid",
            only_chat: false,
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("wxid"));
        assert!(json.contains("false"));
    }
}
