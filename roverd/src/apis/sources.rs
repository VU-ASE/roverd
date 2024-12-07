use tracing::{info, warn};

use axum::async_trait;

use openapi::apis::sources::*;

use openapi::models::*;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;


use crate::state::Roverd;
use crate::unwrap_generic;

#[async_trait]
impl Sources for Roverd {
    /// Retrieves all sources in the rover.yaml
    async fn sources_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<SourcesGetResponse, String> {
        let state = self.state.read().await;

        let config = unwrap_generic!(state.sources.get().await, SourcesGetResponse);

        // Make sure all sources are downloaded
        // let _ = unwrap_generic!(state.sources.install_missing_sources().await, SourcesGetResponse);
    

        let sources: Vec<SourcesGet200ResponseInner> = config
            .0
            .downloaded
            .iter()
            .map(|downloaded| SourcesGet200ResponseInner {
                name: downloaded.name.clone(),
                url: downloaded.source.clone(),
                version: downloaded.version.clone(),
                sha: downloaded.sha.clone(),
            })
            .collect();

        Ok(SourcesGetResponse::Status200_AnArrayOfSources(sources))
    }

    /// Delete a source.
    ///
    /// SourcesDelete - DELETE /sources
    async fn sources_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: SourcesPostRequest,
    ) -> Result<SourcesDeleteResponse, String> {
        let state = self.state.write().await;

        let _ = unwrap_generic!(state.sources.delete(body).await, SourcesDeleteResponse);

        Ok(SourcesDeleteResponse::Status200_TheSourceWasDeletedSuccessfully)
    }

    /// Add a new source.
    ///
    /// SourcesPost - POST /sources
    async fn sources_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: SourcesPostRequest,
    ) -> Result<SourcesPostResponse, String> {
        let state = self.state.write().await;
        if let Err(e) = state.sources.add(body).await {
            warn!("{:?}", e);
            return Ok(SourcesPostResponse::Status400_AnErrorOccurred(
                GenericError {
                    message: Some(format!("{:?}", e)),
                    code: Some(1),
                },
            ));
        }

        Ok(SourcesPostResponse::Status200_TheSourceWasAddedSuccessfully)
    }
}
