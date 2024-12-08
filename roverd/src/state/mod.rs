use openapi::models::DaemonStatus;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub mod rover;
pub mod services;
pub mod sources;

/// Start-up information and system clock
pub mod info;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct State {
    /// Run-time encapsulation of pipeline data (running processes)
    pub core: rover::Core,

    /// Handle for querying and modifying services
    pub services: services::Services,

    /// Handle for querying and modifying services
    pub sources: sources::Sources,
}

/// The main struct that implements functions called from the api and holds all objects
/// in memory necessary for operation.
#[derive(Debug, Clone)]
pub struct Roverd {
    /// Information related to the roverd daemon, contains status.
    pub info: info::Info,

    /// Run-time data structures of the Rover, interacts with the file system
    /// and spawns processes, so must be read/write locked.
    pub state: Arc<RwLock<State>>,
}

impl Roverd {
    pub async fn new() -> Result<Self, Error> {
        // Initialize sources, download if necessary
        sources::Sources.install_missing_sources().await?;

        let roverd = Self {
            info: info::Info::new(),
            state: Arc::from(RwLock::from(State {
                core: rover::Core::new(),
                sources: sources::Sources,
                services: services::Services,
            })),
        };

        if roverd.info.status != DaemonStatus::Operational {
            warn!("did not initialize successfully {:#?}", roverd);
        }

        Ok(roverd)
    }
}

impl AsRef<Roverd> for Roverd {
    fn as_ref(&self) -> &Roverd {
        self
    }
}
