use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuchError {
    #[error("failed to decode AUCH: bad command")]
    BadCommandError,

    #[error("failed to decode AUCH: invalid packet size")]
    InvalidSizeError,

    #[error("failed to decode AUCH: challenge length mismatch")]
    ChallengeLengthMismatch,
}
