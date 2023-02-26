use bytes::Bytes;

use crate::data::token::Interface;
use crate::Result;

use super::token::Token;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refresh(Token);

impl Interface for Refresh {
    /// Generate a new refresh token of the given size, filled with random bytes.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the refresh token in bytes.
    ///
    /// # Returns
    ///
    /// The generated refresh token.
    ///
    /// # Errors
    ///
    /// Returns an error if the refresh token could not be generated.
    fn generate(size: usize) -> Result<Self> {
        Token::generate(size).map(Self)
    }

    /// Get the size of the refresh token in bytes.
    ///
    /// # Returns
    ///
    /// The size of the refresh token in bytes.
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the refresh token is empty.
    ///
    /// # Returns
    ///
    /// `true` if the refresh token is empty, `false` otherwise.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Convert the refresh token into a string.
    ///
    /// # Returns
    ///
    /// The refresh token as a string, or `None` if the refresh token is not valid UTF-8.
    fn to_string(&self) -> Option<String> {
        self.0.to_string()
    }
}

impl AsRef<[u8]> for Refresh {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: Into<Bytes>> From<T> for Refresh {
    fn from(token: T) -> Self {
        let token = token.into().into();
        Self(token)
    }
}
