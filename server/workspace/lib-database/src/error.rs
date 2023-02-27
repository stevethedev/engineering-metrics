pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Database query error: {0}")]
    Query(String),

    #[error("Database migration error: {0}")]
    Migration(String),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
}
