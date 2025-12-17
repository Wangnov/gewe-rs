use crate::client::GeweHttpClient;
use gewe_core::{GeweError, SearchFinderRequest, SearchFinderResponse};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn search_finder(
        &self,
        req: SearchFinderRequest<'_>,
    ) -> Result<SearchFinderResponse, GeweError> {
        let env = self
            .post_api::<_, SearchFinderResponse>("gewe/v2/api/finder/search", &req)
            .await?;
        env.data.ok_or(GeweError::MissingData)
    }
}
