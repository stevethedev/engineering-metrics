# Cryptography Library

This library provides cryptography support to the rest of the application.

## Use Cases

### Cryptographically Secure

This library uses the `ring` crate to provide cryptographic primitives. This crate
is a wrapper around the `BoringSSL` library, which is a fork of `OpenSSL` that is
designed to be more secure. The `ring` crate was audited by Cure53 as part of its
evaluation of RusTLS, and the results can be found [here][ring-audit].

This library uses the `chacha20poly1305` crate to provide the ChaCha20-Poly1305
AEAD algorithm. This crate has been audited by MobileCoin and the NCC Group; the
results can be found [here][chacha20poly1305-audit].

### Generate random bytes

Fill a buffer with random bytes:

```rust
use lib_crypto::fill_bytes;

pub fn main() {
    let mut buf = [0u8; 32];
    fill_bytes(&mut buf).unwrap();
}
```

[ring-audit]: https://cure53.de/pentest-report_rustls.pdf
[chacha20poly1305]: https://crates.io/crates/chacha20poly1305
[chacha20poly1305-audit]: https://research.nccgroup.com/wp-content/uploads/2020/02/NCC_Group_MobileCoin_RustCrypto_AESGCM_ChaCha20Poly1305_Implementation_Review_2020-02-12_v1.0.pdf
