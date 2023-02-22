use lib_authentication::{ProviderInterface, RegisterCredentials, Result};
use lib_json_schema::schema::auth::RegisterRequest;

/// Registers a new user.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `register_request` - The register request.
///
/// # Errors
///
/// Returns an error if the user could not be registered.
pub async fn register(
    provider: &impl ProviderInterface,
    register_request: &RegisterRequest,
) -> Result<()> {
    let credentials = RegisterCredentials {
        username: &register_request.username,
        password: &register_request.password,
    };
    provider.register(&credentials).await?;
    Ok(())
}
