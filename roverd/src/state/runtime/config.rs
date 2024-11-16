// use std::path::PathBuf;

use std::path::PathBuf;

use crate::error::Error;

use openapi::models::SourcesPostRequest;
use rovervalidate::config::{Configuration, Downloaded, Validate, ValidatedConfiguration};
use tracing::{info, warn};

use axum::http::StatusCode;

use crate::util::download_service;

const ROVER_CONFIG_PATH: &str = "/etc/roverd/rover.yaml";

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Config;

impl Config {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub fn get(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
        let file_content =
            std::fs::read_to_string(ROVER_CONFIG_PATH).map_err(|_| Error::ConfigFileNotFound)?;

        let config: ValidatedConfiguration =
            serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

        Ok(config)
    }

    pub async fn add_source(&self, source: SourcesPostRequest) -> Result<(), Error> {
        // First, check if the source to add already exists.
        let config = self.get()?.0;
        let existing_sources = config.downloaded;

        if source.name.is_none() || source.url.is_none() || source.version.is_none() {
            return Err(Error::Http(StatusCode::BAD_REQUEST));
        }

        // The unwraps are safe, since we check them all previously
        let incoming_source = Downloaded {
            sha: None,
            name: source.name.unwrap().to_lowercase(),
            source: source.url.unwrap().to_lowercase(),
            version: source.version.unwrap().to_lowercase(),
        };

        for existing_source in &existing_sources {
            if existing_source.name.to_lowercase() == incoming_source.name
                && existing_source.source.to_lowercase() == incoming_source.source
                && existing_source.version.to_lowercase() == incoming_source.version
            {
                let error_msg = " already exists".to_string();
                warn!(error_msg);
                return Err(Error::Source(error_msg));
            }
        }

        // Source doesn't exist, download it to /tmp and move it correct place on disk

        let path = download_service(
            incoming_source.name.as_str(),
            incoming_source.version.as_str(),
        )
        .await?;

        let path = PathBuf::from(path);

        info!("download success! {path:#?}");

        // Now that Source is downloaded, insert it into config and write back to rover.yaml

        Ok(())
    }
}
