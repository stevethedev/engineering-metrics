#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use decode::{decode, into as decode_into, Decode};
pub use encode::{encode, into as encode_into, Encode};
pub use error::{Error, Result};

mod decode;
mod encode;
mod error;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_encode(
            bytes in prop::collection::vec(any::<u8>(), 0..128)
        ) {
            let mut target = [0u8; 128 * 4 / 3];
            let len = encode_into(&bytes, &mut target).unwrap();

            assert_eq!(len, encode(&bytes).unwrap().len());
            assert_eq!(&target[..len], encode(&bytes).unwrap().as_bytes());
        }

        #[test]
        fn test_decode(
            bytes in prop::collection::vec(any::<u8>(), 0..128)
        ) {
            let encoded = encode(&bytes).unwrap();
            let mut target = [0u8; 128];
            let len = decode_into(&encoded, &mut target).unwrap();

            assert_eq!(len, bytes.len());
            assert_eq!(&target[..len], &bytes);
        }

        #[test]
        fn test_encode_decode_roundtrip(
            bytes in prop::collection::vec(any::<u8>(), 0..128)
        ) {
            let encoded = encode(&bytes).unwrap();
            let decoded = decode(encoded).unwrap();

            assert_eq!(decoded, bytes);
        }
    }

    #[test]
    fn test_encode_into() {
        let bytes = b"example bytestring!";
        let mut target = [0u8; 128];
        let len = encode_into(bytes, &mut target).unwrap();

        assert_eq!(len, 26);
        assert_eq!(&target[..len], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ");
        assert_eq!(&target[..=len], b"ZXhhbXBsZSBieXRlc3RyaW5nIQ\0");
    }

    #[test]
    fn test_decode_into() {
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
