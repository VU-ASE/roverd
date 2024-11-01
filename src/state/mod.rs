use std::path::Path;
use std::{fs::read_to_string, time::SystemTime};

use openapi::models::DaemonStatus;
use tracing::{error, info};

mod health;
mod pipeline;
mod services;
mod sources;

const ROVER_INFO_PATH: &str = "/etc/rover";

use super::Error;

// The script in src/build.rs populates a const containing the version
include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn read_rover_info() -> Result<(i32, String), Error> {
    let text = read_to_string(Path::new(ROVER_INFO_PATH))
        .map_err(|e| Error::RoverInfoFileIo(ROVER_INFO_PATH.to_string(), e))?;

    let text = text.split_whitespace().collect::<Vec<&str>>();
    if text.len() != 2 {
        return Err(Error::RoverInfoFileFormat(format!(
            "Expected 2 lines, got {}",
            text.len()
        )));
    }
    let id: i32 = text[0].trim().parse().map_err(|e| {
        Error::RoverInfoFileFormat(format!("Invalid format of {}, {}", ROVER_INFO_PATH, e))
    })?;

    let rover_name: String = text[1].to_string();

    Ok((id, rover_name))
}

#[derive(Debug, Clone)]
pub struct Info {
    status: DaemonStatus,
    version: String,
    start_time: SystemTime,
    os: String,
    rover_id: Option<i32>,
    rover_name: Option<String>,
    error_msg: Option<String>,
}

impl Info {
    fn new() -> Self {
        let mut status = DaemonStatus::Operational;

        let (id, name, msg) = match read_rover_info() {
            Ok((id, name)) => (Some(id), Some(name), None),
            Err(e) => {
                error!("{:?}", e);
                status = DaemonStatus::Recoverable;
                (None, None, Some(format!("{:?}", e)))
            }
        };

        Self {
            status,
            version: VERSION.to_string(),
            start_time: SystemTime::now(),
            os: os_info::get().to_string(),
            rover_id: id,
            rover_name: name,
            error_msg: msg,
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    InvalidRunnable,
    ValidRunnable,
    ValidRunning,
}

#[derive(Debug, Clone)]
pub struct Roverd {
    pub info: Info,
    pub state: State,
}

impl Roverd {
    pub fn new() -> Self {
        let roverd = Self {
            info: Info::new(),
            state: State::InvalidRunnable,
        };

        if roverd.info.status == DaemonStatus::Operational {
            info!("initialized successully: {:#?}", roverd);
        }

        roverd
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
