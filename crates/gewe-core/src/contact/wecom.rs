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
