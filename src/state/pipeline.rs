use axum::async_trait;

use openapi::apis::pipeline::*;

use openapi::models;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use crate::state::Roverd;

#[async_trait]
impl Pipeline for Roverd {
    /// Retrieve pipeline status and process execution information.
    ///
    /// PipelineGet - GET /pipeline
    async fn pipeline_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<PipelineGetResponse, String> {
        Ok(
            PipelineGetResponse::Status200_PipelineStatusAndAnArrayOfProcesses(
                models::PipelineGet200Response::new(),
            ),
        )
    }

    /// Retrieve the status of a service running as a process in the pipeline.
    ///
    /// PipelineNameGet - GET /pipeline/{name}
    async fn pipeline_name_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::PipelineNameGetPathParams,
        _query_params: models::PipelineNameGetQueryParams,
    ) -> Result<PipelineNameGetResponse, String> {
        Ok(PipelineNameGetResponse::Status200_TheStatusOfTheProcess(
            models::PipelineNameGet200Response::new(),
        ))
    }

    /// Start or stop the pipeline of all enabled services.
    ///
    /// PipelinePost - POST /pipeline
    async fn pipeline_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: models::PipelinePostQueryParams,
    ) -> Result<PipelinePostResponse, String> {
        Ok(PipelinePostResponse::Status200_ThePipelineActionWasPerformedSuccessfully)
    }
}
