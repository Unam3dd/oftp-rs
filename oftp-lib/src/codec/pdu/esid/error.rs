use thiserror::Error;

#[derive(Debug, Error)]
pub enum NumericError {
    #[error("invalid numeric field")]
    Invalid,
}

#[derive(Debug, Error)]
pub enum EsidError {
    #[error("failed to decode ESID: Bad Command !")]
    BadCommandError,
    #[error("failed to decode ESID: Invalid ESID packet size !")]
    InvalidSizeError,
    #[error("failed to decode ESID: Bad Control return !")]
    BadControlReturnError,
    #[error("failed to decode ESID: description length mismatch !")]
    DescriptionLengthMismatch,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}
