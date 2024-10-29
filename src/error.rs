// pub type Result<T> = core::result::Result<T, Error>;
// pub type Result<T, E> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[allow(unused)]
    StdIo(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::StdIo(format!("{:?}", e))
    }
}
