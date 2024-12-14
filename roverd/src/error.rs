#![allow(unused)]

use tracing::warn;

use derive_more::From;

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
    ConfigFileNotFound,
    CouldNotCreateConfigFile,
    CouldNotWriteToConfigFile,

    // --- Installation ---
    ServiceYamlNotInZip,

    // --- Source Errors ---
    SourceAlreadyExists,
    SourceNotFound,

    // --- Downlaod Errors ---
    RemoteServiceNotFound,

    // --- Service Errors ---
    ServiceValidation,
    ServiceNotFound,
    ServiceAlreadyExists,
    ServiceParseIncorrect,
    ServiceDownloadFailed,
    ServiceMissingUrl,
    ServiceUploadData,

    // --- Validation ----
    EnabledPathInvalid,
    EnabledPathNotFound,
    NoRunnableServices,

    // --- Build ---
    BuildLog(Vec<String>),

    IncorrectPayload,

    MissingUrl,

    PathConversion,

    StringToFqServiceConversion,

    // --- Pipeline ---
    PipelineValidation,
    PipelineIsEmpty,

    #[from]
    Serialization(serde_yaml::Error),

    #[from]
    JsonSerialization(serde_json::Error),

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

    #[from]
    Multipart(axum_extra::extract::Multipart),

    Synchronization,

    Url,

    Unimplemented, // Todo this should be removed for prod!
}
