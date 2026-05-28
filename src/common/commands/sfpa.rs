use super::util::{encode_numeric_field, parse_numeric_field, NumericError};
use thiserror::Error;

pub const SFPACMD: u8 = b'2';
pub const SFPA_LEN: usize = 18;

#[derive(Debug, Error)]
pub enum SfpaError {
    #[error("failed to decode SFPA: Bad Command !")]
    BadCommandError,
    #[error("failed to decode SFPA: Invalid SFPA packet size !")]
    InvalidSizeError,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Sfpa {
    pub answer_count: u64,
}

impl Sfpa {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SfpaError> {
        if buf.len() != SFPA_LEN {
            return Err(SfpaError::InvalidSizeError);
        }
        if buf[0] != SFPACMD {
            return Err(SfpaError::BadCommandError);
        }
        self.answer_count = parse_numeric_field(&buf[1..18])?;
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, SfpaError> {
        let mut v = Vec::with_capacity(SFPA_LEN);
        v.push(SFPACMD);
        v.extend_from_slice(&encode_numeric_field(self.answer_count, 17)?);
        Ok(v)
    }
}
