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
pub enum SourcesGetResponse {
    /// An array of sources
    Status200_AnArrayOfSources(Vec<models::SourcesGet200ResponseInner>),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SourcesNameDeleteResponse {
    /// The source was deleted successfully
    Status200_TheSourceWasDeletedSuccessfully,
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
pub enum SourcesNamePostResponse {
    /// The service was downloaded and installed successfully
    Status200_TheServiceWasDownloadedAndInstalledSuccessfully,
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
pub enum SourcesPostResponse {
    /// The source was added successfully
    Status200_TheSourceWasAddedSuccessfully,
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

/// Sources
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Sources {
    /// Retrieve all sources.
    ///
    /// SourcesGet - GET /sources
    async fn sources_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<SourcesGetResponse, String>;

    /// Delete a source.
    ///
    /// SourcesNameDelete - DELETE /sources/{name}
    async fn sources_name_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::SourcesNameDeletePathParams,
    ) -> Result<SourcesNameDeleteResponse, String>;

    /// Download and install a service from a source.
    ///
    /// SourcesNamePost - POST /sources/{name}
    async fn sources_name_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::SourcesNamePostPathParams,
    ) -> Result<SourcesNamePostResponse, String>;

    /// Add a new source.
    ///
    /// SourcesPost - POST /sources
    async fn sources_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::SourcesPostRequest,
    ) -> Result<SourcesPostResponse, String>;
}
