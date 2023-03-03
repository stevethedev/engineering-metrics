use crate::{AuthToken, RefreshToken, Result, TokenRepoInterface};

pub async fn logout(
    auth_token_repo: &impl TokenRepoInterface<AuthToken>,
    refresh_token_repo: &impl TokenRepoInterface<RefreshToken>,
    token: &AuthToken,
) -> Result<()> {
    let refresh_token = auth_token_repo
        .get_tag(token, "refresh-token")
        .await
        .ok()
        .map(RefreshToken::from);

    auth_token_repo.delete(token).await?;

    if let Some(refresh_token) = refresh_token {
        refresh_token_repo.delete(&refresh_token).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::TokenInterface;

    use super::*;

    #[tokio::test]
    async fn test_logout() {
        let auth_token_repo = crate::TokenRepo::memory();
        let refresh_token_repo = crate::TokenRepo::memory();
        let auth_token = AuthToken::generate(32).unwrap();
        let user_id = uuid::Uuid::new_v4();

        auth_token_repo
            .put(&auth_token, &user_id, &[], None)
            .await
            .unwrap();
        assert!(auth_token_repo.get(&auth_token).await.is_ok());

        logout(&auth_token_repo, &refresh_token_repo, &auth_token)
            .await
            .unwrap();
        assert!(auth_token_repo.get(&auth_token).await.is_err());
    }
}
