use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddContactsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub scene: i32,
    pub option: i32,
    pub v3: &'a str,
    pub v4: &'a str,
    pub content: &'a str,
}

pub type AddContactsResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFriendRemarkRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
    pub remark: &'a str,
}

pub type SetFriendRemarkResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFriendPermissionsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
    #[serde(rename = "onlyChat")]
    pub only_chat: bool,
}

pub type SetFriendPermissionsResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPhoneAddressListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub phones: Vec<&'a str>,
    #[serde(rename = "opType")]
    pub op_type: i32,
}

pub type UploadPhoneAddressListResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFriendRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxid: &'a str,
}

pub type DeleteFriendResponse = ();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contacts_request_serialization() {
        let req = AddContactsRequest {
            app_id: "test_app",
            scene: 1,
            option: 2,
            v3: "test_v3",
            v4: "test_v4",
            content: "Hello, add me!",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("Hello, add me!"));
        assert!(json.contains("test_v3"));
        assert!(json.contains("test_v4"));
    }

    #[test]
    fn test_set_friend_remark_request_serialization() {
        let req = SetFriendRemarkRequest {
            app_id: "test_app",
            wxid: "wxid_test",
            remark: "My Best Friend",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid_test"));
        assert!(json.contains("My Best Friend"));
    }

    #[test]
    fn test_set_friend_permissions_request_serialization() {
        let req = SetFriendPermissionsRequest {
            app_id: "test_app",
            wxid: "wxid_test",
            only_chat: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid_test"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_upload_phone_address_list_request_serialization() {
        let req = UploadPhoneAddressListRequest {
            app_id: "test_app",
            phones: vec!["12345678901", "12345678902"],
            op_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("12345678901"));
        assert!(json.contains("12345678902"));
    }

    #[test]
    fn test_delete_friend_request_serialization() {
        let req = DeleteFriendRequest {
            app_id: "test_app",
            wxid: "wxid_test",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid_test"));
    }
}
