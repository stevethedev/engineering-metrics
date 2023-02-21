use crate::token::Token;
use crate::{Result, TokenRepoInterface, UserRepoInterface};
use lib_environment::EnvironmentVariable;
use std::time::Duration;

/// Authentication credentials.
pub struct Credentials<'a> {
    /// The username.
    pub username: &'a str,

    /// The password.
    pub password: &'a str,
}

/// Returns `None` if the login failed, `Some` if it succeeded.
///
/// # Arguments
///
/// - `user_repo` - The user repository.
/// - `token_repo` - The token repository.
/// - `login` - The login data.
/// - `ttl` - The time to live of the token.
///
/// # Returns
///
/// Returns `None` if the login failed, `Some` if it succeeded.
///
/// # Errors
///
/// Returns an error if the token could not be generated.
pub async fn login<'a>(
    user_repo: &impl UserRepoInterface,
    token_repo: &impl TokenRepoInterface,
    login: &Credentials<'a>,
    ttl: Option<&Duration>,
) -> Result<Option<Token>> {
    let user = user_repo
        .check_password(login.username, login.password)
        .await?;

    if user.is_none() {
        return Ok(None);
    }

    let token_size = lib_environment::AuthTokenSize::get();
    let ttl = ttl.copied().or_else(|| {
        let env_ttl = lib_environment::AuthTokenTtl::get();
        let duration = Duration::from_secs(env_ttl);
        Some(duration)
    });

    let token = Token::generate(token_size)?;
    token_repo
        .put(&token, &user.unwrap().id, ttl.as_ref())
        .await?;

    Ok(Some(token))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_repo::CreateUser;
    use crate::{TokenRepo, UserRepo};

    #[test]
    fn test_login() {
        let user_repo = UserRepo::memory();
        let token_repo = TokenRepo::memory();

        let creds = Credentials {
            username: "test",
            password: "test",
        };

        let _ = Box::pin(async move {
            user_repo
                .create(&CreateUser {
                    username: "test",
                    password: "test",
                })
                .await
                .unwrap();
            let token = login(&user_repo, &token_repo, &creds, None).await.unwrap();
            assert!(token.is_some());
        });
    }

    #[test]
    fn test_login_invalid() {
        let user_repo = UserRepo::memory();
        let token_repo = TokenRepo::memory();

        let creds = Credentials {
            username: "admin",
            password: "invalid",
        };

        let _ = Box::pin(async move {
            let token = login(&user_repo, &token_repo, &creds, None).await.unwrap();
            assert!(token.is_none());
        });
    }
}
