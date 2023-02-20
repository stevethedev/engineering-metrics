# Cryptography Library

This library provides cryptography support to the rest of the application.

## Use Cases

### Cryptographically Secure

This library uses the `ring` crate to provide cryptographic primitives. This crate
is a wrapper around the `BoringSSL` library, which is a fork of `OpenSSL` that is
designed to be more secure. The `ring` crate was audited by Cure53 as part of its
evaluation of RusTLS, and the results can be found [here][ring-audit].

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
