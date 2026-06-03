use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SecdError {
    #[error("failed to decode SECD: bad command")]
    BadCommandError,

    #[error("failed to decode SECD: invalid packet size (expected 1 octet)")]
    InvalidSecdSizeError,
}
