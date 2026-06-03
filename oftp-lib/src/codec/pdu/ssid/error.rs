use thiserror::Error;
use super::mode::SsidModeError;
use super::protocol_level::ProtocolLevelError;

#[derive(Debug, Error)]
pub enum SsidError {
    #[error("failed to decode SSID: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SSID: Bad Protocol Level !")]
    BadProtocolLevelError,

    #[error("failed to decode SSID: Bad Buffer Size !")]
    BadBufferSizeError,

    #[error("failed to decode SSID: Bad Send/Receive Capability !")]
    BadModeError,

    #[error("failed to decode SSID: Bad Y/N Indicator !")]
    BadYnError,

    #[error("failed to decode SSID: Bad Credit !")]
    BadCreditError,

    #[error("failed to decode SSID: Bad Control return !")]
    BadControlReturnError,

    #[error("failed to decode SSID: Invalid SSID packet size !")]
    InvalidSsidSizeError,

    #[error("failed to encode SSID: Bad Buffer Size !")]
    EncodeBufferSizeError,

    #[error("failed to encode SSID: Bad Credit !")]
    EncodeCreditError,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SsidFieldError {
    #[error("identification code too long (max 25 characters)")]
    CodeTooLong,

    #[error("password too long (max 8 characters)")]
    PasswordTooLong,

    #[error("buffer size out of range (min 128, max 99999)")]
    BufferSizeOutOfRange,

    #[error("protocol level out of range (min Rev12, max Rev20)")]
    ProtocolLevelOutOfRange,

    #[error("credit out of range (max 999)")]
    CreditOutOfRange,
}

impl From<ProtocolLevelError> for SsidError {
    fn from(_: ProtocolLevelError) -> Self {
        SsidError::BadProtocolLevelError
    }
}

impl From<SsidModeError> for SsidError {
    fn from(_: SsidModeError) -> Self {
        SsidError::BadModeError
    }
}
