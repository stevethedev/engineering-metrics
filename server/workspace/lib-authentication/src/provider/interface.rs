use crate::user_repo::User;
use crate::{Credentials, Result, Token};
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait Interface {
    /// Returns `None` if the login failed, `Some` if it succeeded.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be generated.
    async fn login<'a>(
        &self,
        login: &Credentials<'a>,
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
