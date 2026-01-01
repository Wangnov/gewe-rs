use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListResponse {
    pub friends: Vec<String>,
    pub chatrooms: Vec<String>,
    pub ghs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchContactsListCacheRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

pub type FetchContactsListCacheResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchContactsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "contactsInfo")]
    pub contacts_info: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchContactsResponse {
    pub v3: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub sex: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v4: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactBriefInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactBriefInfo {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "pyInitial")]
    pub py_initial: String,
    #[serde(rename = "quanPin")]
    pub quan_pin: String,
    pub sex: i32,
    pub remark: String,
    #[serde(rename = "remarkPyInitial")]
    pub remark_py_initial: String,
    #[serde(rename = "remarkQuanPin")]
    pub remark_quan_pin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub alias: String,
    #[serde(rename = "snsBgImg", default, skip_serializing_if = "Option::is_none")]
    pub sns_bg_img: Option<String>,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "cardImgUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub card_img_url: Option<String>,
    #[serde(rename = "labelList")]
    pub label_list: String,
    pub province: String,
    pub city: String,
    #[serde(
        rename = "phoneNumList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_num_list: Option<String>,
}

pub type GetContactBriefInfoResponse = Vec<ContactBriefInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContactDetailInfoRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetailInfo {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "pyInitial", default, skip_serializing_if = "Option::is_none")]
    pub py_initial: Option<String>,
    #[serde(rename = "quanPin")]
    pub quan_pin: String,
    pub sex: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(
        rename = "remarkPyInitial",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remark_py_initial: Option<String>,
    #[serde(
        rename = "remarkQuanPin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remark_quan_pin: Option<String>,
    pub signature: String,
    pub alias: String,
    #[serde(rename = "snsBgImg")]
    pub sns_bg_img: String,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "cardImgUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub card_img_url: Option<String>,
    #[serde(rename = "labelList", default, skip_serializing_if = "Option::is_none")]
    pub label_list: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(
        rename = "phoneNumList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_num_list: Option<String>,
}

pub type GetContactDetailInfoResponse = Vec<ContactDetailInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPhoneAddressListRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<&'a str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhoneContact {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v4: Option<String>,
    #[serde(rename = "nickName", default, skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    pub sex: i32,
    #[serde(rename = "phoneMd5")]
    pub phone_md5: String,
    pub signature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub country: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    pub province: String,
    pub city: String,
    #[serde(rename = "personalCard")]
    pub personal_card: i32,
}

pub type GetPhoneAddressListResponse = Vec<PhoneContact>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckRelationRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub wxids: Vec<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContactRelationStatus {
    pub wxid: String,
    pub relation: i32,
}

pub type CheckRelationResponse = Vec<ContactRelationStatus>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contacts_list_response_default() {
        let resp = FetchContactsListResponse::default();
        assert!(resp.friends.is_empty());
        assert!(resp.chatrooms.is_empty());
        assert!(resp.ghs.is_empty());
    }

    #[test]
    fn test_fetch_contacts_list_response_deserialization() {
        let json = r#"{
            "friends": ["wxid1", "wxid2"],
            "chatrooms": ["room1", "room2"],
            "ghs": ["gh1"]
        }"#;
        let resp: FetchContactsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.friends, vec!["wxid1", "wxid2"]);
        assert_eq!(resp.chatrooms, vec!["room1", "room2"]);
        assert_eq!(resp.ghs, vec!["gh1"]);
    }

    #[test]
    fn test_search_contacts_response_default() {
        let resp = SearchContactsResponse::default();
        assert_eq!(resp.v3, "");
        assert_eq!(resp.nick_name, "");
        assert_eq!(resp.sex, 0);
        assert_eq!(resp.signature, None);
    }

    #[test]
    fn test_search_contacts_response_deserialization() {
        let json = r#"{
            "v3": "test_v3",
            "nickName": "TestUser",
            "sex": 1,
            "signature": "Hello World",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg",
            "v4": "test_v4"
        }"#;
        let resp: SearchContactsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.v3, "test_v3");
        assert_eq!(resp.nick_name, "TestUser");
        assert_eq!(resp.sex, 1);
        assert_eq!(resp.signature, Some("Hello World".to_string()));
        assert_eq!(resp.big_head_img_url, "http://example.com/big.jpg");
        assert_eq!(resp.small_head_img_url, "http://example.com/small.jpg");
        assert_eq!(resp.v4, Some("test_v4".to_string()));
    }

    #[test]
    fn test_contact_brief_info_default() {
        let info = ContactBriefInfo::default();
        assert_eq!(info.user_name, "");
        assert_eq!(info.nick_name, "");
        assert_eq!(info.sex, 0);
        assert_eq!(info.remark, "");
    }

    #[test]
    fn test_contact_brief_info_deserialization() {
        let json = r#"{
            "userName": "wxid_test",
            "nickName": "TestUser",
            "pyInitial": "TU",
            "quanPin": "testuser",
            "sex": 1,
            "remark": "My Friend",
            "remarkPyInitial": "MF",
            "remarkQuanPin": "myfriend",
            "signature": "Hello",
            "alias": "test_alias",
            "country": "CN",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg",
            "labelList": "label1,label2",
            "province": "Beijing",
            "city": "Beijing"
        }"#;
        let info: ContactBriefInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.user_name, "wxid_test");
        assert_eq!(info.nick_name, "TestUser");
        assert_eq!(info.py_initial, "TU");
        assert_eq!(info.sex, 1);
        assert_eq!(info.remark, "My Friend");
        assert_eq!(info.alias, "test_alias");
        assert_eq!(info.country, "CN");
        assert_eq!(info.province, "Beijing");
        assert_eq!(info.city, "Beijing");
    }

    #[test]
    fn test_contact_detail_info_default() {
        let info = ContactDetailInfo::default();
        assert_eq!(info.user_name, "");
        assert_eq!(info.nick_name, "");
        assert_eq!(info.sex, 0);
        assert_eq!(info.signature, "");
    }

    #[test]
    fn test_contact_detail_info_deserialization() {
        let json = r#"{
            "userName": "wxid_test",
            "nickName": "TestUser",
            "quanPin": "testuser",
            "sex": 1,
            "signature": "Hello World",
            "alias": "test_alias",
            "snsBgImg": "http://example.com/bg.jpg",
            "country": "CN",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg"
        }"#;
        let info: ContactDetailInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.user_name, "wxid_test");
        assert_eq!(info.nick_name, "TestUser");
        assert_eq!(info.quan_pin, "testuser");
        assert_eq!(info.sex, 1);
        assert_eq!(info.signature, "Hello World");
        assert_eq!(info.alias, "test_alias");
        assert_eq!(info.country, "CN");
    }

    #[test]
    fn test_phone_contact_default() {
        let contact = PhoneContact::default();
        assert_eq!(contact.user_name, "");
        assert_eq!(contact.sex, 0);
        assert_eq!(contact.phone_md5, "");
        assert_eq!(contact.signature, "");
    }

    #[test]
    fn test_phone_contact_deserialization() {
        let json = r#"{
            "userName": "wxid_test",
            "sex": 1,
            "phoneMd5": "abc123",
            "signature": "Hello",
            "country": "CN",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg",
            "province": "Beijing",
            "city": "Beijing",
            "personalCard": 1
        }"#;
        let contact: PhoneContact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.user_name, "wxid_test");
        assert_eq!(contact.sex, 1);
        assert_eq!(contact.phone_md5, "abc123");
        assert_eq!(contact.signature, "Hello");
        assert_eq!(contact.country, "CN");
        assert_eq!(contact.personal_card, 1);
    }

    #[test]
    fn test_contact_relation_status_default() {
        let status = ContactRelationStatus::default();
        assert_eq!(status.wxid, "");
        assert_eq!(status.relation, 0);
    }

    #[test]
    fn test_contact_relation_status_deserialization() {
        let json = r#"{
            "wxid": "wxid_test",
            "relation": 1
        }"#;
        let status: ContactRelationStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.wxid, "wxid_test");
        assert_eq!(status.relation, 1);
    }

    #[test]
    fn test_fetch_contacts_list_request_serialization() {
        let req = FetchContactsListRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_search_contacts_request_serialization() {
        let req = SearchContactsRequest {
            app_id: "test_app",
            contacts_info: "wxid_test",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid_test"));
    }

    #[test]
    fn test_get_contact_brief_info_request_serialization() {
        let req = GetContactBriefInfoRequest {
            app_id: "test_app",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid1"));
        assert!(json.contains("wxid2"));
    }

    #[test]
    fn test_get_phone_address_list_request_serialization() {
        let req = GetPhoneAddressListRequest {
            app_id: "test_app",
            phones: Some(vec!["12345678901", "12345678902"]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("12345678901"));
    }

    #[test]
    fn test_check_relation_request_serialization() {
        let req = CheckRelationRequest {
            app_id: "test_app",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wxid1"));
    }
}
