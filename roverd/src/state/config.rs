// use std::path::PathBuf;

use tracing::info;

// use rovervalidate::config::Configuration;

const ROVER_CONFIG_PATH: &str = "/etc/roverd/rover.yaml";

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Config {
    // roverd:
}

impl Config {
    pub fn new() -> Self {
        info!("initializing config");

        // let config_file_result = std::fs::read_to_string(PathBuf::from(ROVER_CONFIG_PATH));

        // let raw_content = match config_file_result {
        //     Ok(_) => {}
        //     Err(_) => {}
        // };

        // let config: Configuration = serde_yaml::from_str(&raw_content);

        Config {}
    }
}
