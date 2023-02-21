use super::Interface;
use crate::controllers::login::login as login_controller;
use crate::controllers::logout::logout;
use crate::controllers::whoami::whoami;
use crate::user_repo::User;
use crate::{Credentials, Result, Token, TokenRepo, UserRepo};
use async_trait::async_trait;
use std::time::Duration;

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
    async fn login<'a>(
        &self,
        login: &Credentials<'a>,
        ttl: Option<&Duration>,
    ) -> Result<Option<Token>> {
        login_controller(&self.user_repo, &self.token_repo, login, ttl).await
    }

    async fn whoami(&self, token: &Token) -> Result<Option<User>> {
        whoami(&self.token_repo, &self.user_repo, token).await
    }

    async fn logout(&self, token: &Token) -> Result<()> {
        logout(&self.token_repo, token).await
    }
}
