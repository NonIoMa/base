//! Base32, base58 and base64 encoding and decoding.

mod base;
mod error;
pub use error::Error;
pub use base::base32::*;
pub use base::base58::*;
pub use base::base64::*;
pub use base::{base32, base58, base64};
