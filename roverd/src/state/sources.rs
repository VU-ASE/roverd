use std::fs::write;

use crate::{error::Error, util};

use openapi::models::SourcesPostRequest;
use rovervalidate::config::{Configuration, Downloaded, Validate, ValidatedConfiguration};

use crate::util::download_and_install_service;

use tracing::{info, error};

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
        // First, check if the source to add already exists.
        let mut config = self.get().await?.0;

        let incoming_source = Downloaded {
            sha: None,
            name: source.name.to_lowercase(),
            source: source.url.to_lowercase(),
            version: source.version.to_lowercase(),
        };

        if incoming_source.source.contains("http") {
            return Err(Error::Source(
                "source url should not contain schema, remove 'http...'".to_string(),
            ));
        }

        for existing_source in &config.downloaded {
            if existing_source.name.to_lowercase() == incoming_source.name
                && existing_source.source.to_lowercase() == incoming_source.source
                && existing_source.version.to_lowercase() == incoming_source.version
            {
                let error_msg = "already exists".to_string();
                return Err(Error::Source(error_msg));
            }
        }

        // Update the config file with the newly added source
        config.downloaded.push(incoming_source);
        let contents = serde_yaml::to_string(&config)?;
        write(ROVER_CONFIG_FILE, contents)?;

        // Based on the updated config file, download & install all sources that may be missing
        self.install_missing_sources().await?;

        Ok(())
    }

    /// Idempotently installs any missing sources based on roverd config file.
    pub async fn install_missing_sources(&self) -> Result<(), Error> {
        let config = self.get().await?.0;

        for existing_source in &config.downloaded {
            let fq_service = FqService {
                name: &existing_source.name,
                author: AUTHOR,
                version: &existing_source.version,
            };

            if !util::service_exists(&fq_service)? {
                info!("Service {} does not exist, downloading", &fq_service.name);
                download_and_install_service(&fq_service).await?;
            } else {
                info!("Service {} already installed", &fq_service.name);
            }
        }

        Ok(())
    }

    pub async fn delete(&self, _source: SourcesPostRequest) -> Result<(), Error> {
        error!("TODO: unimplemented");
        Ok(())
    }
}
