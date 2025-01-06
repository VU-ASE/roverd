use std::{
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use openapi::models::ProcessStatus;
use rovervalidate::{config::Validate, service::Service};
use tokio::sync::RwLock;
use tracing::info;

use crate::constants::*;
use crate::util::*;
use crate::{time_now, Error};

use super::bootspec::{BootSpecOutput, Input, Stream};
use super::{
    bootspec::{BootSpec, BootSpecTuning},
    process::{Process, SpawnedProcess},
    service::FqBuf,
};

#[derive(Debug, Clone)]
pub struct DaemonManager {
    /// Contains the "application view" of process after validation. In-between start / stop
    /// runs this vec remains unchanged.
    pub processes: Arc<RwLock<Vec<Process>>>,

    /// The "runtime" view of all processes, this contains handles to the spawned children.
    pub spawned: Arc<RwLock<Vec<SpawnedProcess>>>,
}

/// Here are two hard coded daemons, ideally we make this extensible by specifying
/// a separate "daemon pipeline" in the rover file. Don't need to expose setters
/// via the API, but getters would be nice. For now, this is hardcoded and error prone,
/// but it also will not change for the forseeable future.
impl DaemonManager {
    pub async fn new() -> Result<DaemonManager, Error> {
        // First make sure the daemons are installed, this can fail which will
        // put roverd in a non operational state.
        let display_fq = {
            let fq = FqBuf::try_from(&"/home/debix/.rover/vu-ase/display/1.0.1".to_string())?;
            if fq.exists() {
                info!("found daemon 'display'");
                fq
            } else {
                download_and_install_service(
                    &"https://github.com/VU-ASE/display/releases/download/v1.0.1/display.zip"
                        .to_string(),
                )
                .await?
            }
        };

        let battery_fq = {
            let fq = FqBuf::try_from(&"/home/debix/.rover/vu-ase/battery/1.1.0".to_string())?;
            if fq.exists() {
                info!("found daemon 'battery'");
                fq
            } else {
                download_and_install_service(
                    &"https://github.com/VU-ASE/battery/releases/download/v1.1.0/battery.zip"
                        .to_string(),
                )
                .await?
            }
        };

        let display_service_file =
            std::fs::read_to_string(display_fq.path()).map_err(|_| Error::ServiceNotFound)?;
        let battery_service_file =
            std::fs::read_to_string(battery_fq.path()).map_err(|_| Error::ServiceNotFound)?;

        let display_service: Service = serde_yaml::from_str(&display_service_file)?;
        let battery_service: Service = serde_yaml::from_str(&battery_service_file)?;

        let display_service = display_service.validate()?;
        let battery_service = battery_service.validate()?;

        let voltage_stream = Stream {
            name: "voltage".to_string(),
            address: "tcp://localhost:5699".to_string(),
        };

        let display_bootspec = BootSpec {
            name: display_service.0.name.clone(),
            version: display_service.0.version.clone(),
            inputs: vec![Input {
                service: battery_service.0.name.clone(),
                streams: vec![voltage_stream.clone()],
            }],
            outputs: vec![],
            configuration: vec![],
            tuning: BootSpecTuning {
                enabled: false,
                address: format!("{}:{}", DATA_ADDRESS, START_PORT), // transceiver address
            },
        };

        let battery_bootspec = BootSpec {
            name: battery_service.0.name.clone(),
            version: battery_service.0.version.clone(),
            inputs: vec![],
            outputs: vec![voltage_stream],
            configuration: vec![],
            tuning: BootSpecTuning {
                enabled: false,
                address: format!("{}:{}", DATA_ADDRESS, START_PORT), // transceiver address
            },
        };

        let display_injected_env = serde_json::to_string(&display_bootspec)?;
        let battery_injected_env = serde_json::to_string(&battery_bootspec)?;

        let mut procs = vec![];

        procs.push(Process {
            fq: display_fq.clone(),
            command: display_service.0.commands.run.clone(),
            last_pid: None,
            last_exit_code: 0,
            name: display_service.0.name.clone(),
            status: ProcessStatus::Stopped,
            log_file: PathBuf::from(display_fq.log_file()),
            injected_env: display_injected_env.clone(),
            faults: 0,
            start_time: time_now!() as i64,
        });

        procs.push(Process {
            fq: battery_fq.clone(),
            command: battery_service.0.commands.run.clone(),
            last_pid: None,
            last_exit_code: 0,
            name: battery_service.0.name.clone(),
            status: ProcessStatus::Stopped,
            log_file: PathBuf::from(battery_fq.log_file()),
            injected_env: battery_injected_env.clone(),
            faults: 0,
            start_time: time_now!() as i64,
        });

        Ok(DaemonManager {
            processes: Arc::new(RwLock::new(procs)),
            spawned: Arc::new(RwLock::new(vec![])),
        })
    }
}
