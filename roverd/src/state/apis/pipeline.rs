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
        // let a = rovervalidate::error::PipelineValidationError::UnmetDependencyError(
        //     UnmetDependencyError::UnmetStream(UnmetStreamError {
        //         source: "asdf".to_string(),
        //         target: "targ".to_string(),
        //         stream: "stream".to_string(),
        //     }),
        // );

        // let a = UnmetStreamError {
        //     source: "asdf".to_string(),
        //     target: "targ".to_string(),
        //     stream: "stream".to_string(),
        // };

        // let y = models::UnmetStreamError {
        //     source: Some(a.source),
        //     target: Some(a.target),
        //     stream: Some(a.stream),
        // };

        // let x = models::UnmetDependencyError();

        // let a = models::PipelineValidationError::from();

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
        Ok(PipelineNameGetResponse::Status401_UnauthorizedAccess)
    }

    /// Start or stop the pipeline of all enabled services.
    ///
    /// PipelinePost - POST /pipeline
    async fn pipeline_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _query_params: models::PipelinePostQueryParams,
    ) -> Result<PipelinePostResponse, String> {
        Ok(PipelinePostResponse::Status200_ThePipelineActionWasPerformedSuccessfully)
    }
}
