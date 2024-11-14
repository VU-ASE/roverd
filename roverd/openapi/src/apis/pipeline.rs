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
pub enum PipelineGetResponse {
    /// Pipeline status and an array of processes
    Status200_PipelineStatusAndAnArrayOfProcesses(models::PipelineGet200Response),
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PipelineNameGetResponse {
    /// The status of the process
    Status200_TheStatusOfTheProcess(models::PipelineNameGet200Response),
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
pub enum PipelinePostResponse {
    /// The pipeline action was performed successfully
    Status200_ThePipelineActionWasPerformedSuccessfully,
    /// An error occurred
    Status400_AnErrorOccurred(models::GenericError),
    /// Unauthorized access (you need to set the Authorization header with a valid username and password)
    Status401_UnauthorizedAccess,
}

/// Pipeline
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Pipeline {
    /// Retrieve pipeline status and process execution information.
    ///
    /// PipelineGet - GET /pipeline
    async fn pipeline_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<PipelineGetResponse, String>;

    /// Retrieve the status of a service running as a process in the pipeline.
    ///
    /// PipelineNameGet - GET /pipeline/{name}
    async fn pipeline_name_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PipelineNameGetPathParams,
        query_params: models::PipelineNameGetQueryParams,
    ) -> Result<PipelineNameGetResponse, String>;

    /// Start or stop the pipeline of all enabled services.
    ///
    /// PipelinePost - POST /pipeline
    async fn pipeline_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::PipelinePostQueryParams,
    ) -> Result<PipelinePostResponse, String>;
}
