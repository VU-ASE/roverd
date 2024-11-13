// use tracing::info;

use axum::async_trait;

use openapi::apis::services::*;

use openapi::models;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Multipart};

use crate::state::Roverd;

#[async_trait]
impl Services for Roverd {
    /// Retrieve all services and their status.
    ///
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<ServicesGetResponse, String> {
        Ok(ServicesGetResponse::Status401_UnauthorizedAccess)
    }

    /// Retrieve the status and versions of a service.
    ///
    /// ServicesNameGet - GET /services/{name}
    async fn services_name_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesNameGetPathParams,
    ) -> Result<ServicesNameGetResponse, String> {
        Ok(ServicesNameGetResponse::Status401_UnauthorizedAccess)
    }

    /// Delete a specific version of a service.
    ///
    /// ServicesNameVersionDelete - DELETE /services/{name}/{version}
    async fn services_name_version_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesNameVersionDeletePathParams,
    ) -> Result<ServicesNameVersionDeleteResponse, String> {
        Ok(ServicesNameVersionDeleteResponse::Status401_UnauthorizedAccess)
    }

    /// Retrieve the status of a specific version of a service.
    ///
    /// ServicesNameVersionGet - GET /services/{name}/{version}
    async fn services_name_version_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesNameVersionGetPathParams,
    ) -> Result<ServicesNameVersionGetResponse, String> {
        Ok(ServicesNameVersionGetResponse::Status401_UnauthorizedAccess)
    }

    /// Enable, disable or build a specific version of a service in the pipeline.
    ///
    /// ServicesNameVersionPost - POST /services/{name}/{version}
    async fn services_name_version_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesNameVersionPostPathParams,
        _query_params: models::ServicesNameVersionPostQueryParams,
    ) -> Result<ServicesNameVersionPostResponse, String> {
        Ok(ServicesNameVersionPostResponse::Status401_UnauthorizedAccess)
    }

    /// Upload a new service or new version to the rover by uploading a ZIP file.
    ///
    /// ServicesPost - POST /services
    async fn services_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Multipart,
    ) -> Result<ServicesPostResponse, String> {
        Ok(ServicesPostResponse::Status401_UnauthorizedAccess)
    }
}
