#![allow(unused)]

#[derive(Debug)]
pub enum Error {
    RoverInfoFileIo(String, std::io::Error),
    RoverInfoFileFormat(String),

    RoverShadowFile(String, std::io::Error),
    RoverPassword(String),

    ConfigFileNotFound,
    ConfigValidation,

    ServiceValidation,

    PipielineValidation,

    SerializationError,

    Http(axum::http::StatusCode),

    ParseIntFromStr(String),
    Io(std::io::Error),
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }


// impl From<std::io::Error> for Error {
//     fn from(value: std::io::Error) -> Self {
//         Error::Io(value)
//     }
// }

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Error::SerializationError
    }
}

impl From<Vec<rovervalidate::error::Error>> for Error {
    fn from(value: Vec<rovervalidate::error::Error>) -> Self {
        Error::SerializationError
    }
}



