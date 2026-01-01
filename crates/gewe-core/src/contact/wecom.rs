use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchWecomRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub scene: i32,
    pub content: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchWecomResponse {
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub sex: i32,
    pub signature: String,
    #[serde(rename = "bigHeadImgUrl")]
    pub big_head_img_url: String,
    #[serde(rename = "smallHeadImgUrl")]
    pub small_head_img_url: String,
    pub v3: String,
    pub v4: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncWecomContactsRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WecomContact {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub remark: String,
    #[serde(rename = "bigHeadImg")]
    pub big_head_img: String,
    #[serde(rename = "smallHeadImg")]
    pub small_head_img: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "descWordingId")]
    pub desc_wording_id: String,
}

pub type SyncWecomContactsResponse = Vec<WecomContact>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWecomContactRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    pub v3: &'a str,
    pub v4: &'a str,
}

pub type AddWecomContactResponse = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWecomContactDetailRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "toUserName")]
    pub to_user_name: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WecomContactDetail {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub remark: String,
    #[serde(rename = "bigHeadImg")]
    pub big_head_img: String,
    #[serde(rename = "smallHeadImg")]
    pub small_head_img: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "descWordingId")]
    pub desc_wording_id: String,
    pub wording: String,
    #[serde(rename = "wordingPinyin")]
    pub wording_pinyin: String,
    #[serde(rename = "wordingQuanpin")]
    pub wording_quanpin: String,
}

pub type GetWecomContactDetailResponse = WecomContactDetail;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_wecom_response_default() {
        let resp = SearchWecomResponse::default();
        assert_eq!(resp.nick_name, "");
        assert_eq!(resp.sex, 0);
        assert_eq!(resp.signature, "");
        assert_eq!(resp.v3, "");
        assert_eq!(resp.v4, "");
    }

    #[test]
    fn test_search_wecom_response_deserialization() {
        let json = r#"{
            "nickName": "WecomUser",
            "sex": 1,
            "signature": "Enterprise contact",
            "bigHeadImgUrl": "http://example.com/big.jpg",
            "smallHeadImgUrl": "http://example.com/small.jpg",
            "v3": "test_v3",
            "v4": "test_v4"
        }"#;
        let resp: SearchWecomResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.nick_name, "WecomUser");
        assert_eq!(resp.sex, 1);
        assert_eq!(resp.signature, "Enterprise contact");
        assert_eq!(resp.v3, "test_v3");
        assert_eq!(resp.v4, "test_v4");
    }

    #[test]
    fn test_wecom_contact_default() {
        let contact = WecomContact::default();
        assert_eq!(contact.user_name, "");
        assert_eq!(contact.nick_name, "");
        assert_eq!(contact.remark, "");
        assert_eq!(contact.app_id, "");
    }

    #[test]
    fn test_wecom_contact_deserialization() {
        let json = r#"{
            "userName": "wecom_user",
            "nickName": "WecomUser",
            "remark": "Enterprise contact",
            "bigHeadImg": "http://example.com/big.jpg",
            "smallHeadImg": "http://example.com/small.jpg",
            "appId": "test_app",
            "descWordingId": "wording123"
        }"#;
        let contact: WecomContact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.user_name, "wecom_user");
        assert_eq!(contact.nick_name, "WecomUser");
        assert_eq!(contact.remark, "Enterprise contact");
        assert_eq!(contact.app_id, "test_app");
        assert_eq!(contact.desc_wording_id, "wording123");
    }

    #[test]
    fn test_wecom_contact_detail_default() {
        let detail = WecomContactDetail::default();
        assert_eq!(detail.user_name, "");
        assert_eq!(detail.nick_name, "");
        assert_eq!(detail.wording, "");
    }

    #[test]
    fn test_wecom_contact_detail_deserialization() {
        let json = r#"{
            "userName": "wecom_user",
            "nickName": "WecomUser",
            "remark": "Enterprise contact",
            "bigHeadImg": "http://example.com/big.jpg",
            "smallHeadImg": "http://example.com/small.jpg",
            "appId": "test_app",
            "descWordingId": "wording123",
            "wording": "Department: Sales",
            "wordingPinyin": "bmxs",
            "wordingQuanpin": "bumenxiaoshou"
        }"#;
        let detail: WecomContactDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.user_name, "wecom_user");
        assert_eq!(detail.nick_name, "WecomUser");
        assert_eq!(detail.wording, "Department: Sales");
        assert_eq!(detail.wording_pinyin, "bmxs");
        assert_eq!(detail.wording_quanpin, "bumenxiaoshou");
    }

    #[test]
    fn test_search_wecom_request_serialization() {
        let req = SearchWecomRequest {
            app_id: "test_app",
            scene: 1,
            content: "search_term",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("search_term"));
    }

    #[test]
    fn test_sync_wecom_contacts_request_serialization() {
        let req = SyncWecomContactsRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_add_wecom_contact_request_serialization() {
        let req = AddWecomContactRequest {
            app_id: "test_app",
            v3: "test_v3",
            v4: "test_v4",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("test_v3"));
        assert!(json.contains("test_v4"));
    }

    #[test]
    fn test_get_wecom_contact_detail_request_serialization() {
        let req = GetWecomContactDetailRequest {
            app_id: "test_app",
            to_user_name: "wecom_user",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("wecom_user"));
    }
}
