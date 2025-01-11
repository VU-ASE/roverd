use anyhow::Context;
use axum_extra::extract::Multipart;
use daemons::DaemonManager;
use openapi::models::*;
use process::{PipelineStats, Process, ProcessManager};
use rovervalidate::config::{Configuration, ValidatedConfiguration};
use rovervalidate::pipeline::interface::{Pipeline, RunnablePipeline};
use rovervalidate::service::{Service, ValidatedService};
use rovervalidate::validate::Validate;
use service::{Fq, FqBuf, FqBufVec, FqVec};
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::fs::{self, remove_dir_all, remove_file};
use std::io::{BufRead, BufReader, Write};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System};
use tokio::process::Command;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, warn};

use crate::error::Error;
use crate::util::*;
use crate::{constants::*, time_now};

mod bootspec;
pub mod daemons;
pub mod process;
pub mod service;

/// Start-up information, system clock and utilization
pub mod info;

/// The main struct that implements functions called from the api and holds all objects
/// in memory necessary for operation. Info member holds static information derived mostly
/// from the
#[derive(Debug, Clone)]
pub struct Roverd {
    /// Information related to the roverd daemon, contains status.
    pub info: info::Info,

    /// Run-time data structures of the Rover, interacts with the file system
    /// and spawns processes, so must be read/write locked.
    pub state: State,
}

impl Roverd {
    pub async fn new() -> Result<Self, Error> {
        let mut info = info::Info::new();

        // todo when implementing daemons properly, add state here
        let _ = match DaemonManager::init().await {
            Ok(d) => Some(d),
            Err(e) => {
                error!("Unable to start daemons: {:?}", e);
                info.status = DaemonStatus::Unrecoverable;
                None
            }
        };

        let roverd = Self {
            info,
            state: State {
                process_manager: ProcessManager {
                    processes: Arc::new(RwLock::new(vec![])),
                    spawned: Arc::new(RwLock::new(vec![])),
                    stats: Arc::new(RwLock::new(PipelineStats {
                        status: PipelineStatus::Startable,
                        last_start: None,
                        last_stop: None,
                        last_restart: None,
                    })),
                    shutdown_tx: broadcast::channel::<()>(1).0,
                },
                built_services: Arc::new(RwLock::new(HashMap::new())),
                sysinfo: Arc::new(RwLock::new(System::new_with_specifics(
                    RefreshKind::nothing()
                        .with_processes(ProcessRefreshKind::everything())
                        .with_cpu(CpuRefreshKind::everything())
                        .with_memory(MemoryRefreshKind::everything()),
                ))),
            },
        };

        if roverd.info.status != DaemonStatus::Operational {
            warn!("did not initialize successfully {:#?}", roverd.info);
        }

        Ok(roverd)
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}

#[derive(Debug, Clone)]
pub struct State {
    // Holds all necessary data structures for starting/stopping processes
    pub process_manager: ProcessManager,

    // Look up the last built time of a service on disk.
    pub built_services: Arc<RwLock<HashMap<FqBuf, i64>>>,

    // System information initialized once
    pub sysinfo: Arc<RwLock<System>>,
}

impl State {
    pub async fn should_invalidate(&self, fq_buf: &FqBuf) -> Result<bool, Error> {
        let mut config = get_config().await?;
        let enabled_fq = FqVec::try_from(&config.enabled)?;
        let pipeline_invalidated = enabled_fq.0.contains(&Fq::from(fq_buf));

        if pipeline_invalidated {
            config.enabled.clear();
            update_config(&config)?;
        }

        Ok(pipeline_invalidated)
    }

    pub async fn fetch_service(&self, body: &FetchPostRequest) -> Result<(FqBuf, bool), Error> {
        let fq_buf = download_and_install_service(&body.url, false).await?;
        let invalidate_pipline = self.should_invalidate(&fq_buf).await?;
        Ok((fq_buf, invalidate_pipline))
    }

    pub async fn receive_upload(&self, mut body: Multipart) -> Result<(FqBuf, bool), Error> {
        if let Some(field) = body
            .next_field()
            .await
            .map_err(|_| Error::ServiceUploadBadPayload)?
        {
            // Extract bytes from payload
            let data = field
                .bytes()
                .await
                .map_err(|_| Error::ServiceUploadBadPayload)?;

            // Ignore errors, since filesystem can be in any state and
            // get a clean slate of the zip file
            let _ = remove_file(ZIP_FILE);
            let _ = remove_dir_all(UNZIPPED_DIR);

            // Create the zip file handle
            let mut file = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(ZIP_FILE)
                .with_context(|| format!("failed to create file {}", ZIP_FILE))?;

            file.write_all(&data)
                .with_context(|| format!("failed to write data to {}", ZIP_FILE))?;

            let fq_buf = extract_fq_from_zip().await?;

            // syncing can overwrite the current contents
            // if service_exists(&Fq::from(&fq_buf))? {
            //     return Err(Error::ServiceAlreadyExists);
            // }

            install_service(&fq_buf).await?;

            let invalidate_pipline = self.should_invalidate(&fq_buf).await?;

            return Ok((fq_buf, invalidate_pipline));
        }
        Err(Error::ServiceUploadBadPayload)
    }

    pub async fn get_authors(&self) -> Result<Vec<String>, Error> {
        list_dir_contents("")
    }

    pub async fn get_services(
        &self,
        path_params: ServicesAuthorGetPathParams,
    ) -> Result<Vec<String>, Error> {
        list_dir_contents(&path_params.author.to_string())
    }

    pub async fn get_versions(
        &self,
        path_params: ServicesAuthorServiceGetPathParams,
    ) -> Result<Vec<String>, Error> {
        list_dir_contents(format!("{}/{}", path_params.author, path_params.service).as_str())
    }

    pub async fn get_service(&self, fq: FqBuf) -> Result<ValidatedService, Error> {
        let contents = fs::read_to_string(fq.path()).map_err(|_| Error::ServiceNotFound)?;
        let service =
            serde_yaml::from_str::<rovervalidate::service::Service>(&contents)?.validate()?;

        Ok(service)
    }

    pub async fn delete_service(
        &self,
        path_params: &ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<bool, Error> {
        let delete_fq = FqBuf::from(path_params);

        // Get the current configuration from disk
        let mut config = get_config().await?;

        let mut built_services = self.built_services.write().await;

        let mut should_reset = false;
        // Return whether or not the service was enabled and if it was,
        // reset the pipeline
        let enabled_fq_vec = FqBufVec::try_from(&config.enabled)?.0;

        if enabled_fq_vec.contains(&delete_fq) {
            should_reset = true;
            config.enabled.clear();
            update_config(&config)?;

            built_services.remove(&delete_fq);
        }

        // Remove the service to delete from the filesystem
        if Path::new(&delete_fq.dir()).exists() {
            std::fs::remove_dir_all(delete_fq.dir())
                .with_context(|| format!("failed to remove {}", delete_fq.dir()))?;
        } else {
            return Err(Error::ServiceNotFound);
        }

        Ok(should_reset)
    }

    pub async fn build_service(
        &self,
        params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        let fq = FqBuf::from(&params);
        let service = self.get_service(fq.clone()).await?.0;

        let build_string = &service
            .commands
            .build
            .ok_or_else(|| Error::BuildCommandMissing)?;
        let log_file = create_log_file(&PathBuf::from(fq.build_log_file()))?;
        let stdout = Stdio::from(
            log_file
                .try_clone()
                .with_context(|| format!("failed to clone build-log file {:?}", log_file))?,
        );
        let stderr = Stdio::from(log_file);

        let mut built_services = self.built_services.write().await;

        match Command::new("sh")
            .args(["-c", build_string.as_str()])
            .stdout(stdout)
            .stderr(stderr)
            .current_dir(fq.dir())
            .spawn()
        {
            Ok(mut child) => match child.wait().await {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        // Build was not successful, return logs
                        let file = std::fs::File::open(fq.build_log_file())
                            .with_context(|| format!("failed to open {}", fq.build_log_file()))?;
                        let reader = BufReader::new(file);
                        let lines: Vec<String> = reader
                            .lines()
                            .collect::<Result<Vec<String>, _>>()
                            .with_context(|| {
                                format!("failed to collect lines from {}", fq.build_log_file())
                            })?;
                        return Err(Error::BuildLog(lines));
                    } else {
                        // Build was successful, save time it was built at
                        let time_now = time_now!() as i64;

                        *built_services.entry(fq).or_insert_with(|| time_now) = time_now;
                    }
                    Ok(())
                }
                Err(e) => {
                    error!("failed to wait on build command: {:?} {}", &build_string, e);
                    Err(Error::BuildCommandFailed)
                }
            },
            Err(e) => {
                error!("failed to spawn build command: {:?} {}", &build_string, e);
                Err(Error::BuildCommandFailed)
            }
        }
    }

    pub async fn set_pipeline(
        &self,
        incoming_pipeline: Vec<PipelinePostRequestInner>,
    ) -> Result<(), Error> {
        let mut stats = self.process_manager.stats.write().await;

        if incoming_pipeline.is_empty() {
            stats.status = PipelineStatus::Empty;
            return Err(Error::PipelineIsEmpty);
        }

        let services = FqBufVec::from(incoming_pipeline).0;

        let mut valid_services = vec![];

        for enabled in &services {
            let service_file =
                std::fs::read_to_string(enabled.path()).map_err(|_| Error::ServiceNotFound)?;
            let service: Service = serde_yaml::from_str(&service_file)?;
            valid_services.push(service.validate()?);
        }

        let _ = Pipeline::new(valid_services).validate()?;

        // Here we have a valid pipeline, so rover.yaml can be overwritten
        let mut config = get_config().await?;
        config.enabled.clear();

        // Services are valid since we didn't return earlier
        for service in services {
            config.enabled.push(service.path())
        }

        update_config(&config)?;

        stats.status = PipelineStatus::Startable;

        Ok(())
    }

    pub async fn get_pipeline(&self) -> Result<Vec<PipelineGet200ResponseEnabledInner>, Error> {
        let conf = get_config().await?;

        let processes = self.process_manager.processes.read().await;

        let mut responses = vec![];

        for validated_service in conf.enabled.into_iter() {
            let fq = FqBuf::try_from(validated_service)?;

            let mut sysinfo = self.sysinfo.write().await;

            sysinfo.refresh_specifics(
                RefreshKind::nothing()
                    .with_processes(ProcessRefreshKind::everything())
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );

            let proc = get_proc(fq.clone(), &processes);
            responses.push(match proc {
                Ok(p) => {
                    if let Some(pid) = p.last_pid {
                        let status = p.status;
                        let pid = pid as i32;
                        let mut memory = 0;
                        let mut cpu = 0;
                        let uptime = (time_now!() as i64) - p.start_time;
                        // todo: test this with a substantial payload on the debix
                        if let Some(proc_info) = sysinfo.process(Pid::from(pid as usize)) {
                            memory = (proc_info.memory() / 1000000_u64) as i32;
                            cpu = (proc_info.cpu_usage() * 100.0) as i32;
                        }
                        PipelineGet200ResponseEnabledInner {
                            process: Some(PipelineGet200ResponseEnabledInnerProcess {
                                cpu,
                                pid,
                                uptime,
                                memory,
                                status,
                            }),
                            service: PipelineGet200ResponseEnabledInnerService {
                                author: fq.author,
                                name: fq.name,
                                version: fq.version,
                                faults: p.faults as i32,
                                exit: p.last_exit_code,
                            },
                        }
                    } else {
                        PipelineGet200ResponseEnabledInner {
                            process: None,
                            service: PipelineGet200ResponseEnabledInnerService {
                                author: fq.author,
                                name: fq.name,
                                version: fq.version,
                                faults: p.faults as i32,
                                exit: p.last_exit_code,
                            },
                        }
                    }
                }
                Err(_) => PipelineGet200ResponseEnabledInner {
                    process: None,
                    service: PipelineGet200ResponseEnabledInnerService {
                        author: fq.author,
                        name: fq.name,
                        version: fq.version,
                        faults: 0,
                        exit: 0,
                    },
                },
            });
        }

        Ok(responses)
    }

    pub async fn construct_managed_services(
        &self,
        runnable: RunnablePipeline,
    ) -> Result<(), Error> {
        // Assign the new processes state each time, so first
        // clear the existing processes and then add them again
        let mut processes = self.process_manager.processes.write().await;

        let bootspecs = bootspec::BootSpecs::new(runnable.services().clone()).0;

        let mut fqs = vec![];
        let mut service_data = vec![];

        for service in runnable.services() {
            // Create bootspecs with missing inputs, since the first step is to hand out
            // ports. After knowing the ports of all outputs, we can fill in the inputs.
            // Since we are valid, a given input will _always_ have an output.
            let fq = FqBuf::from(service);
            let bootspec = bootspecs.get(&fq);
            let injected_env = serde_json::to_string(&bootspec)?;

            // Save the necessary information from each runnable service
            fqs.push(fq);
            service_data.push((service, injected_env));
        }

        // Most of the time, we will retain all processes, however when the pipeline changes
        // we need to reflect those changes
        processes.retain(|p| fqs.contains(&p.fq));
        let fqs_and_services = fqs.iter().zip(service_data.iter());

        for (fq, (service, injected_env)) in fqs_and_services {
            if let Some(proc) = processes.iter_mut().find(|p| p.fq == *fq) {
                // If the runnable service identified by its fq already exists, there's a chance
                // that the service.yaml has changed, so update only those fields
                proc.command = service.0.commands.run.clone();
                proc.injected_env = injected_env.clone();
                proc.start_time = time_now!() as i64;
            } else {
                // The runnable service has not previously been added, so add a new one
                processes.push(Process {
                    fq: fq.clone(),
                    command: service.0.commands.run.clone(),
                    last_pid: None,
                    last_exit_code: 0,
                    name: service.0.name.clone(),
                    status: ProcessStatus::Stopped,
                    log_file: PathBuf::from(fq.log_file()),
                    injected_env: injected_env.clone(),
                    faults: 0,
                    start_time: time_now!() as i64,
                })
            }
        }

        Ok(())
    }

    pub async fn start(&self) -> Result<(), Error> {
        let enabled_services = get_config().await?.enabled;

        if enabled_services.is_empty() {
            return Err(Error::PipelineIsEmpty);
        }

        // Pipeline validation step
        let runnable = self.get_valid_pipeline().await?;

        // After this, self.processes will be ready
        self.construct_managed_services(runnable).await?;

        // Start the actual processes
        self.process_manager.start().await?;

        Ok(())
    }

    /// If the pipeline is started, it will be stopped.
    pub async fn stop(&self) -> Result<(), Error> {
        let mut stats = self.process_manager.stats.write().await;
        if stats.status != PipelineStatus::Started {
            return Err(Error::NoRunningServices);
        }
        stats.last_stop = Some(time_now!() as i64);
        self.process_manager.shutdown_tx.send(()).ok();
        let mut spawned = self.process_manager.spawned.write().await;
        spawned.clear();
        Ok(())
    }

    /// Reads the config file from disk and returns a RunnablePipeline if it is valid.
    /// If it isn't valid it returns an error and resets the config.
    pub async fn get_valid_pipeline(&self) -> Result<RunnablePipeline, Error> {
        let mut config = get_config().await?;
        let mut enabled_services: Vec<ValidatedService> = vec![];

        let res = {
            for enabled in &config.enabled {
                let service_file =
                    std::fs::read_to_string(enabled).map_err(|_| Error::ServiceNotFound)?;
                let service: Service = serde_yaml::from_str(&service_file)?;
                let validated = service.validate()?;
                enabled_services.push(validated);
            }

            Pipeline::new(enabled_services).validate()
        };
        match res {
            Ok(val) => Ok(val),
            Err(e) => {
                config.enabled.clear();
                update_config(&config)?;
                Err(Error::Validation(e))
            }
        }
    }

    pub async fn get_service_logs(
        &self,
        fq: FqBuf,
        num_lines: usize,
    ) -> Result<Vec<String>, Error> {
        let file = File::open(fq.log_file()).map_err(|_| Error::NoLogsFound)?;
        let mut reader = BufReader::new(file);

        let mut buffer = Vec::new();
        let mut lines = Vec::new();

        // Seek to the end of the file
        let mut position = reader
            .seek(SeekFrom::End(0))
            .with_context(|| format!("failed to seek in {}", fq.log_file()))?;

        // Read the file in reverse to gather lines
        while lines.len() < num_lines && position > 0 {
            // Adjust buffer size based on remaining file size
            let chunk_size = cmp::min(position as usize, 4096);
            position -= chunk_size as u64;
            reader
                .seek(SeekFrom::Start(position))
                .with_context(|| format!("failed to seek in {}", fq.log_file()))?;
            buffer.resize(chunk_size, 0);

            // Read the chunk
            reader
                .get_mut()
                .read_exact(&mut buffer)
                .with_context(|| format!("failed to read chunk for {}", fq.log_file()))?;

            // Split into lines and push to the result
            let chunk = String::from_utf8_lossy(&buffer);
            let mut chunk_lines: Vec<_> = chunk.lines().rev().map(String::from).collect();
            lines.append(&mut chunk_lines);
        }

        // Reverse the lines to restore their original order
        lines.reverse();
        Ok(lines.into_iter().take(num_lines).collect())
    }
}

/// Retrieves rover.yaml file from disk, performs validation and returns object.
pub async fn get_config() -> Result<Configuration, Error> {
    if !Path::new(ROVER_CONFIG_FILE).exists() {
        // If there is no existing config, create a new file and write
        // an empty config to it.
        let empty_config = Configuration { enabled: vec![] };
        update_config(&empty_config)?;
    }

    let file_content =
        std::fs::read_to_string(ROVER_CONFIG_FILE).map_err(|_| Error::ConfigFileIO)?;

    let config: ValidatedConfiguration =
        serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

    Ok(config.0)
}

/// Returns the process which contains the fq.
pub fn get_proc(fq: FqBuf, processes: &Vec<Process>) -> Result<&Process, Error> {
    for p in processes {
        if p.fq == fq {
            return Ok(p);
        }
    }
    Err(Error::ProcessNotFound)
}
