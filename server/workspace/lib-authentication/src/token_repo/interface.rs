use crate::Token;
use async_trait::async_trait;
use std::time::Duration;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for the token repository.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("token not found")]
    TokenNotFound,

    #[error("token expired")]
    TokenExpired,

    #[error("token invalid")]
    TokenInvalid,

    #[error("token repo error: {0}")]
    TokenRepoError(String),
}

/// Provides the interface for a token repository.
#[async_trait]
pub trait Interface: Send + Sync {
    /// Put a user ID into the token repository and return the generated token.
    ///
    /// # Parameters
    ///
    /// - `token`: The token to put into the token repository.
    /// - `user_id`: The user ID to put into the token repository.
    /// - `ttl`: The time to live of the token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be generated.
    async fn put(&self, token: &Token, user_id: &Uuid, ttl: Option<&Duration>) -> Result<()>;

    /// Get the user ID from the token repository.
    ///
    /// # Parameters
    ///
    /// - `token`: The token to get from the token repository.
    ///
    /// # Returns
    ///
    /// The user ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be found.
    async fn get(&self, token: &Token) -> Result<Uuid>;

    /// Delete the token from the token repository.
    ///
    /// # Parameters
    ///
    /// - `token`: The token to delete from the token repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be deleted.
    async fn delete(&self, token: &Token) -> Result<()>;
}