use axum_extra::extract::Multipart;
use openapi::models::{
    DaemonStatus, FetchPostRequest, ServicesAuthorGetPathParams,
    ServicesAuthorServiceGetPathParams, ServicesAuthorServiceVersionDeletePathParams,
    ServicesAuthorServiceVersionGetPathParams, ServicesAuthorServiceVersionPostPathParams,
};
use process::ProcessManager;
use rovervalidate::config::{Configuration, ValidatedConfiguration};
use rovervalidate::service::{Service, ValidatedService};
use rovervalidate::validate::Validate;
use service::{FqService, FqServiceBuf, FqVec};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};

use crate::constants::*;
use crate::util::*;

pub mod process;
pub mod service;

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
    pub async fn get_config(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
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

        Ok(config)
    }

    pub async fn fetch_service(
        &self,
        body: &FetchPostRequest,
    ) -> Result<(FqServiceBuf, bool), Error> {
        let conf = self.get_config().await?;
        let fq_buf = download_and_install_service(&body.url).await?;
        let enabled_fq = FqVec::try_from(&conf.0.enabled)?;
        let pipeline_invalidated = enabled_fq.0.contains(&FqService::from(&fq_buf));

        Ok((fq_buf, pipeline_invalidated))
    }

    pub async fn receive_upload(&self, mut body: Multipart) -> Result<(FqServiceBuf, bool), Error> {
        while let Some(field) = body.next_field().await.map_err(|_| Error::ServiceUploadData)? {
            let data = field.bytes().await.unwrap();
            info!(
                ">>> Received {} bytes",
                data.len()
            );
        }

        Err(Error::Unimplemented)
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
        let contents =
            fs::read_to_string(service_file_path).map_err(|_| Error::ServiceNotFound)?;
        let service =
            serde_yaml::from_str::<rovervalidate::service::Service>(&contents)?.validate()?;

        Ok(service)
    }

    pub async fn delete_service(
        &self,
        path_params: &ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<bool, Error> {
        let delete_fq = FqService::from(path_params);

        // Get the current configuration from disk
        let mut config = self.get_config().await?.0;

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
            return Err(Error::ServiceNotFound)
        }

        Ok(return_bool)
    }

    pub async fn build_service(
        &self,
        _params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
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

    async fn validate(&mut self) -> Result<(), Error> {
        let config = self.get_config().await?.0;
        info!("config: {:?}", config);

        let mut enabled_services: Vec<ValidatedService> = vec![];

        for enabled in config.enabled {
            let service_file = std::fs::read_to_string(&enabled)?;
            let service: Service = serde_yaml::from_str(&service_file)?;
            let validated = service.validate()?;
            enabled_services.push(validated);
        }
        // let p = Pipeline::new(enabled_services).validate()?;
        // info!("{:#?}", p);
        Err(Error::Unimplemented)
    }
}
