use std::{
    fs,
    fs::Permissions,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Stdio,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use openapi::models::ProcessStatus;
use rovervalidate::{config::Validate, service::Service};
use tokio::{process::Command, time::sleep};
use tracing::{error, info};

use crate::util::*;
use crate::{command::ParsedCommand, constants::*};
use crate::{time_now, Error};

use super::bootspec::{Input, Stream};
use super::{
    bootspec::{BootSpec, BootSpecTuning},
    process::Process,
    service::FqBuf,
};

#[derive(Debug, Clone)]
pub struct DaemonManager {
    // When refactoring it, we will have to save the state of processes
    // pub processes: Arc<RwLock<Vec<Process>>>,
}

/// Here are two hard coded daemons, ideally we make this extensible by specifying
/// a separate "daemon pipeline" in the rover file. Don't need to expose setters
/// via the API, but getters would be nice. For now, this is hardcoded and error prone,
/// but it also will not change for the forseeable future.
impl DaemonManager {
    pub async fn init() -> Result<(), Error> {
        // First make sure the daemons are installed, this can fail which will
        // put roverd in a non operational state.
        let display_fq = {
            let fq = FqBuf::new("vu-ase", "display", "1.1.1");
            if fq.exists() {
                info!("found daemon 'display'");
                fq
            } else {
                download_and_install_service(
                    &"https://github.com/VU-ASE/display/releases/download/v1.1.1/display.zip"
                        .to_string(),
                )
                .await?
            }
        };

        let battery_fq = {
            let fq = FqBuf::new("vu-ase", "battery", "1.2.1");
            if fq.exists() {
                info!("found daemon 'battery'");
                fq
            } else {
                download_and_install_service(
                    &"https://github.com/VU-ASE/battery/releases/download/v1.2.1/battery.zip"
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
            name: BATTERY_STREAM_NAME.to_string(),
            address: format!("{}:{}", DATA_ADDRESS, BATTERY_PORT),
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

        let procs = vec![
            Process {
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
            },
            Process {
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
            },
        ];

        start_daemons(procs).await?;

        Ok(())
    }
}

#[allow(unreachable_code)]
pub async fn start_daemons(procs: Vec<Process>) -> Result<(), Error> {
    for proc in procs {
        let parsed_command = ParsedCommand::try_from(&proc.command)?;

        tokio::spawn(async move {
            loop {
                let log_file = create_log_file(&proc.log_file)?;
                let stdout = Stdio::from(log_file.try_clone()?);
                let stderr = Stdio::from(log_file);
                fs::set_permissions(
                    parsed_command.program.clone(),
                    Permissions::from_mode(0o755),
                )?;

                let mut command = Command::new(parsed_command.program.clone());
                command
                    .args(parsed_command.arguments.clone())
                    .env(ENV_KEY, proc.injected_env.clone())
                    .current_dir(proc.fq.dir())
                    .stdout(stdout)
                    .stderr(stderr);
                match command.spawn() {
                    Ok(mut child) => {
                        info!("daemon '{}' started", proc.name);
                        match child.wait().await {
                            Ok(status) => {
                                info!("daemon '{}' exited with status: {}", proc.name, status)
                            }
                            Err(e) => info!("daemon '{}' error: {}", proc.name, e),
                        }
                    }
                    Err(e) => {
                        error!("Could not start daemon '{}': {}", proc.name, e);
                    }
                }
                info!("restarting daemon '{}'", proc.name);
                sleep(Duration::from_secs(3)).await;
            }
            Ok::<(), Error>(())
        });
    }

    Ok(())
}
