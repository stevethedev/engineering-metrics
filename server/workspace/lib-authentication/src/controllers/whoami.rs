use crate::user_repo::User;
use crate::{AuthToken, Result, TokenRepoInterface, UserRepoInterface};

/// Reads the user from the token.
///
/// # Arguments
///
/// - `token_repo` - The token repository.
/// - `user_repo` - The user repository.
/// - `token` - The token.
///
/// # Returns
///
/// Returns the user if the token is valid, `None` otherwise.
pub async fn whoami(
    auth_token_repo: &impl TokenRepoInterface<AuthToken>,
    user_repo: &impl UserRepoInterface,
    token: &AuthToken,
) -> Result<Option<User>> {
    let user_id = auth_token_repo.get(token).await?;
    let user = user_repo.get(user_id).await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use crate::user_repo::CreateUser;
    use crate::{TokenInterface, TokenRepo, UserRepo};

    use super::*;

    #[tokio::test]
    async fn test_whoami() {
        let user_repo = UserRepo::memory();
        let token_repo = TokenRepo::memory();

        let user_id = user_repo
            .create(&CreateUser {
                username: "test",
                password: "test",
            })
            .await
            .unwrap();
        let token = AuthToken::generate(32).unwrap();
        token_repo.put(&token, &user_id, &[], None).await.unwrap();
        let user = whoami(&token_repo, &user_repo, &token).await.unwrap();
        assert!(user.is_some());
    }
}
