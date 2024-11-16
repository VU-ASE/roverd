use tracing::info;

use axum::async_trait;

use openapi::apis::sources::*;

use openapi::models;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use crate::state::Roverd;

#[async_trait]
impl Sources for Roverd {
    /// Retrieves all sources in the rover.yaml
    ///
    /// SourcesGet - GET /sources
    async fn sources_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<SourcesGetResponse, String> {
        info!("get sources!!");

        Ok(SourcesGetResponse::Status401_UnauthorizedAccess)
    }

    /// Delete a source.
    ///
    /// SourcesNameDelete - DELETE /sources/{name}
    async fn sources_name_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::SourcesNameDeletePathParams,
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
        _path_params: models::SourcesNamePostPathParams,
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
        _body: models::SourcesPostRequest,
    ) -> Result<SourcesPostResponse, String> {
        Ok(SourcesPostResponse::Status401_UnauthorizedAccess)
    }
}
