pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unspecified authentication error")]
    CryptoError(#[from] lib_crypto::Error),
}
