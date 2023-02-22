use std::time::Duration;

use async_trait::async_trait;

use crate::controllers::{
    login::login as login_controller, logout::logout, register::register as register_controller,
    whoami::whoami,
};
use crate::user_repo::User;
use crate::{LoginCredentials, RegisterCredentials, Result, Token, TokenRepo, UserRepo};

use super::Interface;

/// The core authentication provider.
#[derive(Clone)]
pub struct Core {
    token_repo: TokenRepo,
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
    pub fn new(token_repo: TokenRepo, user_repo: UserRepo) -> Self {
        Self {
            token_repo,
            user_repo,
        }
    }
}

#[async_trait]
impl Interface for Core {
    async fn register<'a>(&self, credentials: &RegisterCredentials<'a>) -> Result<()> {
        let user_id = register_controller(&self.user_repo, credentials).await?;
        log::info!("Registered user {} ({})", credentials.username, user_id);
        Ok(())
    }

    async fn login<'a>(
        &self,
        login: &LoginCredentials<'a>,
        ttl: Option<&Duration>,
    ) -> Result<Option<Token>> {
        let result = login_controller(&self.user_repo, &self.token_repo, login, ttl).await?;
        log::info!(
            "User {} attempted to log in and {}",
            login.username,
            if result.is_some() {
                "succeeded"
            } else {
                "failed"
            }
        );
        Ok(result)
    }

    async fn whoami(&self, token: &Token) -> Result<Option<User>> {
        whoami(&self.token_repo, &self.user_repo, token).await
    }

    async fn logout(&self, token: &Token) -> Result<()> {
        let user = self.whoami(token).await?;
        logout(&self.token_repo, token).await?;
        if let Some(user) = user {
            log::info!("User {} ({}) logged out", user.username, user.id);
        }
        Ok(())
    }
}
