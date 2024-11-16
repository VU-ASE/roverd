use std::path::Path;
use std::{fs::read_to_string, time::SystemTime};

use openapi::models::DaemonStatus;
use tracing::{error, info};

mod health;
mod pipeline;
mod services;
mod sources;

// The script in src/build.rs populates a const containing the version
include!(concat!(env!("OUT_DIR"), "/version.rs"));

const ROVER_INFO_PATH: &str = "/etc/rover";

/// The rover will never be used with a different user.
const ROVER_USER: &str = "debix";

use super::Error;

/// Reads the /etc/rover file and parses out basic information. Expects to see
/// the rover's id on the first line, the rover's name on the second, and a sha256
/// hash of the user password on the last line.
fn read_rover_info() -> Result<(i32, String, String), Error> {
    let text = read_to_string(Path::new(ROVER_INFO_PATH))
        .map_err(|e| Error::RoverInfoFileIo(ROVER_INFO_PATH.to_string(), e))?;

    let text = text.split_whitespace().collect::<Vec<&str>>();
    if text.len() < 3 {
        return Err(Error::RoverInfoFileFormat(format!(
            "Expected 3 lines, got {}",
            text.len()
        )));
    }
    let id: i32 = text[0].trim().parse().map_err(|e| {
        Error::RoverInfoFileFormat(format!("Invalid format of {}, {}", ROVER_INFO_PATH, e))
    })?;

    let rover_name: String = text[1].to_string();

    let pass_hash: String = text[2].to_string();

    Ok((id, rover_name, pass_hash))
}

#[derive(Debug, Clone)]
pub struct Info {
    pub status: DaemonStatus,
    pub version: String,
    pub start_time: SystemTime,
    pub os: String,
    pub rover_id: Option<i32>,
    pub rover_name: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub error_msg: Option<String>,
}

impl Info {
    fn new() -> Self {
        let mut status = DaemonStatus::Operational;

        let (id, name, hash, msg) = match read_rover_info() {
            Ok((id, name, hash)) => (Some(id), Some(name), Some(hash), None),
            Err(e) => {
                error!("{:?}", e);
                status = DaemonStatus::Recoverable;
                (None, None, None, Some(format!("{:?}", e)))
            }
        };

        Self {
            status,
            version: VERSION.to_string(),
            start_time: SystemTime::now(),
            os: os_info::get().to_string(),
            rover_id: id,
            rover_name: name,
            username: ROVER_USER.to_string(),
            password: hash,
            error_msg: msg,
        }
    }
}

#[derive(Debug, Clone)]
pub enum State {
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
