use std::sync::Arc;

use tokio::sync::RwLock;

use super::process::{Process, SpawnedProcess};

#[derive(Debug, Clone)]
pub struct DaemonManager {
    /// Contains the "application view" of process after validation. In-between start / stop
    /// runs this vec remains unchanged.
    pub processes: Arc<RwLock<Vec<Process>>>,

    /// The "runtime" view of all processes, this contains handles to the spawned children.
    pub spawned: Arc<RwLock<Vec<SpawnedProcess>>>,
}



