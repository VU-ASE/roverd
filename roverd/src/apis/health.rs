use axum::async_trait;

use openapi::{apis::health::*, models::DaemonStatus};

use openapi::models::{self, StatusGet200ResponseCpuInner, StatusGet200ResponseMemory};

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use tokio::process::Command;
use tracing::error;

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::Roverd;
use crate::time_now;

#[async_trait]
impl Health for Roverd {
    /// Retrieve the health and versioning information. Alias of /status
    ///
    /// RootGet - GET /
    async fn root_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<RootGetResponse, ()> {
        match self.status_get(method, host, cookies).await {
            Ok(r) => match r {
                StatusGetResponse::Status200_TheHealthAndVersioningInformation(
                    status_get200_response,
                ) => Ok(
                    RootGetResponse::Status200_TheHealthAndVersioningInformation(
                        status_get200_response,
                    ),
                ),
                StatusGetResponse::Status400_AnErrorOccurred(generic_error) => {
                    Ok(RootGetResponse::Status400_AnErrorOccurred(generic_error))
                }
            },
            Err(_) => Err(()),
        }
    }

    /// Retrieve the health and versioning information.
    ///
    /// StatusGet - GET /status
    async fn status_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<StatusGetResponse, ()> {
        let uptime = SystemTime::now()
            .duration_since(self.info.start_time)
            .unwrap()
            .as_millis() as i64;

        let time_now = time_now!() as i64;

        let error_message = match self.info.status {
            DaemonStatus::Unrecoverable => Some("❌ check logs and restart roverd".to_string()),
            DaemonStatus::Recoverable => Some(match &self.info.error_msg {
                Some(msg) => format!("⚠️ {}", msg),
                None => "⚠️ recoverable error, check logs".to_string(),
            }),
            DaemonStatus::Operational => None,
        };

        let mut sysinfo = self.state.sysinfo.write().await;

        sysinfo.refresh_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        let mut cpus = vec![];

        for (i, c) in sysinfo.cpus().iter().enumerate() {
            cpus.push(StatusGet200ResponseCpuInner {
                core: i as i32,
                total: 100,
                used: c.cpu_usage() as i32,
            });
        }

        let memory = StatusGet200ResponseMemory {
            total: (sysinfo.total_memory() / (1000_u64)) as i32,
            used: (sysinfo.used_memory() / (1000_u64)) as i32,
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
                    cpu: cpus,
                    memory,
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
    ) -> Result<UpdatePostResponse, ()> {
        let mut update_cmd = Command::new("sh");
        update_cmd
            .arg("-c")
            .arg("/home/debix/ase/bin/update-roverd");

        match update_cmd.spawn() {
            Ok(_) => (),
            Err(e) => error!("unable to spawn the update command: {}", e),
        }

        Ok(UpdatePostResponse::Status200_TheRoverdDaemonProcessInitiatedASelf)
    }

    /// Shutdown the rover..
    ///
    /// ShutdownPost - POST /shutdown
    async fn shutdown_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<ShutdownPostResponse, ()> {
        let mut shutdown = Command::new("shutdown");
        shutdown.arg("-h").arg("now");

        match shutdown.spawn() {
            Ok(_) => (),
            Err(e) => error!("unable to run shutdown command: {}", e),
        }

        Ok(ShutdownPostResponse::Status200_RoverShutdownSuccessfully)
    }
}
