use std::path::Path;
use std::{fs::read_to_string, time::SystemTime};

use openapi::models::DaemonStatus;

use super::error::*;

mod health;
mod pipeline;
mod services;
mod sources;

const ROVER_INFO_PATH: &str = "/etc/rover";

// The script in src/build.rs populates a const containing the version
include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn read_rover_info() -> Result<(i32, String)> {
    let text = read_to_string(Path::new(ROVER_INFO_PATH))
        .map_err(|e| Error::RoverInfoFile(format!("Could not open {} {}", ROVER_INFO_PATH, e)))?;

    let text = text.split_whitespace().collect::<Vec<&str>>();
    if text.len() != 2 {
        return Err(Error::RoverInfoFile(format!(
            "Expected 2 lines, got {}",
            text.len()
        )));
    }
    let id: i32 = text[0].trim().parse()?;
    let rover_name: String = text[1].to_string();

    Ok((id, rover_name))
}

#[derive(Debug, Clone)]
pub struct RoverdStatus {
    status: DaemonStatus,
    version: String,
    start_time: SystemTime,
    os: String,
    rover_id: i32,
    rover_name: String,
}

impl RoverdStatus {
    fn new() -> Result<Self> {
        let (rover_id, rover_name) = read_rover_info()?;
        Ok(Self {
            status: DaemonStatus::Operational,
            version: VERSION.to_string(),
            start_time: SystemTime::now(),
            os: os_info::get().to_string(),
            rover_id,
            rover_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Roverd {
    pub status: RoverdStatus,
}

impl Roverd {
    pub fn new() -> Result<Self> {
        Ok(Self {
            status: RoverdStatus::new()?,
        })
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
