use std::time::Duration;

use async_trait::async_trait;

use crate::controllers::login::TokenPair;
use crate::user_repo::User;
use crate::{AuthToken, LoginCredentials, RefreshToken, RegisterCredentials, Result};

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
        auth_token_ttl: Option<&Duration>,
        refresh_token_ttl: Option<&Duration>,
    ) -> Result<Option<TokenPair>>;

    /// Returns `None` if the refresh failed, `Some` if it succeeded.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be refreshed.
    async fn refresh(
        &self,
        refresh_token: &RefreshToken,
        auth_token_ttl: Option<&Duration>,
        refresh_token_ttl: Option<&Duration>,
    ) -> Result<Option<TokenPair>>;

    /// Returns the user associated with the given token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be checked.
    async fn whoami(&self, auth_token: &AuthToken) -> Result<Option<User>>;

    /// Logs out the user associated with the given token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be deleted.
    async fn logout(&self, auth_token: &AuthToken) -> Result<()>;
}
