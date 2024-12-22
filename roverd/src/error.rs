#![allow(unused)]

use derive_more::From;

/// A central definition of all possible errors in roverd. The point of this organization is
/// to be able to explain at a high level all possible error situations from just this file.
/// For this reason, we avoid generic errors where the caller specifies further context
/// about the error, it should all already be clear just based on the enum variant.
#[derive(Debug, From)]
pub enum Error {
    // --- Rover info file ---
    RoverFileNotFound,
    RoverFileFormat,

    // --- Configuration file ---
    ConfigFileIO,
    EnabledPathInvalid,

    // --- Service Errors ---
    ServiceNotFound,
    ServiceAlreadyExists,
    ServiceDownloadFailed,
    ServiceUploadBadPayload,

    // --- Installation ---
    ServiceYamlNotFoundInDownload,

    // --- Build ---
    BuildLog(Vec<String>),
    BuildCommandFailed,
    BuildCommandMissing,

    // --- Runtime ---
    NoLogsFound,
    NoRunningServices,
    ProcessNotFound,
    ParsingRunCommand,
    StringToFqConversion,

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

    #[from]
    Validation(Vec<rovervalidate::error::Error>),

    Synchronization,

    Url,

    Unimplemented, // Todo this should be removed for prod!
}
