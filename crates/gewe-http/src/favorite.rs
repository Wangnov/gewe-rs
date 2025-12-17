use crate::client::GeweHttpClient;
use gewe_core::{
    DeleteFavorRequest, GetFavorContentRequest, GetFavorContentResponse, GeweError,
    SyncFavorRequest, SyncFavorResponse,
};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn sync_favorites(
        &self,
        req: SyncFavorRequest<'_>,
    ) -> Result<SyncFavorResponse, GeweError> {
        let env = self
            .post_api::<_, SyncFavorResponse>("gewe/v2/api/favor/sync", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn get_favor_content(
        &self,
        req: GetFavorContentRequest<'_>,
    ) -> Result<GetFavorContentResponse, GeweError> {
        let env = self
            .post_api::<_, GetFavorContentResponse>("gewe/v2/api/favor/getContent", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }

    #[instrument(skip(self))]
    pub async fn delete_favor(&self, req: DeleteFavorRequest<'_>) -> Result<(), GeweError> {
        self.post_api::<_, serde_json::Value>("gewe/v2/api/favor/delete", &req)
            .await?;
        Ok(())
    }
}
