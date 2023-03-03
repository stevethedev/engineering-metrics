use aead::{
    rand_core::{CryptoRng, RngCore},
    OsRng,
};

use crate::{Error, Result};

/// Get a cryptographically secure random number generator.
pub fn crypto_rng() -> impl CryptoRng + RngCore {
    OsRng
}

/// Fill a byte slice with random bytes.
///
/// # Parameters
///
/// - `dest`: The byte slice to fill with random bytes.
///
/// # Errors
///
/// Returns an error if the random bytes could not be generated.
pub fn fill_bytes(dest: &mut [u8]) -> Result<()> {
    crypto_rng()
        .try_fill_bytes(dest)
        .map_err(|e| Error::Unspecified(format!("{e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_bytes() {
        let mut bytes = [0u8; 32];
        fill_bytes(&mut bytes).unwrap();
        assert_ne!(bytes, [0u8; 32]);
    }
}
