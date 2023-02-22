use lib_authentication::{ProviderInterface, Token};
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
    bearer_token: Option<&Token>,
) -> Option<WhoamiResponse> {
    let bearer_token = bearer_token?;
    let user = provider.whoami(bearer_token).await.ok()??;

    Some(WhoamiResponse {
        id: user.id.to_string(),
        username: user.username,
    })
}
