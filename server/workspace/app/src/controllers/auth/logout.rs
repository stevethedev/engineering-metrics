use lib_authentication::{AuthToken, ProviderInterface, Result};

/// Logs out the user.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `token` - The token.
///
/// # Errors
///
/// Returns an error if the token could not be deleted.
pub async fn logout(provider: &impl ProviderInterface, auth_token: &AuthToken) -> Result<()> {
    provider.logout(auth_token).await?;
    Ok(())
}
