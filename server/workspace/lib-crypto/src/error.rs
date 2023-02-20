pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unspecified cryptography error")]
    Unspecified,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
