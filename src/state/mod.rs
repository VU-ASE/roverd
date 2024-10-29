use std::time::{SystemTime};

use openapi::models::DaemonStatus;

mod health;
mod pipeline;
mod services;
mod sources;

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
    fn new() -> Self {
        Self {
            status: DaemonStatus::Operational,
            version: String::from("todo roverd version"),
            start_time: SystemTime::now(),
            os: String::from("todo os version"),
            rover_id: 69, // todo get rover_id
            rover_name: String::from("todo get rover"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Roverd {
    pub status: RoverdStatus,
}

impl Roverd {
    pub fn new() -> Self {
        Self {
            status: RoverdStatus::new()
        }
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
