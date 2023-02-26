use bytes::Bytes;

use crate::data::token::Interface;
use crate::Result;

use super::token::Token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Auth(Token);

impl Interface for Auth {
    /// Generate a new authentication token of the given size, filled with random bytes.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the authentication token in bytes.
    ///
    /// # Returns
    ///
    /// The generated authentication token.
    ///
    /// # Errors
    ///
    /// Returns an error if the authentication token could not be generated.
    fn generate(size: usize) -> Result<Self> {
        Token::generate(size).map(Self)
    }

    /// Get the size of the authentication token in bytes.
    ///
    /// # Returns
    ///
    /// The size of the authentication token in bytes.
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the authentication token is empty.
    ///
    /// # Returns
    ///
    /// `true` if the authentication token is empty, `false` otherwise.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Convert the authentication token into a string.
    ///
    /// # Returns
    ///
    /// The authentication token as a string, or `None` if the authentication token is not valid UTF-8.
    fn to_string(&self) -> Option<String> {
        self.0.to_string()
    }
}

impl AsRef<[u8]> for Auth {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: Into<Bytes>> From<T> for Auth {
    fn from(token: T) -> Self {
        let token = token.into().into();
        Self(token)
    }
}
