pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Database query error: {0}")]
    Query(String),
}
