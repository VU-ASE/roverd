use tracing::info;

use axum::async_trait;

use openapi::apis::health::*;

use openapi::models;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::Roverd;

#[async_trait]
impl Health for Roverd {
    /// Retrieve the health and versioning information.
    ///
    /// StatusGet - GET /status
    async fn status_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<StatusGetResponse, String> {
        let uptime = SystemTime::now()
            .duration_since(self.info.start_time)
            .unwrap()
            .as_millis() as i64;
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        Ok(
            StatusGetResponse::Status200_TheHealthAndVersioningInformation(
                models::StatusGet200Response {
                    status: Some(self.info.status),
                    error_message: self.info.error_msg.clone(),
                    os: Some(self.info.os.clone()),
                    rover_id: self.info.rover_id,
                    rover_name: self.info.rover_name.clone(),
                    uptime: Some(uptime),
                    version: Some(self.info.version.clone()),
                    systime: Some(time_now),
                },
            ),
        )
    }

    /// Self-update the roverd daemon process.
    ///
    /// UpdatePost - POST /update
    async fn update_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<UpdatePostResponse, String> {
        
        
        Ok(UpdatePostResponse::Status400_AnErrorOccurred(models::GenericError { message: Some("todo: /update is not yet fully implemented".to_string()), code: None }))

    }
}
