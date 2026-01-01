use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLabelRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "labelName")]
    pub label_name: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LabelInfo {
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "labelId")]
    pub label_id: i64,
}

pub type AddLabelResponse = LabelInfo;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteLabelRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "labelIds")]
    pub label_ids: &'a str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLabelRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListLabelResponse {
    #[serde(rename = "labelList")]
    pub label_list: Vec<LabelInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyLabelMemberRequest<'a> {
    #[serde(rename = "appId")]
    pub app_id: &'a str,
    #[serde(rename = "labelIds")]
    pub label_ids: &'a str,
    #[serde(rename = "wxIds")]
    pub wx_ids: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_info_default() {
        let info = LabelInfo::default();
        assert_eq!(info.label_name, "");
        assert_eq!(info.label_id, 0);
    }

    #[test]
    fn test_label_info_deserialization() {
        let json = r#"{
            "labelName": "Friends",
            "labelId": 123
        }"#;
        let info: LabelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.label_name, "Friends");
        assert_eq!(info.label_id, 123);
    }

    #[test]
    fn test_list_label_response_default() {
        let resp = ListLabelResponse::default();
        assert!(resp.label_list.is_empty());
    }

    #[test]
    fn test_add_label_request_serialization() {
        let req = AddLabelRequest {
            app_id: "test_app",
            label_name: "Work",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("Work"));
    }

    #[test]
    fn test_delete_label_request_serialization() {
        let req = DeleteLabelRequest {
            app_id: "test_app",
            label_ids: "1,2,3",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("1,2,3"));
    }

    #[test]
    fn test_list_label_request_serialization() {
        let req = ListLabelRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
    }

    #[test]
    fn test_modify_label_member_request_serialization() {
        let req = ModifyLabelMemberRequest {
            app_id: "test_app",
            label_ids: "1,2",
            wx_ids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("1,2"));
        assert!(json.contains("wxid1"));
    }
}
