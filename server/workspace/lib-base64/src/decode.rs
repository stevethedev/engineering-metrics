use crate::Result;
use base64ct::{Base64UrlUnpadded, Encoding};

/// Decodes the provided bytes into the provided target buffer.
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
/// use lib_base64::decode_into;
///
/// let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
/// let mut target = [0u8; 128];
/// let len = decode_into(bytes, &mut target).unwrap();
///
/// assert_eq!(len, 19);
/// assert_eq!(&target[..len], b"example bytestring!");
/// ```
///
/// ```
/// use lib_base64::{decode_into, Error};
///
/// let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
/// let mut target = [0u8; 18];
/// let result = decode_into(bytes, &mut target);
///
/// assert_eq!(result, Err(Error::InvalidLength));
/// ```
pub fn into(bytes: impl AsRef<[u8]>, mut target: impl AsMut<[u8]>) -> Result<usize> {
    let bytes = bytes.as_ref();
    let target = target.as_mut();

    Base64UrlUnpadded::decode(bytes, target)
        .map_err(Into::into)
        .map(<[u8]>::len)
}

/// Decodes the provided bytes into a new `Vec<u8>`.
///
/// # Errors
///
/// Returns an error if the string could not be decoded.
///
/// # Examples
///
/// ```
/// use lib_base64::decode;
///
/// let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
/// let decoded = decode(bytes).unwrap();
///
/// assert_eq!(decoded, b"example bytestring!");
/// ```
pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let bytes = bytes.as_ref();
    let len = (3 * bytes.len()) / 4;
    let mut target = vec![0u8; len];
    let len = into(bytes, &mut target)?;
    target.truncate(len);
    Ok(target)
}

pub trait Decode {
    /// Decodes the provided bytes into the provided target buffer.
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
    /// use lib_base64::Decode;
    ///
    /// let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
    /// let mut target = [0u8; 128];
    /// let len = bytes.decode_into(&mut target).unwrap();
    ///
    /// assert_eq!(len, 19);
    /// assert_eq!(&target[..len], b"example bytestring!");
    /// ```
    fn decode_into(&self, target: &mut [u8]) -> Result<usize>;

    /// Decodes the provided bytes into a new `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string could not be decoded.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_base64::Decode;
    ///
    /// let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
    /// let decoded = bytes.decode().unwrap();
    ///
    /// assert_eq!(decoded, b"example bytestring!");
    /// ```
    fn decode(&self) -> Result<Vec<u8>>;
}

impl<T: AsRef<[u8]>> Decode for T {
    fn decode_into(&self, target: &mut [u8]) -> Result<usize> {
        into(self, target)
    }

    fn decode(&self) -> Result<Vec<u8>> {
        decode(self)
    }
}
