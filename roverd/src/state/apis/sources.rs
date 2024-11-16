use tracing::warn;

use axum::async_trait;

use openapi::apis::sources::*;

use openapi::models::*;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use crate::state::Roverd;

#[async_trait]
impl Sources for Roverd {
    /// Retrieves all sources in the rover.yaml
    async fn sources_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<SourcesGetResponse, String> {
        let config = match self.config.get() {
            Ok(data) => data,
            Err(e) => {
                warn!("{:#?}", e);
                return Ok(SourcesGetResponse::Status400_AnErrorOccurred(
                    GenericError {
                        message: Some(format!("{:#?}", e)),
                        code: Some(1),
                    },
                ));
            }
        };

        let sources: Vec<SourcesGet200ResponseInner> = config
            .0
            .downloaded
            .iter()
            .map(|downloaded| SourcesGet200ResponseInner {
                name: Some(downloaded.name.clone()),
                url: Some(downloaded.source.clone()),
                version: Some(downloaded.version.clone()),
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
        _body: SourcesPostRequest,
    ) -> Result<SourcesDeleteResponse, String> {
        Ok(SourcesDeleteResponse::Status404_EntityNotFound)
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
        if let Err(e) = self.config.add_source(body).await {
            warn!("{:#?}", e);
            return Ok(SourcesPostResponse::Status400_AnErrorOccurred(
                GenericError {
                    message: Some(format!("{:#?}", e)),
                    code: Some(1),
                },
            ));
        }

        Ok(SourcesPostResponse::Status200_TheSourceWasAddedSuccessfully)
    }
}
