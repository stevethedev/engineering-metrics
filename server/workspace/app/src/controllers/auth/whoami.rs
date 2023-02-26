use lib_authentication::{AuthToken, ProviderInterface};
use lib_json_schema::schema::auth::WhoamiResponse;

/// Returns the user information for the given token.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `bearer_token` - The bearer token.
///
/// # Returns
///
/// - `None` if the token is invalid.
/// - `Some` if the token is valid.
pub async fn whoami(
    provider: &impl ProviderInterface,
    auth_token: Option<&AuthToken>,
) -> Option<WhoamiResponse> {
    let user = provider.whoami(auth_token?).await.ok()??;

    Some(WhoamiResponse {
        id: user.id.to_string(),
        username: user.username,
    })
}
