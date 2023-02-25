use std::time::Duration;

use async_trait::async_trait;

use crate::controllers::login::{Request as LoginRequest, TokenPair};
use crate::controllers::refresh::{refresh, Request as RefreshRequest};
use crate::controllers::{
    login::login as login_controller, logout::logout, register::register as register_controller,
    whoami::whoami,
};
use crate::user_repo::User;
use crate::{
    AuthToken, LoginCredentials, ProviderInterface, RefreshToken, RegisterCredentials, Result,
    TokenRepo, UserRepo,
};

/// The core authentication provider.
#[derive(Clone)]
pub struct Core {
    auth_token_repo: TokenRepo<AuthToken>,
    refresh_token_repo: TokenRepo<RefreshToken>,
    user_repo: UserRepo,
}

impl Core {
    /// Creates a new core authentication provider.
    ///
    /// # Arguments
    ///
    /// - `token_repo` - The token repository.
    /// - `user_repo` - The user repository.
    ///
    /// # Returns
    ///
    /// The core authentication provider.
    #[must_use]
    pub fn new(
        auth_token_repo: TokenRepo<AuthToken>,
        refresh_token_repo: TokenRepo<RefreshToken>,
        user_repo: UserRepo,
    ) -> Self {
        Self {
            auth_token_repo,
            refresh_token_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl ProviderInterface for Core {
    async fn register<'a>(&self, credentials: &RegisterCredentials<'a>) -> Result<()> {
        let user_id = register_controller(&self.user_repo, credentials).await?;
        log::info!("Registered user {} ({})", credentials.username, user_id);
        Ok(())
    }

    async fn login<'a>(
        &self,
        login_credentials: &LoginCredentials<'a>,
        auth_token_ttl: Option<&Duration>,
        refresh_token_ttl: Option<&Duration>,
    ) -> Result<Option<TokenPair>> {
        let result = login_controller(LoginRequest {
            user_repo: &self.user_repo,
            auth_token_repo: &self.auth_token_repo,
            refresh_token_repo: &self.refresh_token_repo,
            login_credentials,
            auth_token_ttl,
            refresh_token_ttl,
        })
        .await?;
        log::info!(
            "User {} attempted to log in and {}",
            login_credentials.username,
            if result.is_some() {
                "succeeded"
            } else {
                "failed"
            }
        );
        Ok(result)
    }

    async fn refresh(
        &self,
        refresh_token: &RefreshToken,
        auth_token_ttl: Option<&Duration>,
        refresh_token_ttl: Option<&Duration>,
    ) -> Result<Option<TokenPair>> {
        refresh(RefreshRequest {
            auth_token_repo: &self.auth_token_repo,
            refresh_token_repo: &self.refresh_token_repo,
            refresh_token,
            auth_token_ttl,
            refresh_token_ttl,
        })
        .await
    }

    async fn whoami(&self, auth_token: &AuthToken) -> Result<Option<User>> {
        whoami(&self.auth_token_repo, &self.user_repo, auth_token).await
    }

    async fn logout(&self, auth_token: &AuthToken) -> Result<()> {
        let user = self.whoami(auth_token).await?;
        logout(&self.auth_token_repo, &self.refresh_token_repo, auth_token).await?;
        if let Some(user) = user {
            log::info!("User {} ({}) logged out", user.username, user.id);
        }
        Ok(())
    }
}
