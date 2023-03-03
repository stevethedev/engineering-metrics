use std::time::Duration;

use async_trait::async_trait;
use uuid::Uuid;

pub use cache::TokenRepo as Cache;
pub use interface::{Error, Interface, Result};
pub use memory::TokenRepo as Memory;

use crate::{TokenInterface, TokenRepoInterface};

mod cache;
mod interface;
mod memory;

/// The master token repository.
///
/// # Examples
///
/// ```
/// use lib_authentication::TokenRepo;
#[derive(Clone)]
pub struct TokenRepo<Token: TokenInterface> {
    repo: std::sync::Arc<Box<dyn TokenRepoInterface<Token>>>,
}

impl<Token: TokenInterface> TokenRepo<Token> {
    #[must_use]
    pub fn memory() -> Self {
        let repo = Memory::default();
        Self {
            repo: std::sync::Arc::new(Box::new(repo)),
        }
    }

    #[must_use]
    pub fn cache(controller: lib_cache::Controller, prefix: String) -> Self {
        let repo = Cache::new(controller, prefix);
        Self {
            repo: std::sync::Arc::new(Box::new(repo)),
        }
    }
}

#[async_trait]
impl<Token: TokenInterface> TokenRepoInterface<Token> for TokenRepo<Token> {
    async fn put(
        &self,
        token: &Token,
        user_id: &Uuid,
        tags: &[(&str, &[u8])],
        ttl: Option<&Duration>,
    ) -> Result<()> {
        self.repo.put(token, user_id, tags, ttl).await
    }

    async fn get(&self, token: &Token) -> Result<Uuid> {
        self.repo.get(token).await
    }

    async fn delete(&self, token: &Token) -> Result<()> {
        self.repo.delete(token).await
    }

    async fn put_tag(&self, token: &Token, tag: &str, value: &[u8]) -> Result<()> {
        self.repo.put_tag(token, tag, value).await
    }

    async fn get_tag(&self, token: &Token, tag: &str) -> Result<Vec<u8>> {
        self.repo.get_tag(token, tag).await
    }
}
