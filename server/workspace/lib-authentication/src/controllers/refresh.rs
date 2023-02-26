use std::time::Duration;

use crate::controllers::login::{force, ForceLoginRequest, TokenPair};
use crate::{AuthToken, RefreshToken, Result, TokenRepoInterface};

pub struct Request<'a, A, R>
where
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
{
    pub refresh_token: &'a RefreshToken,
    pub auth_token_repo: &'a A,
    pub refresh_token_repo: &'a R,
    pub auth_token_ttl: Option<&'a Duration>,
    pub refresh_token_ttl: Option<&'a Duration>,
}

pub async fn refresh<A, R>(
    Request {
        refresh_token,
        auth_token_repo,
        refresh_token_repo,
        auth_token_ttl,
        refresh_token_ttl,
    }: Request<'_, A, R>,
) -> Result<Option<TokenPair>>
where
    A: TokenRepoInterface<AuthToken>,
    R: TokenRepoInterface<RefreshToken>,
{
    let user_id = refresh_token_repo.get(refresh_token).await?;

    // Remove this refresh token from the repository.
    let tokens = auth_token_repo
        .get_by_tag("refresh-token", refresh_token.as_ref())
        .await?;
    for token in tokens {
        auth_token_repo.delete(&token).await?;
    }

    // Remove this refresh token from the repository.
    refresh_token_repo.delete(refresh_token).await?;

    // Generate a new auth token and refresh token.
    let token_pair = force(ForceLoginRequest {
        auth_token_repo,
        refresh_token_repo,
        user_id: &user_id,
        auth_token_ttl,
        refresh_token_ttl,
    })
    .await?;

    Ok(Some(token_pair))
}
