use tracing::{info, warn};

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
                ))
            }
        };

        let sources: Vec<SourcesGet200ResponseInner> = config
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
    /// SourcesNameDelete - DELETE /sources/{name}
    async fn sources_name_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: SourcesNameDeletePathParams,
    ) -> Result<SourcesNameDeleteResponse, String> {
        Ok(SourcesNameDeleteResponse::Status401_UnauthorizedAccess)
    }

    /// Download and install a service from a source.
    ///
    /// SourcesNamePost - POST /sources/{name}
    async fn sources_name_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: SourcesNamePostPathParams,
    ) -> Result<SourcesNamePostResponse, String> {
        Ok(SourcesNamePostResponse::Status401_UnauthorizedAccess)
    }

    /// Add a new source.
    ///
    /// SourcesPost - POST /sources
    async fn sources_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: SourcesPostRequest,
    ) -> Result<SourcesPostResponse, String> {
        Ok(SourcesPostResponse::Status401_UnauthorizedAccess)
    }
}
