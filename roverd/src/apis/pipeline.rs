use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use openapi::apis::pipeline::*;
use openapi::models;

use tracing::{info, warn};

use crate::state::Roverd;

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
        _path_params: models::LogsNameGetPathParams,
        _query_params: models::LogsNameGetQueryParams,
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
        Ok(PipelineGetResponse::Status401_UnauthorizedAccess)
    }

    /// Set the services that are enabled in this pipeline, by specifying the fully qualified services.
    ///
    /// PipelinePost - POST /pipeline
    async fn pipeline_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Vec<models::PipelinePostRequestInner>,
    ) -> Result<PipelinePostResponse, String> {
        Ok(PipelinePostResponse::Status401_UnauthorizedAccess)
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
                    models::GenericError {
                        message: Some(format!("{:?}", e)),
                        code: Some(1),
                    },
                ));
            }
        };

        info!(">> start returning");
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
                    models::GenericError {
                        message: Some(format!("{:?}", e)),
                        code: Some(1),
                    },
                ));
            }
        };
        Ok(PipelineStopPostResponse::Status200_ThePipelineWasStoppedSuccessfully)
    }
}
