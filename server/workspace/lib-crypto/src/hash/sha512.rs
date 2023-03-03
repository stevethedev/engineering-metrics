use sha2::{Digest, Sha512};

const HASH_LEN: usize = 64;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Hash([u8; HASH_LEN]);

impl Hash {
    pub fn new<T>(data: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut hasher = Sha512::new();
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
    fn test_sha512_hash() {
        let input = b"hello world";
        let expected = Hash::from([
            0x30, 0x9e, 0xcc, 0x48, 0x9c, 0x12, 0xd6, 0xeb, 0x4c, 0xc4, 0x0f, 0x50, 0xc9, 0x02,
            0xf2, 0xb4, 0xd0, 0xed, 0x77, 0xee, 0x51, 0x1a, 0x7c, 0x7a, 0x9b, 0xcd, 0x3c, 0xa8,
            0x6d, 0x4c, 0xd8, 0x6f, 0x98, 0x9d, 0xd3, 0x5b, 0xc5, 0xff, 0x49, 0x96, 0x70, 0xda,
            0x34, 0x25, 0x5b, 0x45, 0xb0, 0xcf, 0xd8, 0x30, 0xe8, 0x1f, 0x60, 0x5d, 0xcf, 0x7d,
            0xc5, 0x54, 0x2e, 0x93, 0xae, 0x9c, 0xd7, 0x6f,
        ]);
        let hash = Hash::new(input);

        assert_eq!(hash, expected);
    }
}
