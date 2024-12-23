use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use std::io::Write;

use tracing::{error, info, warn};

use crate::util::*;
use chrono;
use std::sync::Arc;

use tokio::{
    process::{Child, Command},
    select,
    sync::{broadcast::Sender, Mutex},
    time,
};

use crate::command::ParsedCommand;
use crate::constants::*;
use crate::error::Error;
use crate::service::FqBuf;

#[derive(Debug, Clone)]
pub struct SpawnedProcess {
    pub name: String,
    pub child: Arc<Mutex<Child>>,
}

/// A Process
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Process {
    pub fq: FqBuf,
    pub last_pid: u32,
    pub last_exit_code: Option<i32>,
    pub name: String,
    pub command: String,
    pub log_file: PathBuf,
    pub state: openapi::models::ProcessStatus,
    pub injected_env: String,
}

#[derive(Debug, Clone)]
pub struct ProcessManager {
    /// Contains the "application view" of process after validation. In-between start / stop
    /// runs this vec remains unchanged.
    pub processes: Vec<Process>,

    /// The strictly "running" processes, can be thought of as spawned children.
    pub spawned: Vec<SpawnedProcess>,

    /// Broadcast channel to send shutdown command for termination.
    pub shutdown_tx: Sender<()>,
}

impl ProcessManager {
    pub async fn start(&mut self) -> Result<(), Error> {
        self.spawned.clear();

        for p in &mut self.processes {
            let mut log_file = create_log_file(&p.log_file)?;

            let cur_time = chrono::Local::now().format("%H:%M:%S");
            if writeln!(log_file, "[{}] roverd spawned {}", cur_time, p.name).is_err() {
                warn!("Could not write log_line to file: {:?}", p.log_file)
            };

            let stdout = Stdio::from(log_file.try_clone()?);
            let stderr = Stdio::from(log_file);

            let parsed_command = ParsedCommand::try_from(&p.command)?;

            let mut command = Command::new(parsed_command.program);
            command
                .args(parsed_command.arguments)
                .env(ENV_KEY, p.injected_env.clone())
                .stdout(stdout)
                .stderr(stderr);
            match command.spawn() {
                Ok(child) => {
                    if let Some(id) = child.id() {
                        p.last_pid = id;
                    } else {
                        warn!("Couldn't get process id from '{}'", p.name);
                        self.shutdown_tx.send(())?;
                        break;
                    }
                    self.spawned.push(SpawnedProcess {
                        name: p.name.clone(),
                        child: Arc::from(Mutex::from(child)),
                    });
                }
                Err(e) => {
                    let _ = self.shutdown_tx.send(());
                    let err_msg = format!("{}", e);
                    warn!("Failed to spawn process '{}': {}", p.name, &err_msg);
                    return Err(Error::FailedToSpawnProcess(err_msg));
                }
            }
        }

        for proc in self.spawned.clone() {
            let mut shutdown_rx = self.shutdown_tx.subscribe();
            let process_shutdown_tx = self.shutdown_tx.clone();

            tokio::spawn(async move {
                let mut child = proc.child.lock().await;
                // todo test this make sure the loop doesn't need to be here
                select! {
                    // Wait for process completion
                    result_status = child.wait() => {
                        match result_status {
                            Ok(exit_status) => {
                                info!("child {} exited with status {}", proc.name, exit_status);

                                // Todo separate process manager's data structures into concurrent
                                // data structures, then update the last_exit_code:

                                // for saved_process in self.processes.iter_mut() {
                                //     if saved_process.name == proc.name {
                                //         saved_process.last_exit_code = exit_status.code();
                                //     }
                                // }


                                process_shutdown_tx.send(()).ok();


                            }
                            Err(e) => {
                                error!("error waiting for process {}: {}", proc.name, e);
                                process_shutdown_tx.send(()).ok();
                            }
                        }
                    }
                    // Wait for shutdown signal
                    _ = shutdown_rx.recv() => {
                        if let Some(id) = child.id() {
                            unsafe {
                                info!("terminating {} pid ({})", proc.name, id);
                                libc::kill(id as i32, libc::SIGTERM);
                            }
                        }

                        // Wait for 1 second before sending KILL signal
                        time::sleep(Duration::from_secs(1)).await;

                        match child.try_wait() {
                            Ok(None) => {
                                info!("child {} did not terminate", proc.name);
                                warn!("killing {}", proc.name);
                                if let Err(e) = child.kill().await {
                                    error!("error killing process {:?}: {:?}", proc.name, e);
                                }
                            },
                            Ok(Some(_)) => {},
                            Err(e) => {
                                error!("Error: {:?}", e);
                                if let Err(e) = child.kill().await {
                                    error!("error killing process {:?}: {:?}", proc.name, e);
                                }
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.shutdown_tx.send(()).ok();
        self.spawned.clear();
        Ok(())
    }
}
