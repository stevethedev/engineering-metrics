mod interface;
mod memory;

use crate::Token;
use async_trait::async_trait;
pub use interface::{Error, Interface, Result};
use std::time::Duration;
use uuid::Uuid;

pub use memory::TokenRepo as Memory;

/// The master token repository.
///
/// # Examples
///
/// ```
/// use lib_authentication::TokenRepo;
#[derive(Clone)]
pub struct TokenRepo {
    repo: std::sync::Arc<Box<dyn Interface>>,
}

impl TokenRepo {
    #[must_use]
    pub fn memory() -> Self {
        let repo = Memory::default();
        Self {
            repo: std::sync::Arc::new(Box::new(repo)),
        }
    }
}

#[async_trait]
impl Interface for TokenRepo {
    async fn put(&self, token: &Token, user_id: &Uuid, ttl: Option<&Duration>) -> Result<()> {
        self.repo.put(token, user_id, ttl).await
    }

    async fn get(&self, token: &Token) -> Result<Uuid> {
        self.repo.get(token).await
    }

    async fn delete(&self, token: &Token) -> Result<()> {
        self.repo.delete(token).await
    }
}
