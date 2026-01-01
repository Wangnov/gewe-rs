use crate::client::GeweHttpClient;
use gewe_core::{
    AddLabelRequest, AddLabelResponse, DeleteLabelRequest, GeweError, ListLabelRequest,
    ListLabelResponse, ModifyLabelMemberRequest,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn add_label(&self, req: AddLabelRequest<'_>) -> Result<AddLabelResponse, GeweError> {
        let env = self
            .post_api::<_, AddLabelResponse>("gewe/v2/api/label/add", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn delete_label(&self, req: DeleteLabelRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/label/delete", &req)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_labels(
        &self,
        req: ListLabelRequest<'_>,
    ) -> Result<ListLabelResponse, GeweError> {
        let env = self
            .post_api::<_, ListLabelResponse>("gewe/v2/api/label/list", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn modify_label_members(
        &self,
        req: ModifyLabelMemberRequest<'_>,
    ) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/label/modifyMemberList", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_label_request() {
        let req = AddLabelRequest {
            app_id: "test_app",
            label_name: "Friends",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("labelName"));
    }

    #[test]
    fn test_delete_label_request() {
        let req = DeleteLabelRequest {
            app_id: "test_app",
            label_ids: "1,2,3",
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("labelIds"));
    }

    #[test]
    fn test_list_labels_request() {
        let req = ListLabelRequest { app_id: "test_app" };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
    }

    #[test]
    fn test_modify_label_members_request() {
        let req = ModifyLabelMemberRequest {
            app_id: "test_app",
            label_ids: "1,2",
            wx_ids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("appId"));
        assert!(json.contains("labelIds"));
        assert!(json.contains("wxIds"));
    }
}
