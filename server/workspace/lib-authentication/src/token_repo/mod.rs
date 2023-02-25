use std::time::Duration;

use async_trait::async_trait;
use uuid::Uuid;

pub use interface::{Error, Interface, Result};
pub use memory::TokenRepo as Memory;

use crate::{TokenInterface, TokenRepoInterface};

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
}

#[async_trait]
impl<Token: TokenInterface> TokenRepoInterface<Token> for TokenRepo<Token> {
    async fn put(&self, token: &Token, user_id: &Uuid, ttl: Option<&Duration>) -> Result<()> {
        self.repo.put(token, user_id, ttl).await
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

    async fn get_by_tag(&self, tag: &str, value: &[u8]) -> Result<Vec<Token>> {
        self.repo.get_by_tag(tag, value).await
    }

    async fn delete_by_tag(&self, tag: &str, value: &[u8]) -> Result<()> {
        self.repo.delete_by_tag(tag, value).await
    }
}
