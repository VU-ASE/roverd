use axum_extra::extract::Multipart;
use openapi::models::{
    DaemonStatus, FetchPostRequest, PipelinePostRequestInner, ServicesAuthorGetPathParams,
    ServicesAuthorServiceGetPathParams, ServicesAuthorServiceVersionDeletePathParams,
    ServicesAuthorServiceVersionGetPathParams, ServicesAuthorServiceVersionPostPathParams,
};
use process::ProcessManager;





use rovervalidate::config::{Configuration, ValidatedConfiguration};
use rovervalidate::pipeline::interface::Pipeline;
use rovervalidate::service::{Service, ValidatedService};
use rovervalidate::validate::Validate;
use service::{Fq, FqBuf, FqBufVec, FqVec};
use std::fs::{self, remove_dir_all, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};

use crate::constants::*;
use crate::util::*;

pub mod process;
pub mod service;
// pub mod types;


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
        if Path::new(&delete_fq.path()).exists() {
            std::fs::remove_dir_all(delete_fq.path())?;
        } else {
            return Err(Error::ServiceNotFound);
        }

        Ok(return_bool)
    }

    pub async fn build_service(
        &self,
        _params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    pub async fn set_pipeline(
        &self,
        incoming_pipeline: Vec<PipelinePostRequestInner>,
    ) -> Result<(), Error> {
        let services = FqBufVec::from(incoming_pipeline).0;
        let mut valid_services = vec![];

        for enabled in &services {
            let service_file = std::fs::read_to_string(format!("{}/service.yaml", enabled.path()))?;
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

    pub async fn get_pipeline(&self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        // TODO run verification, check

        // self.validate()?;

        // Check on disk pipeline validates
        // if not: remove it
        //

        // TODO assign ports

        // self.process_manager.start().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        // self.process_manager.stop().await?;
        Ok(())
    }

    fn get_valid_service(&self) -> Result<Service, Error> {
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

        Err(Error::Unimplemented)
    }

    pub async fn get_valid_pipeline(&mut self) -> Result<(), Error> {
        let config = self.get_config().await?;
        let mut enabled_services: Vec<ValidatedService> = vec![];

        for enabled in config.enabled {
            let service_file = std::fs::read_to_string(&enabled)?;
            let service: Service = serde_yaml::from_str(&service_file)?;
            let validated = service.validate()?;
            enabled_services.push(validated);
        }

        let p = Pipeline::new(enabled_services).validate()?;

        Err(Error::Unimplemented)
    }
}
