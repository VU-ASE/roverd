use crate::{error::Error, util::*};

use std::path::Path;

use rovervalidate::config::{Configuration, Validate, ValidatedConfiguration};

use crate::constants::*;

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Config;

impl Config {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub async fn get(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
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
}
