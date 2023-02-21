use crate::Result;
use bytes::{Bytes, BytesMut};
use lib_crypto::fill_bytes;

/// A token is a random string of bytes.
///
/// # Example
///
/// ```
/// use lib_authentication::Token;
///
/// let token = Token::generate(32);
/// assert_eq!(token.len(), 32);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token: Bytes,
}

impl Token {
    /// Generate a new token of the given size, filled with random bytes.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the token in bytes.
    ///
    /// # Returns
    ///
    /// The generated token.
    ///
    /// # Errors
    ///
    /// Returns an error if the token could not be generated.
    pub fn generate(size: usize) -> Result<Self> {
        let mut token = BytesMut::zeroed(size);
        fill_bytes(&mut token)?;
        let token = token.freeze();
        Ok(Token { token })
    }

    /// Get the size of the token in bytes.
    ///
    /// # Returns
    ///
    /// The size of the token in bytes.
    pub fn len(&self) -> usize {
        self.token.len()
    }

    /// Check if the token is empty.
    ///
    /// # Returns
    ///
    /// `true` if the token is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.token.is_empty()
    }

    /// Convert the token into a string.
    ///
    /// # Returns
    ///
    /// The token as a string, or `None` if the token is not valid UTF-8.
    pub fn to_string(&self) -> Option<String> {
        String::from_utf8(self.as_ref().to_vec()).ok()
    }
}

impl AsRef<[u8]> for Token {
    fn as_ref(&self) -> &[u8] {
        self.token.as_ref()
    }
}

impl<T: Into<Bytes>> From<T> for Token {
    fn from(token: T) -> Self {
        let token = token.into();
        Token { token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let token = Token::generate(32).unwrap();
        assert_eq!(token.len(), 32);
    }

    #[test]
    fn test_generate_different() {
        let token1 = Token::generate(32).unwrap();
        let token2 = Token::generate(32).unwrap();
        assert_ne!(token1, token2);
    }
}
