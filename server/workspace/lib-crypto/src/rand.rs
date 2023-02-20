use crate::{Error, Result};
use ring::rand::{SecureRandom, SystemRandom};

pub fn fill_bytes(dest: &mut [u8]) -> Result<()> {
    let rng = SystemRandom::new();
    rng.fill(dest).map_err(|_| Error::Unspecified)?;
    Ok(())
}
