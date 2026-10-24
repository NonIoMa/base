use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidLength { base: &'static str },
    InvalidPadding,
    InvalidCharacter { base: &'static str, ch: char },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidLength { base } => write!(f, "invalid {base} length"),
            Error::InvalidPadding => write!(f, "invalid padding"),
            Error::InvalidCharacter { base, ch } => write!(f, "invalid {base} character: {ch}"),
        }
    }
}

impl std::error::Error for Error {}
