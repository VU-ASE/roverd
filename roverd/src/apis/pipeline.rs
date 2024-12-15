use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use openapi::apis::pipeline::*;
use openapi::models::*;

use tracing::{info, warn};

use crate::{state::Roverd, warn_generic, Error};

use rovervalidate::error::{PipelineValidationError, UnmetDependencyError};

use crate::apis::pipeline::PipelineValidationError::DuplicateServiceError;

#[async_trait]
impl Pipeline for Roverd {
    /// Retrieve logs for a pipeline service (this can be logs from multiple processes, if the service was restarted). These logs are still queryable if a process has been terminated or if the pipeline was stopped..
    ///
    /// LogsNameGet - GET /logs/{name}
    async fn logs_name_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: LogsNameGetPathParams,
        _query_params: LogsNameGetQueryParams,
    ) -> Result<LogsNameGetResponse, String> {
        Ok(LogsNameGetResponse::Status401_UnauthorizedAccess)
    }

    /// Retrieve pipeline status and process execution information.
    ///
    /// PipelineGet - GET /pipeline
    async fn pipeline_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<PipelineGetResponse, String> {
        let mut state = self.state.write().await;
        let enabled: Vec<PipelineGet200ResponseEnabledInner> =
            warn_generic!(state.get_pipeline().await, PipelineGetResponse);

        let status = if enabled.len() > 0 {
            PipelineStatus::Startable
        } else {
            PipelineStatus::Empty
        };

        Ok(
            PipelineGetResponse::Status200_PipelineStatusAndAnArrayOfProcesses(
                PipelineGet200Response {
                    status,
                    last_start: None,
                    last_stop: None,
                    last_restart: None,
                    enabled,
                },
            ),
        )
    }

    /// Set the services that are enabled in this pipeline, by specifying the fully qualified services.
    ///
    /// PipelinePost - POST /pipeline
    async fn pipeline_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Vec<PipelinePostRequestInner>,
    ) -> Result<PipelinePostResponse, String> {
        let state = self.state.write().await;

        let _ = match state.set_pipeline(body).await {
            Ok(a) => a,
            Err(e) => match e {
                Error::ConfigValidation(val_errors) => {
                    let mut pipeline_errors = vec![];
                    let mut string_errors = vec![];

                    for val_error in val_errors {
                        match val_error {
                            rovervalidate::error::Error::PipelineValidationError(
                                pipeline_validation_error,
                            ) => pipeline_errors.push(pipeline_validation_error),
                            e => string_errors.push(e.to_string()),
                        }
                    }

                    let mut unmet_streams = vec![];
                    let mut unmet_services = vec![];
                    let mut duplicate_service = vec![];

                    for i in pipeline_errors {
                        match i {
                                rovervalidate::error::PipelineValidationError::UnmetDependencyError(unmet_dependency_error) => {
                                    match unmet_dependency_error {
                                        rovervalidate::error::UnmetDependencyError::UnmetStream(unmet_stream_error) => {
                                            unmet_streams.push(
                                                UnmetStreamError {
                                                    source: Some(unmet_stream_error.source),
                                                    target: Some(unmet_stream_error.target),
                                                    stream: Some(unmet_stream_error.stream),
                                                }
                                            );
                                        },
                                        rovervalidate::error::UnmetDependencyError::UnmetService(unmet_service_error) => {
                                            unmet_services.push(
                                                UnmetServiceError {
                                                    source: Some(unmet_service_error.source),
                                                    target: Some(unmet_service_error.target)
                                                }
                                            )
                                        },
                                    }
                                },
                                rovervalidate::error::PipelineValidationError::DuplicateServiceError(s) => {
                                    duplicate_service.push(openapi::models::DuplicateServiceError(s));
                                },
                            }
                    }

                    let string_errors = if string_errors.len() > 0 {
                        Some(string_errors.concat().to_string())
                    } else {
                        None
                    };

                    let unmet_streams = if unmet_streams.len() > 0 {
                        Some(unmet_streams)
                    } else {
                        None
                    };

                    let unmet_services = if unmet_services.len() > 0 {
                        Some(unmet_services)
                    } else {
                        None
                    };

                    let duplicate_service = if duplicate_service.len() > 0 {
                        Some(duplicate_service)
                    } else {
                        None
                    };

                    return Ok(
                        PipelinePostResponse::Status400_ThePipelineWasNotValidAndCouldNotBeSet(
                            PipelinePost400Response {
                                message: string_errors,
                                validation_errors: PipelinePost400ResponseValidationErrors {
                                    unmet_streams,
                                    unmet_services,
                                    duplicate_service,
                                },
                            },
                        ),
                    );
                }
                all_other_errors => {
                    return Ok(
                        PipelinePostResponse::Status400_ThePipelineWasNotValidAndCouldNotBeSet(
                            PipelinePost400Response {
                                message: Some(format!("{:?}", all_other_errors)),
                                validation_errors: PipelinePost400ResponseValidationErrors {
                                    unmet_streams: None,
                                    unmet_services: None,
                                    duplicate_service: None,
                                },
                            },
                        ),
                    );
                }
            },
        };

        Ok(PipelinePostResponse::Status200_ThePipelineWasUpdatedSuccessfully)
    }

    /// Start the pipeline.
    ///
    /// PipelineStartPost - POST /pipeline/start
    async fn pipeline_start_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<PipelineStartPostResponse, String> {
        let mut state = self.state.write().await;

        let _ = match state.start().await {
            Ok(data) => data,
            Err(e) => {
                warn!("{:#?}", e);
                return Ok(PipelineStartPostResponse::Status400_AnErrorOccurred(
                    GenericError {
                        message: Some(format!("{:?}", e)),
                        code: Some(1),
                    },
                ));
            }
        };
        Ok(PipelineStartPostResponse::Status200_ThePipelineWasStartedSuccessfully)
    }

    /// Stop the pipeline.
    ///
    /// PipelineStopPost - POST /pipeline/stop
    async fn pipeline_stop_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<PipelineStopPostResponse, String> {
        info!(">> before lock");
        let mut state = self.state.write().await;

        info!(">> calling stop");
        let _ = match state.stop().await {
            Ok(data) => data,
            Err(e) => {
                warn!("{:#?}", e);
                return Ok(PipelineStopPostResponse::Status400_AnErrorOccurred(
                    GenericError {
                        message: Some(format!("{:?}", e)),
                        code: Some(1),
                    },
                ));
            }
        };
        Ok(PipelineStopPostResponse::Status200_ThePipelineWasStoppedSuccessfully)
    }
}
