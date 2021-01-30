
#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    RootDropped,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIo(e)
    }
}

// impl From<NoneError> for Error {
//     fn from(_: NoneError) -> Self {
//         Error::NoneError
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;
