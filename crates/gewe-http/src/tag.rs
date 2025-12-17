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
