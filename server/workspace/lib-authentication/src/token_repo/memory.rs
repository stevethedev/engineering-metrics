use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

use async_trait::async_trait;
use uuid::Uuid;

use crate::token_repo::{Error, Result, TokenInterface};
use crate::TokenRepoInterface;

/// A record within the token repository.
struct Record {
    user_id: Uuid,
    expiry: Option<SystemTime>,
}

struct TagRecord {
    tag: String,
    value: Vec<u8>,
    token: Vec<u8>,
}

/// An in-memory token repository.
#[derive(Default)]
pub struct TokenRepo {
    token_repo: RwLock<HashMap<Vec<u8>, Record>>,
    tags_repo: RwLock<Vec<TagRecord>>,
}

#[async_trait]
impl<Token: TokenInterface> TokenRepoInterface<Token> for TokenRepo {
    async fn put(
        &self,
        token: &Token,
        user_id: &Uuid,
        tags: &[(&str, &[u8])],
        ttl: Option<&Duration>,
    ) -> Result<()> {
        let expiry = ttl.map(|ttl| SystemTime::now() + *ttl);
        let user_id = *user_id;
        let token = token.as_ref().to_owned();

        {
            let mut token_repo = self
                .token_repo
                .write()
                .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

            token_repo.insert(token.clone(), Record { user_id, expiry });
        }

        {
            let mut tags_repo = self
                .tags_repo
                .write()
                .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

            for &(tag, value) in tags {
                tags_repo.push(TagRecord {
                    tag: tag.to_owned(),
                    value: value.to_vec(),
                    token: token.clone(),
                });
            }
        }

        Ok(())
    }

    async fn get(&self, token: &Token) -> Result<Uuid> {
        let (user_id, expiry) = {
            let token_repo = self
                .token_repo
                .read()
                .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

            let record = token_repo.get(token.as_ref()).ok_or(Error::TokenNotFound)?;

            let expiry = record.expiry;
            let user_id = record.user_id;

            (user_id, expiry)
        };

        if let Some(expiry) = expiry {
            let now = SystemTime::now();
            if now > expiry {
                self.delete(token).await?;
                return Err(Error::TokenExpired);
            }
        }
        Ok(user_id)
    }

    async fn delete(&self, token: &Token) -> Result<()> {
        let mut token_repo = self
            .token_repo
            .write()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

        token_repo.remove(token.as_ref());

        let mut tags_repo = self
            .tags_repo
            .write()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;

        tags_repo.retain(|t| t.token != token.as_ref());

        Ok(())
    }

    async fn put_tag(&self, token: &Token, tag: &str, value: &[u8]) -> Result<()> {
        let mut tags_repo = self
            .tags_repo
            .write()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;
        tags_repo.push(TagRecord {
            tag: tag.to_owned(),
            value: value.to_vec(),
            token: token.as_ref().to_owned(),
        });
        Ok(())
    }

    async fn get_tag(&self, token: &Token, tag: &str) -> Result<Vec<u8>> {
        let tags_repo = self
            .tags_repo
            .read()
            .map_err(|e| Error::TokenRepoError(format!("{e}")))?;
        let tag_record = tags_repo
            .iter()
            .find(|t| t.token == token.as_ref() && t.tag == tag)
            .ok_or(Error::TokenNotFound)?;
        Ok(tag_record.value.clone())
    }
}
