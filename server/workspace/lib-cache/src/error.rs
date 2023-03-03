pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the cache.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Create pool error: {0}")]
    CreatePoolError(#[from] deadpool_redis::CreatePoolError),

    #[error("Pool error: {0}")]
    PoolError(#[from] deadpool_redis::PoolError),

    #[error("Invalid Expiry")]
    InvalidExpiry,
}
