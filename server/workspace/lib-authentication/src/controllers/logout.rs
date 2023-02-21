use crate::{Result, Token, TokenRepoInterface};

pub async fn logout(token_repo: &impl TokenRepoInterface, token: &Token) -> Result<()> {
    token_repo.delete(token).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logout() {
        let token_repo = crate::TokenRepo::memory();
        let token = Token::generate(32).unwrap();
        let user_id = uuid::Uuid::new_v4();

        let _ = Box::pin(async move {
            token_repo.put(&token, &user_id, None).await.unwrap();
            assert!(token_repo.get(&token).await.is_ok());

            logout(&token_repo, &token).await.unwrap();
            assert!(token_repo.get(&token).await.is_err());
        });
    }
}
