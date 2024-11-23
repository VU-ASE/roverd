use axum::async_trait;

use openapi::{apis::health::*, models::DaemonStatus};

use openapi::models::{self, StatusGet200ResponseCpuInner, StatusGet200ResponseMemory};

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use tracing::info;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::Roverd;

#[async_trait]
impl Health for Roverd {
    /// Retrieve the health and versioning information.
    ///
    /// StatusGet - GET /status
    async fn status_get(
        &self,
        method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<StatusGetResponse, String> {
        info!("{:?} /status", method);
        let uptime = SystemTime::now()
            .duration_since(self.info.start_time)
            .unwrap()
            .as_millis() as i64;
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let error_message = match self.info.status {
            DaemonStatus::Unrecoverable => Some("❌ check logs and restart roverd".to_string()),
            DaemonStatus::Recoverable => Some(match &self.info.error_msg {
                Some(msg) => format!("⚠️ {}", msg),
                None => "⚠️ recoverable error, check logs".to_string(),
            }),
            DaemonStatus::Operational => None,
        };

        Ok(
            StatusGetResponse::Status200_TheHealthAndVersioningInformation(
                models::StatusGet200Response {
                    status: self.info.status,
                    error_message,
                    os: self.info.os.clone(),
                    rover_id: self.info.rover_id,
                    rover_name: self.info.rover_name.clone(),
                    uptime,
                    version: self.info.version.clone(),
                    systime: time_now,
                    cpu: vec![StatusGet200ResponseCpuInner {
                        core: 0,
                        total: 0,
                        used: 0,
                    }],
                    memory: StatusGet200ResponseMemory { total: 0, used: 0 },
                },
            ),
        )
    }

    /// Self-update the roverd daemon process.
    ///
    /// UpdatePost - POST /update
    async fn update_post(
        &self,
        method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<UpdatePostResponse, String> {
        info!("{:?} /update", method);
        Ok(UpdatePostResponse::Status400_AnErrorOccurred(
            models::GenericError {
                message: Some("todo: /update is not yet fully implemented".to_string()),
                code: None,
            },
        ))
    }
}
