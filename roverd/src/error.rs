#![allow(unused)]

use tracing::warn;

use derive_more::From;

// mod state;
// use state::*;
use crate::state::rover::process::ProcessManager;

#[derive(Debug, From)]
pub enum Error {
    RoverInfoFileIo(String, std::io::Error),
    RoverInfoFileFormat(String),

    ConfigFileNotFound,

    #[from]
    ConfigValidation(Vec<rovervalidate::error::Error>),

    // --- Roverd Generic ---
    Generic(String),

    // --- Source Errors ---
    SourceAlreadyExists,
    SourceNotFound,

    // --- Service Errors ---
    ServiceValidation,
    ServiceNotFound,
    ServiceAlreadyExists,
    ServiceParseIncorrect,
    ServiceDownloadFailed,

    PipielineValidation,

    #[from]
    Serialization(serde_yaml::Error),

    Download,

    #[from]
    Zip(zip::result::ZipError),

    #[from]
    Http(axum::http::StatusCode),

    #[from]
    ParseIntFromStr(String),

    #[from]
    Io(std::io::Error),

    #[from]
    Reqwest(reqwest::Error),

    #[from]
    Broadcast(tokio::sync::broadcast::error::SendError<()>),

    Synchronization,

    Url,
}
