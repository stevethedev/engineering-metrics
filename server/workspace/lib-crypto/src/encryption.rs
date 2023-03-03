use aead::{Aead, AeadCore, Nonce};
use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

use crate::rand::crypto_rng;
use crate::{Error, Result, Sha256Hash};

const NONCE_LEN: usize = 24;
// const TAG_LEN: usize = 16;
const KEY_LEN: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key([u8; KEY_LEN]);

impl Key {
    /// Create a new encryption key by hashing the given source.
    ///
    /// # Arguments
    ///
    /// * `source` - source to hash
    #[must_use]
    pub fn hash_from(source: &[u8]) -> Self {
        let hash = Sha256Hash::new(source);
        Self(hash.into())
    }

    /// Generate a new encryption key.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unspecified`] if key generation fails.
    pub fn generate() -> Result<Self> {
        let mut key = [0u8; KEY_LEN];
        let x_chacha_key = XChaCha20Poly1305::generate_key(&mut crypto_rng());
        let slice = x_chacha_key
            .get(0..KEY_LEN)
            .ok_or_else(|| Error::Unspecified("failed to generate encryption key".to_string()))?;

        key.copy_from_slice(slice);

        Ok(Self(key))
    }

    /// Encrypt plaintext using `XChaCha20Poly1305`. The nonce is generated
    /// automatically and prepended to the ciphertext.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-bit key
    /// * `plaintext` - plaintext to encrypt
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unspecified`] if encryption fails.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new(&self.0.into());
        let nonce = XChaCha20Poly1305::generate_nonce(&mut crypto_rng()); // 192-bits; unique per message

        let result = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| Error::Unspecified(format!("{e:?}")))?;

        let result = {
            let mut tmp = nonce.to_vec();
            tmp.extend(result);
            tmp
        };

        Ok(result)
    }

    /// Decrypt ciphertext using `XChaCha20Poly1305`. The nonce is extracted
    /// from the ciphertext.
    ///
    /// # Arguments
    ///
    /// * `key` - 256-bit key
    /// * `ciphertext` - ciphertext to decrypt
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unspecified`] if decryption fails.
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let nonce = ciphertext
            .get(..NONCE_LEN)
            .ok_or_else(|| Error::Unspecified("ciphertext is too short".to_string()))?;
        let ciphertext = ciphertext
            .get(NONCE_LEN..)
            .ok_or_else(|| Error::Unspecified("ciphertext is too short".to_string()))?;

        let cipher = XChaCha20Poly1305::new(&self.0.into());
        let nonce = Nonce::<XChaCha20Poly1305>::from_slice(nonce);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| Error::Unspecified(e.to_string()))
    }
}

impl From<[u8; KEY_LEN]> for Key {
    fn from(data: [u8; KEY_LEN]) -> Self {
        Self(data)
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; KEY_LEN]> for Key {
    fn as_ref(&self) -> &[u8; KEY_LEN] {
        &self.0
    }
}

impl From<Key> for [u8; KEY_LEN] {
    fn from(key: Key) -> Self {
        key.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = Key::generate().unwrap();
        let plaintext = b"Hello, world!";

        let ciphertext = key.encrypt(plaintext).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = key.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    proptest! {
        #[test]
        fn test_encrypt_decrypt_prop(plaintext in ".*") {
            let plaintext = plaintext.as_bytes();

            let key = Key::generate().unwrap();
            let ciphertext = key.encrypt(plaintext).unwrap();
            assert_ne!(ciphertext, plaintext);

            let decrypted = key.decrypt(&ciphertext).unwrap();
            assert_eq!(decrypted, plaintext);
        }

        #[test]
        fn test_get_key_from(plaintext in ".*") {
            let plaintext = plaintext.as_bytes();
            let key = Key::hash_from(plaintext);
            let key2 = Key::hash_from(plaintext);
            assert_eq!(key, key2);
            assert_ne!(key, [0u8; KEY_LEN].into());
        }
    }
}
