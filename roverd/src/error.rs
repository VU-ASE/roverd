#![allow(unused)]

use tracing::warn;

#[derive(Debug)]
pub enum Error {
    RoverInfoFileIo(String, std::io::Error),
    RoverInfoFileFormat(String),

    RoverShadowFile(String, std::io::Error),
    RoverPassword(String),

    ConfigFileNotFound,
    ConfigValidation(String),

    Source(String),

    ServiceValidation,

    PipielineValidation,

    Serialization,

    Download,

    Http(axum::http::StatusCode),

    ParseIntFromStr(String),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Error::Serialization
    }
}

impl From<Vec<rovervalidate::error::Error>> for Error {
    // Not efficient, someday change, see: https://rust-lang.github.io/rust-clippy/master/index.html#format_collect
    #[allow(clippy::format_collect)]
    fn from(error_vec: Vec<rovervalidate::error::Error>) -> Self {
        let error_string: String = error_vec.iter().map(|s| format!("{s}\n")).collect();
        Error::ConfigValidation(format!("{}", error_string))
    }
}
