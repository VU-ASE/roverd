// use std::path::PathBuf;

use tracing::info;

use crate::error::Error;

use rovervalidate::config::{Configuration, Validate, ValidatedConfiguration};

const ROVER_CONFIG_PATH: &str = "/etc/roverd/rover.yaml";

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Config;

impl Config {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub fn get(&self) -> Result<rovervalidate::config::Configuration, Error> {
        let file_content =
            std::fs::read_to_string(ROVER_CONFIG_PATH).map_err(|_| Error::ConfigFileNotFound)?;

        let config: ValidatedConfiguration =
            serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

        info!("{:#?}", config);

        Err(Error::ConfigValidation)
    }
}
