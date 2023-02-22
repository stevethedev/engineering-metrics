use std::time::Duration;

use async_trait::async_trait;

use crate::user_repo::User;
use crate::{LoginCredentials, RegisterCredentials, Result, Token};

#[async_trait]
pub trait Interface {
    /// Creates a new user with the given credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be created.
    async fn register<'a>(&self, credentials: &RegisterCredentials<'a>) -> Result<()>;

    /// Returns `None` if the login failed, `Some` if it succeeded.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be generated.
    async fn login<'a>(
        &self,
        login: &LoginCredentials<'a>,
        ttl: Option<&Duration>,
    ) -> Result<Option<Token>>;

    /// Returns the user associated with the given token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be checked.
    async fn whoami(&self, token: &Token) -> Result<Option<User>>;

    /// Logs out the user associated with the given token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be deleted.
    async fn logout(&self, token: &Token) -> Result<()>;
}
