use argon2::{
    password_hash::{
        PasswordHash as ArgonPasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

use crate::{Error, Result};

/// Hashes a password using Argon2id.
///
/// The returned string is a valid password hash that can be stored in the database.
///
/// # Arguments
///
/// * `password` - The password to hash.
///
/// # Errors
///
/// Returns an error if the password could not be hashed.
pub fn hash_password(password: &[u8]) -> Result<String> {
    let salt = {
        let mut rng = crate::rand::crypto_rng();
        SaltString::generate(&mut rng)
    };

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| Error::InvalidPasswordHash(e.to_string()))?
        .to_string();
    Ok(hash)
}

/// Verifies a password against a password hash.
///
/// # Arguments
///
/// * `password` - The password to verify.
/// * `password_hash` - The password hash to verify against.
///
/// # Errors
///
/// Returns an error if the password could not be verified.
pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool> {
    let parsed_hash = ArgonPasswordHash::new(password_hash)
        .map_err(|e| Error::InvalidPasswordHash(e.to_string()))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password, &parsed_hash).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() {
        let password = "hunter42"; // Bad password; don't actually use!
        let password_hash = hash_password(password.as_bytes()).unwrap();
        assert!(verify_password(password.as_bytes(), &password_hash).unwrap());
    }

    #[test]
    fn test_invalid_password() {
        let password = "hunter42"; // Bad password; don't actually use!
        let password_hash = hash_password(password.as_bytes()).unwrap();
        assert!(!verify_password(b"hunter43", &password_hash).unwrap());
    }

    #[test]
    fn unique_hashes() {
        let password = "hunter42"; // Bad password; don't actually use!
        let password_hash = hash_password(password.as_bytes()).unwrap();
        let password_hash2 = hash_password(password.as_bytes()).unwrap();

        assert_ne!(password_hash, password_hash2);
    }
}
