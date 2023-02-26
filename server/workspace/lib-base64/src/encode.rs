use crate::{Error, Result};
use base64ct::{Base64UrlUnpadded, Encoding};

/// Encodes the provided bytes into the provided target buffer.
///
/// Returns the number of bytes written to the target buffer.
///
/// # Errors
///
/// Returns an error if the target buffer is too small.
///
/// # Examples
///
/// ```
/// use lib_base64::encode_into;
///
/// let bytes = b"example bytestring!";
/// let mut target = [0u8; 128];
/// let len = encode_into(bytes, &mut target).unwrap();
///
/// assert_eq!(len, 26);
/// assert_eq!(&target[..len], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ");
/// ```
///
/// ```
/// use lib_base64::{encode_into, Error};
///
/// let bytes = b"example bytestring!";
/// let mut target = [0u8; 23];
/// let result = encode_into(bytes, &mut target);
///
/// assert_eq!(result, Err(Error::InvalidLength));
/// ```
pub fn into(bytes: impl AsRef<[u8]>, mut target: impl AsMut<[u8]>) -> Result<usize> {
    let bytes = bytes.as_ref();
    let target = target.as_mut();

    let len = Base64UrlUnpadded::encoded_len(bytes);
    if target.len() < len {
        return Err(Error::InvalidLength);
    }

    let result = Base64UrlUnpadded::encode(bytes, target).map_err(Into::<Error>::into)?;

    let result_len = result.len();

    std::str::from_utf8(target)
        .map_err(|_| Error::InvalidEncoding)
        .map(|_| result_len)
}

/// Encodes the provided bytes into a new `String`.
///
/// # Errors
///
/// Returns an error if the string could not be encoded.
///
/// # Examples
///
/// ```
/// use lib_base64::encode;
///
/// let bytes = b"example bytestring!";
/// let encoded = encode(bytes).unwrap();
///
/// assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ");
/// ```
pub fn encode(bytes: impl AsRef<[u8]>) -> Result<String> {
    let bytes = bytes.as_ref();
    let len = Base64UrlUnpadded::encoded_len(bytes);
    let mut target = vec![0u8; len];

    into(bytes, &mut target)?;

    String::from_utf8(target).map_err(|_| Error::InvalidEncoding)
}

pub trait Encode {
    /// Encodes the provided bytes into the provided target buffer.
    ///
    /// Returns the number of bytes written to the target buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if the target buffer is too small.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_base64::Encode;
    ///
    /// let bytes = b"example bytestring!";
    /// let mut target = [0u8; 128];
    /// let len = bytes.write_to(&mut target).unwrap();
    ///
    /// assert_eq!(len, 26);
    /// assert_eq!(&target[..len], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ");
    /// ```
    ///
    /// ```
    /// use lib_base64::{Encode, Error};
    ///
    /// let bytes = b"example bytestring!";
    /// let mut target = [0u8; 23];
    /// let result = bytes.write_to(&mut target);
    ///
    /// assert_eq!(result, Err(Error::InvalidLength));
    /// ```
    fn write_to(&self, target: impl AsMut<[u8]>) -> Result<usize>;

    /// Encodes the provided bytes into a new `String`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string could not be encoded.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_base64::Encode;
    ///
    /// let bytes = b"example bytestring!";
    /// let encoded = bytes.encode().unwrap();
    ///
    /// assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ");
    /// ```
    fn encode(&self) -> Result<String>;
}

impl<T: AsRef<[u8]>> Encode for T {
    fn write_to(&self, target: impl AsMut<[u8]>) -> Result<usize> {
        into(self, target)
    }

    fn encode(&self) -> Result<String> {
        let bytes = self.as_ref();
        let len = Base64UrlUnpadded::encoded_len(bytes);
        let mut target = vec![0u8; len];
        Base64UrlUnpadded::encode(bytes, &mut target).map_err(Into::<Error>::into)?;
        let result = std::str::from_utf8(&target)
            .map_err(|_| Error::InvalidEncoding)
            .map(std::string::ToString::to_string)?;
        Ok(result)
    }
}
