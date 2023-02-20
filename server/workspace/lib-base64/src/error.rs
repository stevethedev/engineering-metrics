#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Invalid length")]
    InvalidLength,

    #[error("Invalid encoding")]
    InvalidEncoding,
}

impl From<base64ct::InvalidLengthError> for Error {
    fn from(_: base64ct::InvalidLengthError) -> Self {
        Error::InvalidLength
    }
}

impl From<base64ct::InvalidEncodingError> for Error {
    fn from(_: base64ct::InvalidEncodingError) -> Self {
        Error::InvalidEncoding
    }
}

impl From<base64ct::Error> for Error {
    fn from(error: base64ct::Error) -> Self {
        match error {
            base64ct::Error::InvalidLength => Error::InvalidLength,
            base64ct::Error::InvalidEncoding => Error::InvalidEncoding,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
