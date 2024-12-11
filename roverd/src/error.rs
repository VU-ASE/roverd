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

    

    #[from]
    ConfigValidation(Vec<rovervalidate::error::Error>),

    // --- Roverd Generic ---
    Generic(String),



    // --- Rover info file /etc/rover ---
    RoverInfoFileNotFound,

    // --- Config File /etc/roverd/rover.yaml ---
    CouldNotCreateConfigFile,
    CouldNotWriteToConfigFile,


    // --- Source Errors ---
    SourceAlreadyExists,
    SourceNotFound,

    // --- Service Errors ---
    ServiceValidation,
    ServiceNotFound,
    ServiceAlreadyExists,
    ServiceParseIncorrect,
    ServiceDownloadFailed,
    ServiceMissingUrl,

    // --- Validation ----
    EnabledPathInvalid,
    EnabledPathNotFound,

    // ---
    MissingUrl,

    PathConversion,

    StringToFqServiceConversion,

    PipielineValidation,

    #[from]
    Serialization(serde_yaml::Error),

    #[from]
    OsString(std::ffi::OsString),

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

    Unimplemented, // Todo this should be removed for prod!
}
