// This file defines constants used across all over the crate

pub const LISTEN_ADDRESS: &str = "0.0.0.0:80";

pub const ROVER_INFO_FILE: &str = "/etc/rover";
pub const ROVER_CONFIG_DIR: &str = "/etc/roverd";
pub const ROVER_CONFIG_FILE: &str = "/etc/roverd/rover.yaml";
pub const ROVER_USER: &str = "debix";
pub const ROVER_DIR: &str = "/home/debix/.rover";
pub const LOG_DIR: &str = "/tmp/roverlog";
pub const BUILD_LOG_DIR: &str = "/tmp/roverbuildlog";

pub const ZIP_FILE: &str = "/tmp/incoming-service.zip";
pub const UNZIPPED_DIR: &str = "/tmp/incoming-service";

pub const ENV_KEY: &str = "ASE_SERVICE";
pub const START_PORT: u32 = 5700;

pub const DATA_ADDRESS: &str = "tcp://localhost";
pub const DEFAULT_LOG_LINES: i32 = 50;
