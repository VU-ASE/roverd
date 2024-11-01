use std::path::Path;
use std::{fs::read_to_string, time::SystemTime};

use openapi::models::DaemonStatus;
use tokio::sync::broadcast::error;
use tracing::{error, info};

mod health;
mod pipeline;
mod services;
mod sources;

// The script in src/build.rs populates a const containing the version
include!(concat!(env!("OUT_DIR"), "/version.rs"));

const ROVER_INFO_PATH: &str = "/etc/rover";

const ROVER_SHADOW_PATH: &str = "/etc/shadow";

/// The rover will never be used with a different user.
const ROVER_USER: &str = "debix";

use super::Error;

fn get_password() -> Result<String, Error> {
    // We can read /etc/shadow since we are root

    let user = pgs_files::shadow::get_entry_by_name(ROVER_USER)
        .ok_or_else(|| Error::RoverPassword(format!("Could not find user '{}'", ROVER_USER)))?;

    info!("{:#?}", user.passwd);

    Ok(user.passwd)
}

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
    username: String,
    password: Option<String>,
    error_msg: Option<String>,
}

impl Info {
    fn new() -> Self {
        let mut status = DaemonStatus::Operational;

        let (id, name, mut msg) = match read_rover_info() {
            Ok((id, name)) => (Some(id), Some(name), None),
            Err(e) => {
                error!("{:?}", e);
                status = DaemonStatus::Recoverable;
                (None, None, Some(format!("{:?}", e)))
            }
        };

        let password = match get_password() {
            Ok(pass) => Some(pass),
            Err(e) => {
                error!("{:?}", e);
                status = DaemonStatus::Recoverable;
                msg = Some(format!("{:?}", e));
                None
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
            password,
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
