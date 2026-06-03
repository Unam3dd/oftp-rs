use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AurpError {
    #[error("failed to decode AURP: bad command")]
    BadCommandError,

    #[error("failed to decode AURP: invalid packet size (expected 21 octets)")]
    InvalidSizeError
}
