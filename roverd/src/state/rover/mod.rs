use base64::engine::Config;
use openapi::models::{PipelineGet200Response, PipelineStatus};
use std::path::PathBuf;

pub mod process;
use process::{Process, ProcessManager};

use tracing::info;

use rovervalidate::config::{Configuration, Validate};
use rovervalidate::pipeline::interface::{Pipeline, RunnablePipeline};
use rovervalidate::service::{Service, ValidatedService};

use tokio::sync::broadcast;

use crate::error::Error;

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Core {
    pub response: PipelineGet200Response,
    pub process_manager: ProcessManager,
}

impl Core {
    pub fn new() -> Self {
        Core {
            process_manager: ProcessManager {
                processes: vec![
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'proc A'; echo $ASE_SERVICE; sleep 1; done;"
                            .to_string(),
                        name: "procA".to_string(),
                        log_file: PathBuf::from("/var/log/AAAA.txt"),
                        state: process::ProcessState::Stopped,
                        injected_env: "testing something".to_string(),
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'proc B'; sleep 1; done;".to_string(),
                        name: "procB".to_string(),
                        log_file: PathBuf::from("/var/log/AAAB.txt"),
                        state: process::ProcessState::Stopped,
                        injected_env: "testing something".to_string(),
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'procC'; sleep 1; done;".to_string(),
                        name: "procC".to_string(),
                        log_file: PathBuf::from("/var/log/AAAC.txt"),
                        state: process::ProcessState::Stopped,
                        injected_env: "testing something".to_string(),
                    },
                ],
                spawned: vec![],
                shutdown_tx: broadcast::channel::<()>(1).0,
            },
            response: PipelineGet200Response {
                status: PipelineStatus::Startable,
                last_start: None,
                last_stop: None,
                last_restart: None,
                enabled: vec![],
            },
        }
    }

    fn get_config(&self) -> Result<Configuration, Error> {
        let config_file = std::fs::read_to_string(ROVER_CONFIG_FILE)?;
        let mut config: Configuration = serde_yaml::from_str(&config_file)?;

        for e in &mut config.enabled {
            if !e.ends_with("/service.yaml") {
                if e.ends_with("/") {
                    e.push_str("service.yaml");
                } else {
                    e.push_str("/service.yaml");
                }
            }
        }

        Ok(config)
    }

    fn validate(&mut self) -> Result<(), Error> {
        let config = self.get_config()?;
        info!("config: {:?}", config);

        let mut enabled_services: Vec<ValidatedService> = vec![];

        for enabled in config.enabled {
            let service_file = std::fs::read_to_string(&enabled)?;
            let service: Service = serde_yaml::from_str(&service_file)?;
            let validated = service.validate()?;
            enabled_services.push(validated);
        }

        let p = Pipeline::new(enabled_services).validate()?;

        info!("{:#?}", p);

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        // TODO run verification, check

        self.validate()?;

        // Check on disk pipeline validates
        // if not: remove it
        //

        // TODO assign ports

        // self.process_manager.start().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.process_manager.stop().await?;
        Ok(())
    }
}
