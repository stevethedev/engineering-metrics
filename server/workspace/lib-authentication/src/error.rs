use crate::{TokenRepoError, UserRepoError};

/// The result type for the authentication library.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the authentication library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unspecified authentication error")]
    CryptoError(#[from] lib_crypto::Error),

    #[error("unspecified authentication error")]
    TokenRepoError(#[from] TokenRepoError),

    #[error("unspecified authentication error")]
    UserRepoError(#[from] UserRepoError),
}
