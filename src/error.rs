use std::fmt;

/// Errors that can occur during RLP encoding/decoding
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Input bytes are too short
    InputTooShort,
    /// Value length exceeds max allowed
    ValueTooLong,
    /// Non-minimal encoding used for length
    NonMinimalEncoding,
    /// Length prefix is invalid
    InvalidLength,
    /// Expected list, got something else
    UnexpectedString,
    /// Expected string, got something else
    UnexpectedList,
    /// Input contains additional data after valid RLP
    UnexpectedTrailing,
    /// Invalid UTF-8 while decoding a String.
    InvalidUtf8,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InputTooShort => write!(f, "RLP input too short"),
            Error::ValueTooLong => write!(f, "RLP value length exceeds maximum"),
            Error::NonMinimalEncoding => write!(f, "Non-minimal length encoding"),
            Error::InvalidLength => write!(f, "Invalid length prefix"),
            Error::UnexpectedString => write!(f, "Expected list, got string"),
            Error::UnexpectedList => write!(f, "Expected string, got list"),
            Error::UnexpectedTrailing => write!(f, "Unexpected trailing bytes"),
            Error::InvalidUtf8 => write!(f, "Invalid UTF-8 while decoding a String"),
        }
    }
}