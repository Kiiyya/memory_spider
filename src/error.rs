
#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    RootOrParentInvalid,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIo(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
