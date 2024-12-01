use openapi::models::{PipelineGet200Response, PipelineStatus};
use std::path::PathBuf;
use std::sync::{Arc};

use tokio::sync::Mutex;

pub mod process;
use process::{Process, ProcessManager};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Pipeline {
    response: PipelineGet200Response,
    process_manager: Arc<Mutex<ProcessManager>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            process_manager: Arc::from(Mutex::new(ProcessManager {
                processes: vec![
                    //             ("sleep", "sleep 10"),
                    // ("echo", "echo 'Hello' && sleep 5"),
                    // ("fail", "sleep 2 && exit 1"),  // This will trigger shutdown after 2 seconds
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "sleep 2".to_string(),
                        name: "sleeper".to_string(),
                        log_file: PathBuf::from("/var/log/AAA.txt"),
                        state: process::ProcessState::Stopped,
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "echo 'Hello yooyyoyoyoyoyoy'".to_string(),
                        name: "echo".to_string(),
                        log_file: PathBuf::from("/var/log/AAAB.txt"),
                        state: process::ProcessState::Stopped,
                    },
                    Process {
                        last_exit_code: None,
                        pid: 0,
                        command: "echo yooooo".to_string(),
                        name: "killer".to_string(),
                        log_file: PathBuf::from("/var/log/AAAC.txt"),
                        state: process::ProcessState::Stopped,
                    },
                ],
            })),
            response: PipelineGet200Response {
                status: PipelineStatus::Startable,
                last_start: None,
                last_stop: None,
                last_restart: None,
                enabled: vec![],
            },
        }
    }

    pub async fn start(&self) -> Result<(), Error> {


        // TODO run verification, check

    
        // TODO panics on error, avoid unwrap here, by implementing proper Error type
        self.process_manager.lock().await.start().await?;

    
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Error> {
        // TODO panics on error, avoid unwrap here, by implementing proper Error type
        self.process_manager.lock().await.stop().await?;

        Ok(())
    }
}
