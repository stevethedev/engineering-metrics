pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unspecified cryptography error")]
    Unspecified(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid hash")]
    InvalidHash,

    #[error("invalid key length: {0}")]
    InvalidKeyLength(usize),

    #[error("invalid password hash: {0}")]
    InvalidPasswordHash(String),
}
