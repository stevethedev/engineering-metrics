use crate::token_repo::{Error, Interface, Result};
use crate::Token;
use async_trait::async_trait;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// A record within the token repository.
struct Record {
    user_id: Uuid,
    expiry: Option<SystemTime>,
}

/// An in-memory token repository.
#[derive(Default)]
pub struct TokenRepo {
    token_repo: std::sync::RwLock<std::collections::HashMap<Vec<u8>, Record>>,
}

#[async_trait]
impl Interface for TokenRepo {
    async fn put(&self, token: &Token, user_id: &Uuid, ttl: Option<&Duration>) -> Result<()> {
        let expiry = ttl.map(|ttl| SystemTime::now() + *ttl);
        let user_id = *user_id;
        let token = token.as_ref().to_owned();

        let mut token_repo = self
            .token_repo
            .write()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

        token_repo.insert(token, Record { user_id, expiry });
        Ok(())
    }

    async fn get(&self, token: &Token) -> Result<Uuid> {
        let token = token.as_ref();
        let token_repo = self
            .token_repo
            .read()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

        let record = token_repo.get(token).ok_or(Error::TokenNotFound)?;

        if let Some(expiry) = record.expiry {
            let now = SystemTime::now();
            if now > expiry {
                return Err(Error::TokenExpired);
            }
        }
        Ok(record.user_id)
    }

    async fn delete(&self, token: &Token) -> Result<()> {
        let mut token_repo = self
            .token_repo
            .write()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;
        token_repo.remove(token.as_ref());
        Ok(())
    }
}
