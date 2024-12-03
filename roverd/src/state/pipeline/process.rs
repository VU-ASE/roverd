use futures::future::join_all;

use std::path::PathBuf;
use std::process::Stdio;
use std::{fs::File, fs::OpenOptions, time::Duration};

use std::io::Write;

use tracing::{error, info, warn};

use chrono;

use std::sync::Arc;

use tokio::{
    process::{Child, Command},
    select,
    sync::{Mutex, broadcast, broadcast::Sender},
    time,
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct SpawnedProcess {
    pub name: String,
    pub child: Arc<Mutex<Child>>,
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    Started,
    Stopped,
    Killed,
}

/// A Process
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: i32,
    pub last_exit_code: Option<i32>,
    pub name: String,
    pub command: String,
    pub log_file: PathBuf,
    pub state: ProcessState,
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
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p.log_file.clone())?;

            let cur_time = chrono::Local::now().format("%H:%M:%S");
            if writeln!(file, "[{}] roverd spawned {}", cur_time, p.name).is_err() {
                warn!("Could not write log_line to file: {:?}", p.log_file)
            };

            let stdout = Stdio::from(file.try_clone()?);
            let stderr = Stdio::from(file);



            let mut command = Command::new("sh");
            command
                .arg("-c")
                .arg(p.command.clone())
                .stdout(stdout)
                .stderr(stderr);
            match command.spawn() {
                Ok(child) => {
                    p.pid = 0;
                    self.spawned.push(SpawnedProcess {
                        name: p.name.clone(),
                        child: Arc::from(Mutex::from(child)),
                    });
                }
                Err(e) => {
                    println!("Failed to spawn process {}: {}", p.name, e);
                    self.shutdown_tx.send(())?;
                    break;
                }
            }
        }

        for proc in self.spawned.clone() {
            let mut shutdown_rx = self.shutdown_tx.subscribe();
            let process_shutdown_tx = self.shutdown_tx.clone();

            tokio::spawn(async move {
                loop {
                    info!(">> start: before child lock of {:?}", proc.name);
                    let mut child = proc.child.lock().await;
                    select! {
                        // Wait for process completion
                        status = child.wait() => {
                            match status {
                                Ok(status) => {
                                    if !status.success() {
                                        process_shutdown_tx.send(()).ok();
                                    }
                                }
                                Err(e) => {
                                    error!("Error waiting for process {}: {}", proc.name, e);
                                    process_shutdown_tx.send(()).ok();
                                }
                            }
                            break;
                        }
                        // Wait for shutdown signal
                        _ = shutdown_rx.recv() => {
                            if let Some(id) = child.id() {
                                unsafe {
                                    info!("Sending terminate to {}", proc.name);
                                    libc::kill(id as i32, libc::SIGTERM);
                                }
                            }

                            // Wait for 1 second before sending KILL signal
                            time::sleep(Duration::from_secs(1)).await;


                            match child.try_wait() {
                                Ok(None) => {
                                    info!("Force killing process: {}", proc.name);
                                    if let Err(e) = child.kill().await {
                                        error!("Error killing process {:?}: {:?}", proc.name, e);
                                    }
                                },
                                Ok(Some(status)) => {
                                    info!("Successfully terminated child: {:?} with {:?}", child, status)
                                },
                                Err(e) => {
                                    error!("Error: {:?}", e);
                                }
                            }
                            break;
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
