use axum_extra::extract::Multipart;
use openapi::models::*;
use process::{Process, ProcessManager};

use rovervalidate::config::{Configuration, ValidatedConfiguration};
use rovervalidate::pipeline::interface::{Pipeline, RunnablePipeline};
use rovervalidate::service::{Service, ValidatedService};
use rovervalidate::validate::Validate;
use service::{Fq, FqBuf, FqBufVec, FqVec};
use std::fs::{self, remove_dir_all, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};

use crate::constants::*;
use crate::util::*;

pub mod process;
pub mod service;

mod bootspec;

/// Start-up information and system clock
pub mod info;

use crate::error::Error;

/// The main struct that implements functions called from the api and holds all objects
/// in memory necessary for operation.
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
                runnable: None,
                process_manager: ProcessManager {
                    processes: vec![],
                    spawned: vec![],
                    shutdown_tx: broadcast::channel::<()>(1).0,
                },
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
    pub runnable: Option<RunnablePipeline>,
    pub process_manager: ProcessManager,
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

        let file_content = std::fs::read_to_string(ROVER_CONFIG_FILE)
            .map_err(|_| Error::CouldNotCreateConfigFile)?;

        let config: ValidatedConfiguration =
            serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

        Ok(config.0)
    }

    pub async fn should_invalidate(&self, fq_buf: &FqBuf) -> Result<bool, Error> {
        let conf = self.get_config().await?;
        let enabled_fq = FqVec::try_from(&conf.enabled)?;
        let pipeline_invalidated = enabled_fq.0.contains(&Fq::from(fq_buf));
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
            .map_err(|_| Error::ServiceUploadData)?
        {
            let data = field.bytes().await.map_err(|_| Error::IncorrectPayload)?;

            // Ignore errors, since filesystem can be in any state
            let _ = remove_file(ZIP_FILE);
            let _ = remove_dir_all(UNZIPPED_DIR);

            let mut file = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(ZIP_FILE)?;

            file.write_all(&data)?;

            let fq_buf = extract_fq().await?;

            if service_exists(&Fq::from(&fq_buf))? {
                return Err(Error::ServiceAlreadyExists);
            }

            install_service(&fq_buf).await?;

            let invalidate_pipline = self.should_invalidate(&fq_buf).await?;

            return Ok((fq_buf, invalidate_pipline));
        }
        Err(Error::IncorrectPayload)
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

    pub async fn get_service(
        &self,
        path_params: ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ValidatedService, Error> {
        // Load config from file on disk
        let service_file_path = format!(
            "{}/{}/{}/{}/service.yaml",
            ROVER_DIR, path_params.author, path_params.service, path_params.version
        );
        let contents = fs::read_to_string(service_file_path).map_err(|_| Error::ServiceNotFound)?;
        let service =
            serde_yaml::from_str::<rovervalidate::service::Service>(&contents)?.validate()?;

        Ok(service)
    }

    pub async fn delete_service(
        &self,
        path_params: &ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<bool, Error> {
        let delete_fq = Fq::from(path_params);

        // Get the current configuration from disk
        let mut config = self.get_config().await?;

        let mut return_bool = false;
        // Return whether or not the service was enabled and if it was,
        // reset the pipeline
        let enabled_fq_vec = FqVec::try_from(&config.enabled)?.0;
        if enabled_fq_vec.contains(&delete_fq) {
            return_bool = true;
            config.enabled.clear();
            update_config(&config)?;
        }

        // Remove the service to delete from the filesystem
        if Path::new(&delete_fq.dir()).exists() {
            std::fs::remove_dir_all(delete_fq.dir())?;
        } else {
            return Err(Error::ServiceNotFound);
        }

        Ok(return_bool)
    }

    pub async fn build_service(
        &self,
        _params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        // Todo build process async with tokio::process::Command

        Err(Error::Unimplemented)
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

        // If we got here, config can be overwritten
        let mut config = self.get_config().await?;
        config.enabled.clear();

        // Services are valid since we didn't return earlier
        for service in services {
            config.enabled.push(service.path())
        }

        update_config(&config)?;

        Ok(())
    }

    pub async fn get_pipeline(&self) -> Result<Vec<PipelineGet200ResponseEnabledInner>, Error> {
        let conf = self.get_config().await?;

        let responses = conf
            .enabled
            .into_iter()
            .map(|e| {
                let fq_buf = FqBuf::try_from(e)?;

                Ok::<_, Error>(PipelineGet200ResponseEnabledInner {
                    process: Some(PipelineGet200ResponseEnabledInnerProcess {
                        cpu: 69,
                        pid: 69,
                        uptime: 69,
                        memory: 69,
                        status: ProcessStatus::Terminated,
                    }),
                    service: PipelineGet200ResponseEnabledInnerService {
                        author: fq_buf.author,
                        name: fq_buf.name,
                        version: fq_buf.version,
                        faults: None,
                    },
                })
            })
            .collect::<Result<Vec<PipelineGet200ResponseEnabledInner>, Error>>()?;

        Ok(responses)
    }

    pub async fn construct_managed_services(&mut self) -> Result<(), Error> {
        // Assign the new processes state each time, so first
        // clear the existing processes and then add them again
        self.process_manager.processes.clear();

        let mut start_port = START_PORT;

        if let Some(runnable) = &self.runnable {
            // Create bootspecs with missing inputs, since the first step is to hand out
            // ports. After knowing the ports of all outputs, we can fill in the inputs.
            // Since we are valid, a given input will _always_ have an output.
            let mut bootspecs = runnable
                .services()
                .iter()
                .map(|s| bootspec::BootSpec::new(&mut start_port, s))
                .collect::<Vec<_>>();

            // Now, for all services we can fill in the missing inputs.
            // for b in bootspecs.iter_mut() {
            //     b.fill_dependencies(runnable);
            // }

            
            // bootspec::BootSpec::fill_dependencies(runnable, bootspecs);

            // for (service_index, service) in runnable.services().iter().enumerate() {
            //     info!("first zmq: {}", start_port);

            //     let indicies = runnable.dependencies.get(&service_index);

            //     if let Some(is) = indicies {
            //         let a = is.iter().map(|f| f);
            //     }

            //     // .iter().map(|i| runnable.services()[i]);

            //     // let mut p = Process::new();

            //     // let fq = FqBuf::from(service);
            //     // let last_pid = 0;
            //     // let last_exit_code: Option<i32> = None;
            //     // let name = service.0.name.clone();
            //     // let command = service.0.commands.run.clone();
            //     // let log_file = PathBuf::from("asdf");
            //     // let state = openapi::models::ProcessStatus::Stopped;

            //     // let injected_env = bootspec::BootSpec::new(&mut zmq_start, service.0)?;

            //     info!("injected_env: {}, {}", "asd", zmq_start);
            // }
        } else {
            return Err(Error::NoRunnableServices);
        }

        // On start from scratch:
        // for each service:
        //  - make name of log_file
        //  - generate bootspec (assign port numbers)

        //

        // for service in runnable.services() {
        //     self.process_manager.processes.push(Process {
        //         command: service.0.commands.run.clone(),
        //         last_pid: 0,
        //         last_exit_code: Some(0),
        //         name: service.0.name,
        //         state: ProcessStatus::Stopped,
        //         log_file: get_log_file(&Service),
        //         injected_env: ,
        //     })
        // }

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

        // Start the processes
        // self.process_manager.start().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        // self.process_manager.stop().await?;
        Ok(())
    }

    fn get_valid_service(&self) -> Result<Service, Error> {
        let config_file =
            std::fs::read_to_string(ROVER_CONFIG_FILE).map_err(|_| Error::ConfigFileNotFound)?;
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

        Err(Error::Unimplemented)
    }

    pub async fn get_valid_pipeline(&mut self) -> Result<RunnablePipeline, Error> {
        let config = self.get_config().await?;
        let mut enabled_services: Vec<ValidatedService> = vec![];

        for enabled in config.enabled {
            let service_file =
                std::fs::read_to_string(&enabled).map_err(|_| Error::ServiceNotFound)?;
            let service: Service = serde_yaml::from_str(&service_file)?;
            let validated = service.validate()?;
            enabled_services.push(validated);
        }

        let p = Pipeline::new(enabled_services).validate()?;

        Ok(p)
    }
}
