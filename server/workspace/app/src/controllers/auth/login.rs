use std::time::{Duration, UNIX_EPOCH};

use lib_authentication::{LoginCredentials, ProviderInterface, TokenPair};
use lib_base64::Encode;
use lib_environment::EnvironmentVariable;
use lib_json_schema::schema::auth::{LoginRequest, LoginResponseSuccess};

/// Controls the login process.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `login_request` - The login request.
///
/// # Returns
///
/// - `None` if the login failed.
/// - `Some` if the login succeeded.
pub async fn login(
    provider: &lib_authentication::Provider,
    login_request: &LoginRequest,
) -> Option<LoginResponseSuccess> {
    let auth_token_ttl = Duration::from_secs(lib_environment::AuthTokenTtl::get());
    let refresh_token_ttl = Duration::from_secs(lib_environment::RefreshTokenTtl::get());

    let now = std::time::SystemTime::now();
    let auth_token_expires = now + auth_token_ttl;
    let refresh_token_expires = now + refresh_token_ttl;

    let auth_token_expires = auth_token_expires
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0u32);
    let refresh_token_expires = refresh_token_expires
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0u32);

    let token = provider
        .login(
            &LoginCredentials {
                username: &login_request.username,
                password: &login_request.password,
            },
            Some(&auth_token_ttl),
            Some(&refresh_token_ttl),
        )
        .await;

    let TokenPair {
        auth_token,
        refresh_token,
    } = match token {
        Ok(Some(token)) => token,
        Ok(None) => {
            return None;
        }
        Err(err) => {
            log::error!("Error while logging in: {}", err);
            return None;
        }
    };

    let auth_token = match auth_token.encode() {
        Ok(auth_token) => auth_token,
        Err(err) => {
            log::error!("Error while encoding auth token: {}", err);
            return None;
        }
    };

    let refresh_token = match refresh_token.encode() {
        Ok(refresh_token) => refresh_token,
        Err(err) => {
            log::error!("Error while encoding refresh token: {}", err);
            return None;
        }
    };

    Some(LoginResponseSuccess {
        auth_token,
        auth_token_expires,
        refresh_token,
        refresh_token_expires,
    })
}
