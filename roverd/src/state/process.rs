use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::{SystemTime, UNIX_EPOCH};

use tracing::{error, info, warn};

use crate::{command::ParsedCommand, time_now, util::*};
use chrono;
use std::sync::Arc;

use tokio::{
    process::{Child, Command},
    select,
    sync::{broadcast::Sender, Mutex, RwLock},
};

use openapi::models::*;

use crate::constants::*;
use crate::error::Error;
use crate::service::FqBuf;

#[derive(Debug, Clone)]
pub struct SpawnedProcess {
    pub fq: FqBuf,
    pub name: String,
    pub child: Arc<Mutex<Child>>,
}

/// A Process
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Process {
    pub fq: FqBuf,
    pub last_pid: Option<u32>,
    pub last_exit_code: i32,
    pub name: String,
    pub command: String,
    pub log_file: PathBuf,
    pub status: openapi::models::ProcessStatus,
    pub injected_env: String,
    pub faults: u32,
    pub start_time: i64,
}

#[derive(Debug)]
pub struct PipelineStats {
    pub status: PipelineStatus,
    pub last_start: Option<i64>,
    pub last_stop: Option<i64>,
    pub last_restart: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ProcessManager {
    /// Contains the "application view" of process after validation. In-between start / stop
    /// runs this vec remains unchanged.
    pub processes: Arc<RwLock<Vec<Process>>>,

    /// The "runtime" view of all processes, this contains handles to the spawned children.
    pub spawned: Arc<RwLock<Vec<SpawnedProcess>>>,

    /// Overall status of the pipeline.
    pub stats: Arc<RwLock<PipelineStats>>,

    /// Broadcast channel to send shutdown command for termination.
    pub shutdown_tx: Sender<()>,
}

impl ProcessManager {
    /// The main starting procedure of all processes.
    pub async fn start(&self) -> Result<(), Error> {
        let mut stats = self.stats.write().await;

        if stats.status == PipelineStatus::Started {
            return Err(Error::PipelineAlreadyStarted);
        } else if stats.status == PipelineStatus::Empty {
            return Err(Error::PipelineIsEmpty);
        }

        let mut spawned_procs = self.spawned.write().await;
        let mut procs = self.processes.write().await;

        spawned_procs.clear();

        for p in &mut *procs {
            let mut log_file = create_log_file(&p.log_file)?;

            let cur_time = chrono::Local::now().format("%H:%M:%S");
            if writeln!(log_file, "[{}] roverd spawned {}", cur_time, p.name).is_err() {
                warn!("could not write log_line to file: {:?}", p.log_file)
            };

            let stdout = Stdio::from(log_file.try_clone()?);
            let stderr = Stdio::from(log_file);

            let parsed_command = ParsedCommand::try_from(&p.command)?;

            let mut command = Command::new(parsed_command.program);
            command
                .args(parsed_command.arguments)
                .env(ENV_KEY, p.injected_env.clone())
                .current_dir(p.fq.dir())
                .stdout(stdout)
                .stderr(stderr);
            match command.spawn() {
                Ok(child) => {
                    p.status = ProcessStatus::Running;
                    if let Some(id) = child.id() {
                        info!("spawned process: {:?} at {}", p.name, id);
                        p.last_pid = Some(id);
                    } else {
                        let err_msg = format!("process: {} exited immediately", p.name);
                        warn!(err_msg);
                        p.faults += 1;
                        p.last_exit_code = 1;
                        cancel_start(&mut stats, &mut procs, &mut spawned_procs);
                        return Err(Error::FailedToSpawnProcess(err_msg));
                    }
                    spawned_procs.push(SpawnedProcess {
                        fq: p.fq.clone(),
                        name: p.name.clone(),
                        child: Arc::from(Mutex::from(child)),
                    });
                }
                Err(e) => {
                    let err_msg = format!("{}", e);
                    warn!("failed to spawn process '{}': {}", p.name, &err_msg);
                    p.faults += 1;
                    p.last_exit_code = 1;
                    cancel_start(&mut stats, &mut procs, &mut spawned_procs);
                    return Err(Error::FailedToSpawnProcess(err_msg));
                }
            }
        }

        stats.status = PipelineStatus::Started;
        stats.last_start = Some(time_now!() as i64);

        for spawned in spawned_procs.clone() {
            let mut shutdown_rx = self.shutdown_tx.subscribe();
            let process_shutdown_tx = self.shutdown_tx.clone();

            let procs_clone = Arc::clone(&self.processes);
            let stats_clone = Arc::clone(&self.stats);

            tokio::spawn(async move {
                let mut child = spawned.child.lock().await;
                select! {
                    // Wait for process completion
                    result_status = child.wait() => {
                        match result_status {
                            Ok(exit_status) => {
                                // Update the pipeline's status.
                                let mut stats = stats_clone.write().await;
                                stats.status = PipelineStatus::Startable;
                                stats.last_restart = Some(time_now!() as i64);

                                info!("child {} exited with status {}", spawned.name, exit_status);
                                let exit_code = exit_status.code();
                                let mut procs_guard = procs_clone.write().await;

                                if let Some(proc) = procs_guard.iter_mut().find(|p| p.fq == spawned.fq) {
                                    proc.status = ProcessStatus::Stopped;
                                    if let Some(e) = exit_code {
                                        proc.last_exit_code = e;
                                    }
                                    if !exit_status.success() {
                                        proc.faults += 1
                                    }
                                }
                                process_shutdown_tx.send(()).ok();
                            }
                            Err(e) => {
                                error!("error waiting for process {}: {}", spawned.name, e);
                                process_shutdown_tx.send(()).ok();
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        // We have been sent a terminate signal, so end the process

                        // Update the pipeline's status.
                        let mut stats = stats_clone.write().await;
                        stats.status = PipelineStatus::Startable;

                        let mut procs_guard = procs_clone.write().await;
                        if let Some(proc) = procs_guard.iter_mut().find(|p| p.fq == spawned.fq) {
                            proc.status = ProcessStatus::Terminated;
                            proc.last_exit_code = 0;
                        }

                        if let Some(id) = child.id() {
                            info!("terminating {} pid ({})", spawned.name, id);
                            unsafe {
                                libc::kill(id as i32, libc::SIGTERM);
                            }
                        }

                        // Wait a short while before checking if child still exists
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

                        // If the child has not terminated, kill it.
                        match child.try_wait() {
                            Ok(None) => {
                                info!("process {} did not terminate, killing", spawned.name);
                                if let Err(e) = child.kill().await {
                                    error!("error killing process {:?}: {:?}", spawned.name, e);
                                }
                            },
                            Ok(Some(_)) => {},
                            Err(e) => {
                                error!("Error: {:?}", e);
                                if let Err(e) = child.kill().await {
                                    error!("error killing process {:?}: {:?}", spawned.name, e);
                                }
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }
}

/// If one process fails during the beginning of the starting procedure, we need to
/// kill all started children manually, set their states and clear the spawned vec.
fn cancel_start(
    stats: &mut PipelineStats,
    processes: &mut Vec<Process>,
    spawned_procs: &mut Vec<SpawnedProcess>,
) {
    stats.status = PipelineStatus::Startable;
    for p in &mut *processes {
        if let Some(pid) = p.last_pid {
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
        }
        p.status = ProcessStatus::Killed
    }

    spawned_procs.clear();
}
