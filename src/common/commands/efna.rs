use super::util::{encode_numeric_field, parse_numeric_field, NumericError};
use thiserror::Error;

pub const EFNACMD: u8 = b'5';

#[derive(Debug, Error)]
pub enum EfnaError {
    #[error("failed to decode EFNA: Bad Command !")]
    BadCommandError,
    #[error("failed to decode EFNA: Invalid EFNA packet size !")]
    InvalidSizeError,
    #[error("failed to decode EFNA: description length mismatch !")]
    DescriptionLengthMismatch,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

#[derive(Debug, Clone, Default)]
pub struct Efna {
    pub reason: u8,
    pub reason_text: Vec<u8>,
}

impl Efna {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, EfnaError> {
        if buf.len() < 6 || buf[0] != EFNACMD {
            return Err(if buf.first() == Some(&EFNACMD) {
                EfnaError::InvalidSizeError
            } else {
                EfnaError::BadCommandError
            });
        }
        self.reason = parse_numeric_field(&buf[1..3])? as u8;
        let text_len = parse_numeric_field(&buf[3..6])? as usize;
        let expected = 6 + text_len;
        if buf.len() != expected {
            return Err(EfnaError::DescriptionLengthMismatch);
        }
        self.reason_text.clear();
        if text_len > 0 {
            self.reason_text.extend_from_slice(&buf[6..6 + text_len]);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, EfnaError> {
        if self.reason_text.len() > 999 {
            return Err(EfnaError::InvalidSizeError);
        }
        let mut v = Vec::with_capacity(6 + self.reason_text.len());
        v.push(EFNACMD);
        v.extend_from_slice(&encode_numeric_field(self.reason as u64, 2)?);
        v.extend_from_slice(&encode_numeric_field(self.reason_text.len() as u64, 3)?);
        v.extend_from_slice(&self.reason_text);
        Ok(v)
    }
}
