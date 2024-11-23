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
    /// Retrieve the list of parsable services for a specific author.
    ///
    /// ServicesAuthorGet - GET /services/{author}
    async fn services_author_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesAuthorGetPathParams,
    ) -> Result<ServicesAuthorGetResponse, String> {
        Ok(ServicesAuthorGetResponse::Status404_EntityNotFound)
    }

    /// Retrieve the list of parsable service versions for a specific author and service.
    ///
    /// ServicesAuthorServiceGet - GET /services/{author}/{service}
    async fn services_author_service_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesAuthorServiceGetPathParams,
    ) -> Result<ServicesAuthorServiceGetResponse, String> {
        Ok(ServicesAuthorServiceGetResponse::Status404_EntityNotFound)
    }

    /// Delete a specific version of a service.
    ///
    /// ServicesAuthorServiceVersionDelete - DELETE /services/{author}/{service}/{version}
    async fn services_author_service_version_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<ServicesAuthorServiceVersionDeleteResponse, String> {
        Ok(ServicesAuthorServiceVersionDeleteResponse::Status404_EntityNotFound)
    }

    /// Retrieve the status of a specific version of a service.
    ///
    /// ServicesAuthorServiceVersionGet - GET /services/{author}/{service}/{version}
    async fn services_author_service_version_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ServicesAuthorServiceVersionGetResponse, String> {
        Ok(ServicesAuthorServiceVersionGetResponse::Status404_EntityNotFound)
    }

    /// Build a fully qualified service version.
    ///
    /// ServicesAuthorServiceVersionPost - POST /services/{author}/{service}/{version}
    async fn services_author_service_version_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<ServicesAuthorServiceVersionPostResponse, String> {
        Ok(ServicesAuthorServiceVersionPostResponse::Status404_EntityNotFound)
    }

    /// Retrieve the list of all authors that have parsable services. With these authors you can query further for services.
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
