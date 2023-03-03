use sha2::{Digest, Sha256};

const HASH_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hash([u8; HASH_LEN]);

impl Hash {
    pub fn new<T>(data: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0; HASH_LEN];
        hash.copy_from_slice(result.as_ref());
        Self(hash)
    }
}

impl From<[u8; HASH_LEN]> for Hash {
    fn from(data: [u8; HASH_LEN]) -> Self {
        Self(data)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Hash> for [u8; HASH_LEN] {
    fn from(hash: Hash) -> Self {
        hash.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let input = b"hello world";
        let expected = Hash::from([
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
            0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
            0xe2, 0xef, 0xcd, 0xe9,
        ]);
        let hash = Hash::new(input);

        assert_eq!(hash, expected);
    }
}
