use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesGetResponse {
    /// An array of services
    Status200_AnArrayOfServices(Vec<models::ServicesGet200ResponseInner>),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesNameGetResponse {
    /// The status of the service
    Status200_TheStatusOfTheService(models::ServicesNameGet200Response),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Entity not found
    Status404_EntityNotFound,
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesNameVersionDeleteResponse {
    /// The service version was deleted successfully
    Status200_TheServiceVersionWasDeletedSuccessfully,
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Entity not found
    Status404_EntityNotFound,
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesNameVersionGetResponse {
    /// The status of the service
    Status200_TheStatusOfTheService(models::ServicesNameVersionGet200Response),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Entity not found
    Status404_EntityNotFound,
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesNameVersionPostResponse {
    /// The service action was performed successfully
    Status200_TheServiceActionWasPerformedSuccessfully,
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Entity not found
    Status404_EntityNotFound,
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesPostResponse {
    /// The service action was performed successfully
    Status200_TheServiceActionWasPerformedSuccessfully(models::ServicesPost200Response),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

/// Services
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Services {
    /// Retrieve all parsable services and their status from disk..
    ///
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<ServicesGetResponse, String>;

    /// Retrieve the status and versions of a service.
    ///
    /// ServicesNameGet - GET /services/{name}
    async fn services_name_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesNameGetPathParams,
    ) -> Result<ServicesNameGetResponse, String>;

    /// Delete a specific version of a service.
    ///
    /// ServicesNameVersionDelete - DELETE /services/{name}/{version}
    async fn services_name_version_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesNameVersionDeletePathParams,
    ) -> Result<ServicesNameVersionDeleteResponse, String>;

    /// Retrieve the status of a specific version of a service.
    ///
    /// ServicesNameVersionGet - GET /services/{name}/{version}
    async fn services_name_version_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesNameVersionGetPathParams,
    ) -> Result<ServicesNameVersionGetResponse, String>;

    /// Enable, disable or build a specific version of a service in the pipeline.
    ///
    /// ServicesNameVersionPost - POST /services/{name}/{version}
    async fn services_name_version_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesNameVersionPostPathParams,
        query_params: models::ServicesNameVersionPostQueryParams,
    ) -> Result<ServicesNameVersionPostResponse, String>;

    /// Upload a new service or new version to the rover by uploading a ZIP file.
    ///
    /// ServicesPost - POST /services
    async fn services_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Multipart,
    ) -> Result<ServicesPostResponse, String>;
}
