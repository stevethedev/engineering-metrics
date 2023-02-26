use std::time::{Duration, UNIX_EPOCH};

use lib_authentication::{ProviderInterface, RefreshToken, TokenPair};
use lib_base64::{decode, Encode};
use lib_environment::EnvironmentVariable;
use lib_json_schema::schema::auth::{RefreshRequest, RefreshResponseSuccess};

pub async fn refresh(
    provider: &impl ProviderInterface,
    refresh_request: &RefreshRequest,
) -> Option<RefreshResponseSuccess> {
    let refresh_token = decode(&refresh_request.refresh_token).ok()?;
    let refresh_token = RefreshToken::from(refresh_token);

    let auth_token_ttl = Duration::from_secs(lib_environment::AuthTokenTtl::get());
    let refresh_token_ttl = Duration::from_secs(lib_environment::RefreshTokenTtl::get());

    let now = std::time::SystemTime::now();
    let auth_token_expires = (now + auth_token_ttl)
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0u32);
    let refresh_token_expires = (now + refresh_token_ttl)
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0u32);

    let TokenPair {
        auth_token,
        refresh_token,
    } = provider
        .refresh(
            &refresh_token,
            Some(&auth_token_ttl),
            Some(&refresh_token_ttl),
        )
        .await
        .ok()??;

    let Ok(auth_token) = auth_token.encode() else { return None };
    let Ok(refresh_token) = refresh_token.encode() else { return None };

    Some(RefreshResponseSuccess {
        auth_token,
        auth_token_expires,
        refresh_token,
        refresh_token_expires,
    })
}
