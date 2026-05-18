//! Deterministic, human-readable identifiers using BLAKE3 and the AIS ID57 alphabet.
//!
//! Canonical flow:
//!
//! BLAKE3(input) -> base-57 projection -> ID57 string
//!
//! Supported lengths: 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57.

use std::fmt;

use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

/// AIS ID57 character set: Base62 minus 0, o, O, I, and l.
pub const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz123456789";

/// Default ID57 output length.
pub const DEFAULT_LENGTH: usize = 12;

/// Go-style alias for [`ALPHABET`].
pub use ALPHABET as Alphabet;
/// Go-style alias for [`DEFAULT_LENGTH`].
pub use DEFAULT_LENGTH as DefaultLength;

const SUPPORTED_LENGTHS: [usize; 11] = [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57];

fn alphabet_bytes() -> &'static [u8] {
    ALPHABET.as_bytes()
}

/// Error returned when a requested length is not supported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedLengthError {
    pub length: usize,
}

impl fmt::Display for UnsupportedLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id57: unsupported length; must be 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, or 57; got {}",
            self.length
        )
    }
}

impl std::error::Error for UnsupportedLengthError {}

/// Error returned when a value contains a character outside the ID57 alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidCharsetError {
    pub character: char,
    pub index: usize,
}

impl fmt::Display for InvalidCharsetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id57: value contains a character outside the ID57 alphabet: character {:?} at index {}",
            self.character, self.index
        )
    }
}

impl std::error::Error for InvalidCharsetError {}

/// Error returned when a value length does not match the requested length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthMismatchError {
    pub got: usize,
    pub want: usize,
}

impl fmt::Display for LengthMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id57: value length does not match the requested length: got {}, want {}",
            self.got, self.want
        )
    }
}

impl std::error::Error for LengthMismatchError {}

/// Error type for validation and generation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnsupportedLength(UnsupportedLengthError),
    InvalidCharset(InvalidCharsetError),
    LengthMismatch(LengthMismatchError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLength(err) => err.fmt(f),
            Self::InvalidCharset(err) => err.fmt(f),
            Self::LengthMismatch(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<UnsupportedLengthError> for Error {
    fn from(value: UnsupportedLengthError) -> Self {
        Self::UnsupportedLength(value)
    }
}

impl From<InvalidCharsetError> for Error {
    fn from(value: InvalidCharsetError) -> Self {
        Self::InvalidCharset(value)
    }
}

impl From<LengthMismatchError> for Error {
    fn from(value: LengthMismatchError) -> Self {
        Self::LengthMismatch(value)
    }
}

/// Reports whether `length` is one of the supported ID57 sizes.
pub fn is_supported_length(length: usize) -> bool {
    SUPPORTED_LENGTHS.contains(&length)
}

/// Go-style alias for [`is_supported_length`].
pub use is_supported_length as IsSupportedLength;

/// Returns an error if `length` is not one of the supported ID57 sizes.
pub fn validate_length(length: usize) -> Result<(), Error> {
    if is_supported_length(length) {
        Ok(())
    } else {
        Err(UnsupportedLengthError { length }.into())
    }
}

/// Go-style alias for [`validate_length`].
pub use validate_length as ValidateLength;

/// Encodes digest bytes into a deterministic ID57 value of the requested supported length.
///
/// A nil or empty digest is treated as zero and produces deterministic all-'A' output.
pub fn from_digest(digest: &[u8], length: usize) -> Result<String, Error> {
    validate_length(length)?;

    let alphabet = alphabet_bytes();
    let alphabet_len = BigUint::from(alphabet.len());
    let mut number = if digest.is_empty() {
        BigUint::zero()
    } else {
        BigUint::from_bytes_be(digest)
    };
    let mut buffer = vec![b'A'; length];

    for slot in &mut buffer {
        let remainder = (&number % &alphabet_len)
            .to_usize()
            .expect("alphabet remainder fits usize");
        *slot = alphabet[remainder];
        number /= &alphabet_len;
    }

    Ok(String::from_utf8(buffer).expect("ID57 alphabet is ASCII"))
}

/// Go-style alias for [`from_digest`].
pub use from_digest as FromDigest;

/// Hashes input with BLAKE3 and projects the digest into an ID57 value.
pub fn generate(input: &[u8], length: usize) -> Result<String, Error> {
    validate_length(length)?;
    let digest = blake3::hash(input);
    from_digest(digest.as_bytes(), length)
}

/// Go-style alias for [`generate`].
pub use generate as Generate;

/// Convenience wrapper around [`generate`] for UTF-8 string input.
pub fn generate_string(input: &str, length: usize) -> Result<String, Error> {
    generate(input.as_bytes(), length)
}

/// Go-style alias for [`generate_string`].
pub use generate_string as GenerateString;

/// Checks that `value` has the correct length and only ID57 characters.
pub fn validate(value: &str, length: usize) -> Result<(), Error> {
    validate_length(length)?;

    if value.chars().count() != length {
        return Err(LengthMismatchError {
            got: value.chars().count(),
            want: length,
        }
        .into());
    }

    for (index, character) in value.chars().enumerate() {
        if !ALPHABET.contains(character) {
            return Err(InvalidCharsetError { character, index }.into());
        }
    }

    Ok(())
}

/// Go-style alias for [`validate`].
pub use validate as Validate;

/// Reports whether `value` is a valid ID57 value of the requested length.
pub fn is_valid(value: &str, length: usize) -> bool {
    validate(value, length).is_ok()
}

/// Go-style alias for [`is_valid`].
pub use is_valid as IsValid;
