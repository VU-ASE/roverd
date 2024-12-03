use openapi::models::{PipelineGet200Response, PipelineStatus};
use std::path::PathBuf;

pub mod process;
use process::{Process, ProcessManager};

use tracing::info;

use tokio::sync::broadcast;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub response: PipelineGet200Response,
    pub process_manager: ProcessManager,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            process_manager: ProcessManager {
                processes: vec![
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'proc A'; sleep 1; done;".to_string(),
                        name: "procA".to_string(),
                        log_file: PathBuf::from("/var/log/AAAA.txt"),
                        state: process::ProcessState::Stopped,
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'proc B'; sleep 1; done;".to_string(),
                        name: "procB".to_string(),
                        log_file: PathBuf::from("/var/log/AAAB.txt"),
                        state: process::ProcessState::Stopped,
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "while true; do echo 'procC'; sleep 1; done;".to_string(),
                        name: "procC".to_string(),
                        log_file: PathBuf::from("/var/log/AAAC.txt"),
                        state: process::ProcessState::Stopped,
                    }
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

    pub async fn start(&mut self) -> Result<(), Error> {

        info!(">> start called");
        
        // TODO run verification, check
        
        self.process_manager.start().await?;
    
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.process_manager.stop().await?;
        Ok(())
    }
}
