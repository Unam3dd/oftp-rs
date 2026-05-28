use super::util::{encode_numeric_field, parse_numeric_field, parse_yn, yn_to_byte, NumericError, YnError};
use thiserror::Error;

pub const SFNACMD: u8 = b'3';

#[derive(Debug, Error)]
pub enum SfnaError {
    #[error("failed to decode SFNA: Bad Command !")]
    BadCommandError,
    #[error("failed to decode SFNA: Invalid SFNA packet size !")]
    InvalidSizeError,
    #[error("failed to decode SFNA: description length mismatch !")]
    DescriptionLengthMismatch,
    #[error(transparent)]
    Numeric(#[from] NumericError),
    #[error(transparent)]
    Yn(#[from] YnError),
}

#[derive(Debug, Clone)]
pub struct Sfna {
    pub reason: u8,
    pub retry: bool,
    pub reason_text: Vec<u8>,
}

impl Default for Sfna {
    fn default() -> Self {
        Self {
            reason: 99,
            retry: false,
            reason_text: Vec::new(),
        }
    }
}

impl Sfna {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SfnaError> {
        if buf.len() < 7 || buf[0] != SFNACMD {
            return Err(if buf.first() == Some(&SFNACMD) {
                SfnaError::InvalidSizeError
            } else {
                SfnaError::BadCommandError
            });
        }
        self.reason = parse_numeric_field(&buf[1..3])? as u8;
        self.retry = parse_yn(buf[3])?;
        let text_len = parse_numeric_field(&buf[4..7])? as usize;
        let expected = 7 + text_len;
        if buf.len() != expected {
            return Err(SfnaError::DescriptionLengthMismatch);
        }
        self.reason_text.clear();
        if text_len > 0 {
            self.reason_text.extend_from_slice(&buf[7..7 + text_len]);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, SfnaError> {
        if self.reason_text.len() > 999 {
            return Err(SfnaError::InvalidSizeError);
        }
        let mut v = Vec::with_capacity(7 + self.reason_text.len());
        v.push(SFNACMD);
        v.extend_from_slice(&encode_numeric_field(self.reason as u64, 2)?);
        v.push(yn_to_byte(self.retry));
        v.extend_from_slice(&encode_numeric_field(self.reason_text.len() as u64, 3)?);
        v.extend_from_slice(&self.reason_text);
        Ok(v)
    }
}
