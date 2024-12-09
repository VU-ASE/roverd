use crate::{error::Error, util::*};

use openapi::models::SourcesPostRequest;
use rovervalidate::config::{Configuration, Downloaded, Validate, ValidatedConfiguration};

use crate::util::download_and_install_service;

use tracing::{error, info, warn};

use crate::constants::*;

use super::services::FqService;

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Sources;

impl Sources {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub async fn get(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
        let file_content =
            std::fs::read_to_string(ROVER_CONFIG_FILE).map_err(|_| Error::ConfigFileNotFound)?;

        let config: ValidatedConfiguration =
            serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

        Ok(config)
    }

    pub async fn add(&self, source: SourcesPostRequest) -> Result<(), Error> {
        let mut config = self.get().await?.0;

        let incoming_fq = FqService::from(&source);

        let source_url = incoming_fq.url.ok_or(Error::MissingUrl)?;

        if source_url.contains("http") {
            return Err(Error::Generic(
                "source url should not contain schema, remove 'http...'".to_string(),
            ));
        }

        for existing_source in &config.downloaded {
            let rhs = FqService::from(existing_source);

            if incoming_fq == rhs {
                return Err(Error::SourceAlreadyExists);
            }
        }

        // If it exists, delete it and re-add it
        if service_exists(&incoming_fq)? {
            std::fs::remove_dir_all(incoming_fq.path())?;
        }

        config.enabled.clear();

        download_and_install_service(&incoming_fq).await?;

        // If the download was successful, add it to the config file
        config.downloaded.push(Downloaded {
            name: incoming_fq.name.to_string(),
            source: source_url.to_string(),
            version: incoming_fq.version.to_string(),
            sha: None, // todo add sha
        });
        update_config(&config)?;

        Ok(())
    }

    pub async fn delete(&self, source: SourcesPostRequest) -> Result<(), Error> {
        let mut config = self.get().await?.0;
        let fq_to_delete = FqService::from(&source);

        if !download_exists(&config, &fq_to_delete) {
            return Err(Error::SourceNotFound);
        }

        // Remove from config file
        let delete_index = config
            .downloaded
            .iter()
            .position(|x| FqService::from(x) == fq_to_delete)
            .unwrap();
        config.downloaded.remove(delete_index);

        // Delete files on disk
        if service_exists(&fq_to_delete)? {
            std::fs::remove_dir_all(fq_to_delete.path())?;
        }

        config.enabled.clear();

        // If the directory has been removed, update the file on disk
        update_config(&config)?;
        info!("Deleted {}", fq_to_delete);

        Ok(())
    }

    /// Idempotent operation that downloads and installs any
    /// missing sources based on roverd config file.
    pub async fn install_missing_sources(&self) -> Result<(), Error> {
        let config = self.get().await?.0;

        for existing_source in &config.downloaded {
            let fq = FqService::from(existing_source);

            if !service_exists(&fq)? {
                info!("Missing source: {} ", fq);
                download_and_install_service(&fq).await?;
            } else {
                info!("Source {} installed", fq);
            }
        }

        Ok(())
    }
}
