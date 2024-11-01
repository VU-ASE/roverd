#![allow(unused)]

#[derive(Debug)]
pub enum Error {
    RoverInfoFileIo(String, std::io::Error),
    RoverInfoFileFormat(String),

    ParseIntFromStr(String),
    Io(String),
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }
