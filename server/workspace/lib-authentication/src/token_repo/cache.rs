use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use uuid::Uuid;

use lib_base64::Encode;

use crate::{TokenInterface, TokenRepoError, TokenRepoInterface, TokenRepoResult};

/// A token repository that uses a cache.
pub struct TokenRepo {
    prefix: String,
    controller: lib_cache::Controller,
}

impl TokenRepo {
    /// Create a new token repository.
    ///
    /// # Arguments
    ///
    /// * `controller` - The cache controller.
    /// * `prefix` - The prefix to use for keys.
    pub fn new(controller: lib_cache::Controller, prefix: String) -> Self {
        Self { prefix, controller }
    }

    /// Get formatted key for retrieving a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to format.
    fn get_key(&self, key: &str) -> String {
        format!("{}:{}", self.prefix, key)
    }

    /// Get a connection to the cache.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection could not be established.
    async fn get_connection(&self) -> TokenRepoResult<lib_cache::Connection> {
        self.controller
            .connect()
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))
    }
}

/// The key used to store the user ID.
const USER_ID: &str = "uid";

#[async_trait]
impl<Token: TokenInterface> TokenRepoInterface<Token> for TokenRepo {
    async fn put(
        &self,
        token: &Token,
        user_id: &Uuid,
        tags: &[(&str, &[u8])],
        ttl: Option<&Duration>,
    ) -> TokenRepoResult<()> {
        let user_id = user_id.to_string();
        let token = token.encode().map_err(|_| TokenRepoError::TokenInvalid)?;
        let key = self.get_key(&token);

        let mut c = self.get_connection().await?;

        c.hset(&key, USER_ID, user_id)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(format!("{e}: {key}")))?;

        for &(tag, value) in tags {
            c.hset(&key, tag, value)
                .await
                .map_err(|e| TokenRepoError::TokenRepoError(format!("{e}: {key}")))?;
        }

        if let Some(expiry) = ttl {
            let _ = c
                .expire(&key, expiry)
                .await
                .map_err(|e| TokenRepoError::TokenRepoError(format!("{e}: {key}")))?;
        }

        Ok(())
    }

    async fn get(&self, token: &Token) -> crate::token_repo::Result<Uuid> {
        let token = token.encode().map_err(|_| TokenRepoError::TokenInvalid)?;
        let key = self.get_key(&token);

        let mut c = self.get_connection().await?;

        let uuid: String = c
            .hget(&key, USER_ID)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))?
            .ok_or(TokenRepoError::TokenNotFound)?;

        let uuid = Uuid::from_str(&uuid).map_err(|_| TokenRepoError::TokenInvalid)?;
        Ok(uuid)
    }

    async fn delete(&self, token: &Token) -> crate::token_repo::Result<()> {
        let token = token.encode().map_err(|_| TokenRepoError::TokenInvalid)?;
        let key = self.get_key(&token);

        let mut c = self.get_connection().await?;

        if !c
            .exists(&key)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))?
        {
            return Err(TokenRepoError::TokenNotFound);
        }

        c.delete(&key)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))?;

        Ok(())
    }

    async fn get_tag(&self, token: &Token, tag: &str) -> crate::token_repo::Result<Vec<u8>> {
        let token = token.encode().map_err(|_| TokenRepoError::TokenInvalid)?;
        let key = self.get_key(&token);

        let mut c = self.get_connection().await?;

        let value: Vec<u8> = c
            .hget(&key, tag)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))?
            .ok_or(TokenRepoError::TokenNotFound)?;

        Ok(value)
    }

    async fn put_tag(&self, token: &Token, tag: &str, value: &[u8]) -> TokenRepoResult<()> {
        let token = token.encode().map_err(|_| TokenRepoError::TokenInvalid)?;
        let key = self.get_key(&token);

        let mut c = self.get_connection().await?;

        c.hset(&key, tag, value)
            .await
            .map_err(|e| TokenRepoError::TokenRepoError(e.to_string()))?;

        Ok(())
    }
}
