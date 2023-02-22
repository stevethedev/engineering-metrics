use lib_authentication::{LoginCredentials, ProviderInterface};
use lib_base64::Encode;
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
    let token_ttl = None;

    let token = provider
        .login(
            &LoginCredentials {
                username: &login_request.username,
                password: &login_request.password,
            },
            token_ttl,
        )
        .await;

    let token = match token {
        Ok(Some(token)) => token,
        Ok(None) => {
            return None;
        }
        Err(err) => {
            log::error!("Error while logging in: {}", err);
            return None;
        }
    };

    let token = match token.encode() {
        Ok(token) => token,
        Err(err) => {
            log::error!("Error while encoding token: {}", err);
            return None;
        }
    };

    Some(LoginResponseSuccess { token })
}
