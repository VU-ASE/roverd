use axum_extra::extract::Multipart;
use openapi::models::*;
use process::{Process, ProcessManager};
use rovervalidate::config::{Configuration, ValidatedConfiguration};
use rovervalidate::pipeline::interface::{Pipeline, RunnablePipeline};
use rovervalidate::service::{Service, ValidatedService};
use rovervalidate::validate::Validate;
use service::{Fq, FqBuf, FqBufVec, FqVec};
use std::collections::HashMap;
use std::fs::{self, remove_dir_all, remove_file};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::process::Command;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, warn};

use crate::command::ParsedCommand;
use crate::error::Error;
use crate::util::*;
use crate::{constants::*, time_now};

mod bootspec;
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
    pub state: Arc<RwLock<State>>,
}

impl Roverd {
    pub async fn new() -> Result<Self, Error> {
        let roverd = Self {
            info: info::Info::new(),
            state: Arc::from(RwLock::from(State {
                process_manager: ProcessManager {
                    processes: vec![],
                    spawned: vec![],
                    shutdown_tx: broadcast::channel::<()>(1).0,
                },
                built_services: HashMap::new(),
            })),
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
    pub built_services: HashMap<FqBuf, i64>,
}

impl State {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub async fn get_config(&self) -> Result<Configuration, Error> {
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

    pub async fn should_invalidate(&self, fq_buf: &FqBuf) -> Result<bool, Error> {
        let mut config = self.get_config().await?;
        let enabled_fq = FqVec::try_from(&config.enabled)?;
        let pipeline_invalidated = enabled_fq.0.contains(&Fq::from(fq_buf));

        if pipeline_invalidated {
            config.enabled.clear();
            update_config(&config)?;
        }

        Ok(pipeline_invalidated)
    }

    pub async fn fetch_service(&self, body: &FetchPostRequest) -> Result<(FqBuf, bool), Error> {
        let fq_buf = download_and_install_service(&body.url).await?;
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
                .open(ZIP_FILE)?;

            file.write_all(&data)?;

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
        &mut self,
        path_params: &ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<bool, Error> {
        let delete_fq = FqBuf::from(path_params);

        // Get the current configuration from disk
        let mut config = self.get_config().await?;

        let mut should_reset = false;
        // Return whether or not the service was enabled and if it was,
        // reset the pipeline
        let enabled_fq_vec = FqBufVec::try_from(&config.enabled)?.0;

        if enabled_fq_vec.contains(&delete_fq) {
            should_reset = true;
            config.enabled.clear();
            update_config(&config)?;

            self.built_services.remove(&delete_fq);
        }

        // Remove the service to delete from the filesystem
        if Path::new(&delete_fq.dir()).exists() {
            std::fs::remove_dir_all(delete_fq.dir())?;
        } else {
            return Err(Error::ServiceNotFound);
        }

        Ok(should_reset)
    }

    pub async fn build_service(
        &mut self,
        params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        let fq = FqBuf::from(&params);
        let service = self.get_service(fq.clone()).await?.0;

        let build_string = &service
            .commands
            .build
            .ok_or_else(|| Error::BuildCommandMissing)?;
        let build_command = ParsedCommand::try_from(build_string)?;
        let log_file = create_log_file(&PathBuf::from(fq.build_log_file()))?;
        let stdout = Stdio::from(log_file.try_clone()?);
        let stderr = Stdio::from(log_file);
        let program = &build_command.program;
        let arguments = &build_command.arguments;

        match Command::new(program)
            .args(arguments)
            .stdout(stdout)
            .stderr(stderr)
            .current_dir(fq.dir())
            .spawn()
        {
            Ok(mut child) => match child.wait().await {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        // Build was not successful, return logs
                        let file = std::fs::File::open(fq.build_log_file())?;
                        let reader = BufReader::new(file);
                        let lines: Vec<String> =
                            reader.lines().collect::<Result<Vec<String>, _>>()?;
                        return Err(Error::BuildLog(lines));
                    } else {
                        // Build was successful, save time it was built at
                        let time_now = time_now!() as i64;

                        dbg!(&self.built_services);

                        *self.built_services.entry(fq).or_insert_with(|| time_now) = time_now;
                    }
                    Ok(())
                }
                Err(e) => {
                    error!(
                        "failed to wait on build command: {:?} {}",
                        &build_command, e
                    );
                    Err(Error::BuildCommandFailed)
                }
            },
            Err(e) => {
                error!("failed to spawn build command: {:?} {}", &build_command, e);
                Err(Error::BuildCommandFailed)
            }
        }
    }

    pub async fn set_pipeline(
        &self,
        incoming_pipeline: Vec<PipelinePostRequestInner>,
    ) -> Result<(), Error> {
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
        let mut config = self.get_config().await?;
        config.enabled.clear();

        // Services are valid since we didn't return earlier
        for service in services {
            config.enabled.push(service.path())
        }

        update_config(&config)?;

        Ok(())
    }

    // todo implement this
    pub fn get_proc(&self, fq: FqBuf) -> Result<&Process, Error> {
        for p in self.process_manager.processes.iter() {
            if p.fq == fq {
                return Ok(p);
            }
        }
        Err(Error::ProcessNotFound)
    }

    pub async fn get_pipeline(&mut self) -> Result<Vec<PipelineGet200ResponseEnabledInner>, Error> {
        // todo: a pipeline can only be valid, meaning that a pipeline enabled on disk is
        // always valid. if the pipeline from the rover.yaml file is not valid, clear it.
        // let conf = self.get_valid_pipeline().await?;
        let conf = self.get_config().await?;

        let responses = conf
            .enabled
            .into_iter()
            .map(|validated_service| {
                let fq = FqBuf::try_from(validated_service)?;

                // todo get proc and populate cpu, uptime, memory, status
                let proc = self.get_proc(fq.clone());
                match proc {
                    Ok(p) => {
                        let status = p.state.clone();
                        let cpu = 32;
                        let pid = p.last_pid;
                        let uptime = 32;
                        let memory = 32;

                        Ok(PipelineGet200ResponseEnabledInner {
                            process: Some(PipelineGet200ResponseEnabledInnerProcess {
                                cpu,
                                pid: pid as i32,
                                uptime,
                                memory,
                                status,
                            }),
                            service: PipelineGet200ResponseEnabledInnerService {
                                author: fq.author,
                                name: fq.name,
                                version: fq.version,
                                faults: Some(p.faults as i32),
                            },
                        })
                    }
                    Err(e) => Ok(PipelineGet200ResponseEnabledInner {
                        process: None,
                        service: PipelineGet200ResponseEnabledInnerService {
                            author: fq.author,
                            name: fq.name,
                            version: fq.version,
                            faults: Some(0),
                        },
                    }),
                }

                // Ok::<_, Error>(PipelineGet200ResponseEnabledInner {
                //     process: Some(PipelineGet200ResponseEnabledInnerProcess {
                //         cpu: 69,
                //         pid: 69,
                //         uptime: 69,
                //         memory: 69,
                //         status: ProcessStatus::Terminated,
                //     }),
                //     service: PipelineGet200ResponseEnabledInnerService {
                //         author: fq.author,
                //         name: fq.name,
                //         version: fq.version,
                //         faults: None,
                //     },
                // })
            })
            .collect::<Result<Vec<PipelineGet200ResponseEnabledInner>, Error>>()?;

        Ok(responses)
    }

    pub async fn construct_managed_services(&mut self) -> Result<(), Error> {
        // Assign the new processes state each time, so first
        // clear the existing processes and then add them again
        self.process_manager.processes.clear();

        let runnable = self.get_valid_pipeline().await?;

        let bootspecs = bootspec::BootSpecs::new(runnable.services()).0;

        for service in runnable.services() {
            // Create bootspecs with missing inputs, since the first step is to hand out
            // ports. After knowing the ports of all outputs, we can fill in the inputs.
            // Since we are valid, a given input will _always_ have an output.

            let fq = FqBuf::from(service);

            let bootspec = bootspecs.get(&fq);
            let injected_env = serde_json::to_string(&bootspec)?;

            self.process_manager.processes.push(Process {
                fq: fq.clone(),
                command: service.0.commands.run.clone(), // run the command in the service's working directory
                last_pid: 0,
                last_exit_code: Some(0),
                name: service.0.name.clone(),
                state: ProcessStatus::Stopped,
                log_file: PathBuf::from(fq.log_file()),
                injected_env,
                faults: 0,
            })
        }

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let enabled_services = self.get_config().await?.enabled;

        if enabled_services.is_empty() {
            return Err(Error::PipelineIsEmpty);
        }

        // Pipeline validation step
        self.get_valid_pipeline().await?;

        // After this, self.processes will be ready
        self.construct_managed_services().await?;

        // Start the actual processes
        self.process_manager.start().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        if self.process_manager.spawned.is_empty() {
            return Err(Error::NoRunningServices);
        }
        self.process_manager.stop().await?;
        Ok(())
    }

    pub async fn get_valid_pipeline(&mut self) -> Result<RunnablePipeline, Error> {
        let mut config = self.get_config().await?;
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
        let file = std::fs::File::open(fq.log_file()).map_err(|_| Error::NoLogsFound)?;
        let reader = BufReader::new(file);
        let log_lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;
        let min_lines = std::cmp::min(num_lines, log_lines.len());
        let index = log_lines.len() - min_lines;

        Ok(log_lines[index..].to_vec())
    }
}
