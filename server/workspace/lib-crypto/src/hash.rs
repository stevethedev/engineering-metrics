use sha2::{Digest, Sha256};

const HASH_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Sha256Hash([u8; HASH_LEN]);

impl Sha256Hash {
    pub fn new<T>(data: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0; HASH_LEN];
        hash.copy_from_slice(&result);
        Self(hash)
    }
}

impl From<[u8; HASH_LEN]> for Sha256Hash {
    fn from(data: [u8; HASH_LEN]) -> Self {
        Self(data)
    }
}

impl AsRef<[u8]> for Sha256Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Sha256Hash> for [u8; HASH_LEN] {
    fn from(hash: Sha256Hash) -> Self {
        hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let input = b"hello world";
        let expected = Sha256Hash::from([
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
            0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
            0xe2, 0xef, 0xcd, 0xe9,
        ]);
        let hash = Sha256Hash::new(input);

        assert_eq!(hash, expected);
    }
}
