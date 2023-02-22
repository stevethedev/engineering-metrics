use lib_authentication::{ProviderInterface, Result, Token};

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
pub async fn logout(provider: &impl ProviderInterface, token: &Token) -> Result<()> {
    provider.logout(token).await?;
    Ok(())
}
