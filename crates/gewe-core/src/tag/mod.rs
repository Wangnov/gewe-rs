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
