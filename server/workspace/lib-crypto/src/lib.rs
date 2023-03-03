#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use encryption::Key as EncryptionKey;
pub use error::{Error, Result};
pub use hash::Sha256 as Sha256Hash;
pub use password::{hash as hash_password, verify as verify_password};
pub use rand::fill_bytes;

mod encryption;
mod error;
mod hash;
mod password;
mod rand;
