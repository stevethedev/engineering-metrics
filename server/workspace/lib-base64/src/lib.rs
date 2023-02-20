#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

mod decode;
mod encode;
mod error;

pub use decode::{decode, into as decode_into, Decode};
pub use encode::{encode, into as encode_into, Encode};
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let bytes = b"example bytestring!";
        let mut target = [0u8; 128];
        let len = encode_into(bytes, &mut target).unwrap();

        assert_eq!(len, 26);
        assert_eq!(&target[..len], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ");
        assert_eq!(&target[..len + 1], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ\0");
    }

    #[test]
    fn test_decode() {
        let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
        let mut target = [0u8; 128];
        let len = decode_into(bytes, &mut target).unwrap();

        assert_eq!(len, 19);
        assert_eq!(&target[..len], b"example bytestring!");
    }

    #[test]
    fn test_encode_decode() {
        let bytes = b"example bytestring!";
        let mut target = [0u8; 128];
        let len = encode_into(bytes, &mut target).unwrap();
        let encoded = &target[..len];

        let mut target = [0u8; 128];
        let len = decode_into(encoded, &mut target).unwrap();
        let decoded = &target[..len];

        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_decode_encode() {
        let bytes = b"ZXhhbXBsZSBieXRlc3RyaW5nIQ";
        let mut target = [0u8; 128];
        let len = decode_into(bytes, &mut target).unwrap();
        let decoded = &target[..len];

        let mut target = [0u8; 128];
        let len = encode_into(decoded, &mut target).unwrap();
        let encoded = &target[..len];

        assert_eq!(encoded, bytes);
    }
}
