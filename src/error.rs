#![allow(unused)]

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RoverInfoFile(String),
    ParseIntFromStr(String),
    Io(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseIntFromStr(e.to_string())
    }
}
