use openapi::models::DaemonStatus;
use tracing::{info, warn};

/// Run-time api handler functions and mutable state
mod runtime;
use runtime::*;

/// Start-up information and system clock
mod info;

#[derive(Debug, Clone)]
enum State {
    InvalidRunnable,
    // ValidRunnable,
    // ValidRunning,
}

/// The main struct that implements functions called from the api and holds all objects
/// in memory necessary for operation.
#[derive(Debug, Clone)]
pub struct Roverd {
    /// Information related to the roverd daemon, contains status.
    pub info: info::Info,

    /// Run-time state machine of the rover
    state: State,

    /// Run-time encapsulation of pipeline data (running processes)
    pub pipeline: pipeline::Pipeline,

    /// Handle for querying and modifying services
    pub services: services::Services,

    /// Handle for querying and modifying services
    pub sources: sources::Sources,
}

impl Roverd {
    pub fn new() -> Self {
        let roverd = Self {
            info: info::Info::new(),
            state: State::InvalidRunnable,
            sources: sources::Sources,
            pipeline: pipeline::Pipeline::new(),
            services: services::Services::new(),
        };

        if roverd.info.status == DaemonStatus::Operational {
            info!("initialized successfully");
        } else {
            warn!("did not initialize successfully {:#?}", roverd);
        }

        roverd
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
