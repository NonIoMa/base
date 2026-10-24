# base

Dependency-free base32, base58 and base64 encoding and decoding in Rust.

## Usage

```rust
use base::{base64_encode, base64_decode, base58_decode_fix};

let encoded = base64_encode(b"foobar");
assert_eq!(encoded, "Zm9vYmFy");
assert_eq!(base64_decode(&encoded).unwrap(), b"foobar");

let (result, cleaned) = base58_decode_fix("0O2gIl");
assert_eq!(cleaned, "2g");
assert!(result.is_ok());
```

## API

Each of `base32`, `base58` and `base64` provides:

| Function | Description |
| --- | --- |
| `*_encode(&[u8]) -> String` | Encode bytes |
| `*_decode(&str) -> Result<Vec<u8>, Error>` |
| `*_check(&str) -> bool` | Checks the base and returns bool|
| `*_decode_fix(&str) -> (Result<Vec<u8>, Error>, String)` | Strip invalid characters, repair padding, decode; returns (base,clean)|

Base32 uses the RFC 4648 alphabet and base64 the standard alphabet, both with `=` padding.\ 
Base58 uses the Bitcoin alphabet.

## Errors

`Error` has three variants: `InvalidLength`, `InvalidPadding` and `InvalidCharacter`.

## Testing

```
cargo test
```
