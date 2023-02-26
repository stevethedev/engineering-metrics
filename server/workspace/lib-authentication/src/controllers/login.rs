use std::time::Duration;

use lib_environment::EnvironmentVariable;

use crate::{
    AuthToken, RefreshToken, Result, TokenInterface, TokenRepoInterface, UserId, UserRepoInterface,
};

/// Authentication credentials.
pub struct Credentials<'a> {
    /// The username.
    pub username: &'a str,

    /// The password.
    pub password: &'a str,
}

/// A pair of authentication and refresh tokens.
pub struct TokenPair {
    /// The authentication token.
    pub auth_token: AuthToken,

    /// The refresh token.
    pub refresh_token: RefreshToken,
}

pub struct Request<
    'a,
    U: UserRepoInterface,
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
> {
    pub user_repo: &'a U,
    pub auth_token_repo: &'a A,
    pub refresh_token_repo: &'a R,
    pub login_credentials: &'a Credentials<'a>,
    pub auth_token_ttl: Option<&'a Duration>,
    pub refresh_token_ttl: Option<&'a Duration>,
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
pub async fn login<U, A, R>(
    Request {
        user_repo,
        auth_token_repo,
        refresh_token_repo,
        login_credentials,
        auth_token_ttl,
        refresh_token_ttl,
    }: Request<'_, U, A, R>,
) -> Result<Option<TokenPair>>
where
    U: UserRepoInterface,
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
{
    let user = user_repo
        .check_password(login_credentials.username, login_credentials.password)
        .await?;

    let Some(user) = user else { return Ok(None) };

    let user_id = &user.id;

    let token_pair = force(ForceLoginRequest {
        auth_token_repo,
        refresh_token_repo,
        user_id,
        auth_token_ttl,
        refresh_token_ttl,
    })
    .await?;

    Ok(Some(token_pair))
}

pub struct ForceLoginRequest<'a, A, R>
where
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
{
    pub auth_token_repo: &'a A,
    pub refresh_token_repo: &'a R,
    pub user_id: &'a UserId,
    pub auth_token_ttl: Option<&'a Duration>,
    pub refresh_token_ttl: Option<&'a Duration>,
}

/// Generates a new token for the given user.
///
/// # Arguments
///
/// - `token_repo` - The token repository.
/// - `user_id` - The user id.
/// - `ttl` - The time to live of the token.
///
/// # Returns
///
/// Returns the generated token.
///
/// # Errors
///
/// Returns an error if the token could not be generated.
pub async fn force<A, R>(
    ForceLoginRequest {
        auth_token_repo,
        refresh_token_repo,
        user_id,
        auth_token_ttl,
        refresh_token_ttl,
    }: ForceLoginRequest<'_, A, R>,
) -> Result<TokenPair>
where
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
{
    let auth_token = AuthToken::generate(lib_environment::AuthTokenSize::get())?;
    let refresh_token = RefreshToken::generate(lib_environment::RefreshTokenSize::get())?;

    auth_token_repo
        .put(&auth_token, user_id, auth_token_ttl)
        .await?;

    auth_token_repo
        .put_tag(&auth_token, "refresh-token", refresh_token.as_ref())
        .await?;

    refresh_token_repo
        .put(&refresh_token, user_id, refresh_token_ttl)
        .await?;

    refresh_token_repo
        .put_tag(&refresh_token, "auth-token", auth_token.as_ref())
        .await?;

    Ok(TokenPair {
        auth_token,
        refresh_token,
    })
}

#[cfg(test)]
mod tests {
    use crate::user_repo::CreateUser;
    use crate::{TokenRepo, UserRepo};

    use super::*;

    #[tokio::test]
    async fn test_login() {
        let user_repo = UserRepo::memory();
        let auth_token_repo = TokenRepo::memory();
        let refresh_token_repo = TokenRepo::memory();

        let login_credentials = Credentials {
            username: "test",
            password: "test",
        };

        user_repo
            .create(&CreateUser {
                username: "test",
                password: "test",
            })
            .await
            .unwrap();
        let token = login(Request {
            user_repo: &user_repo,
            auth_token_repo: &auth_token_repo,
            refresh_token_repo: &refresh_token_repo,
            login_credentials: &login_credentials,
            auth_token_ttl: None,
            refresh_token_ttl: None,
        })
        .await
        .unwrap();
        assert!(token.is_some());
    }

    #[tokio::test]
    async fn test_login_invalid() {
        let user_repo = UserRepo::memory();
        let auth_token_repo = TokenRepo::memory();
        let refresh_token_repo = TokenRepo::memory();

        let login_credentials = Credentials {
            username: "admin",
            password: "invalid",
        };

        let token = login(Request {
            user_repo: &user_repo,
            auth_token_repo: &auth_token_repo,
            refresh_token_repo: &refresh_token_repo,
            login_credentials: &login_credentials,
            auth_token_ttl: None,
            refresh_token_ttl: None,
        })
        .await
        .unwrap();
        assert!(token.is_none());
    }
}
