use thiserror::Error;

#[derive(Debug, Error)]
pub enum SsrmError {
    #[error("failed to decode SSRM: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SSRM: Bad Message !")]
    BadSsrmMessageError,

    #[error("failed to decode SSRM: Bad Control return !")]
    BadControlReturnError,

    #[error("failed to decode SSRM: Invalid SSRM packet size !")]
    InvalidSsrmSizeError,

    #[error("failed to decode SSRM: {0} !")]
    DecodeError(#[from] std::io::Error),
}
