use crate::{error::Error, util::*};

use std::path::Path;

use axum::routing::delete;
use openapi::models::SourcesPostRequest;
use rovervalidate::config::{Configuration, Downloaded, Validate, ValidatedConfiguration};

use crate::util::download_and_install_service;

use tracing::{error, info, warn};

use crate::constants::*;

use super::services::{FqService, FqVec};

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Sources;

impl Sources {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub async fn get(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
        if !Path::new(ROVER_CONFIG_FILE).exists() {
            // If there is no existing config, create a new file and write
            // an empty config to it.
            let empty_config = Configuration {
                enabled: vec![],
                downloaded: vec![],
            };

            update_config(&empty_config)?;
        }

        let file_content = std::fs::read_to_string(ROVER_CONFIG_FILE)
            .map_err(|_| Error::CouldNotCreateConfigFile)?;

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
            // Clear pipeline if incoming service is replacing one that is enabled
            let enabled_fq_vec = FqVec::try_from(&config.enabled)?.0;
            if enabled_fq_vec.contains(&incoming_fq) {
                config.enabled.clear();
            }
            std::fs::remove_dir_all(incoming_fq.path())?;
        }

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

        // Delete service files on disk if there are any
        delete_service_from_disk(&fq_to_delete)?;

        // Remove service from list of downloads if it was there
        remove_download(&mut config, &fq_to_delete);

        // Clear the enabled pipeline if it was there
        remove_enabled(&mut config, &fq_to_delete);

        // Write the updated config back to disk
        update_config(&config)?;
        info!("Deleted source: {}", fq_to_delete);

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

fn remove_download(config: &mut Configuration, fq: &FqService) {
    // Remove from config file
    let delete_index = config
        .downloaded
        .iter()
        .position(|x| FqService::from(x) == *fq);

    if let Some(i) = delete_index {
        config.downloaded.remove(i);
    }
}

fn remove_enabled(config: &mut Configuration, fq: &FqService) {}
