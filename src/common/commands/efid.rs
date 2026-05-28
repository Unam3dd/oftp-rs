use super::util::{encode_numeric_field, parse_numeric_field, NumericError};
use thiserror::Error;

pub const EFIDCMD: u8 = b'T';
pub const EFID_LEN: usize = 35;

#[derive(Debug, Error)]
pub enum EfidError {
    #[error("failed to decode EFID: Bad Command !")]
    BadCommandError,
    #[error("failed to decode EFID: Invalid EFID packet size !")]
    InvalidSizeError,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Efid {
    pub record_count: u64,
    pub unit_count: u64,
}

impl Efid {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, EfidError> {
        if buf.len() != EFID_LEN {
            return Err(EfidError::InvalidSizeError);
        }
        if buf[0] != EFIDCMD {
            return Err(EfidError::BadCommandError);
        }
        self.record_count = parse_numeric_field(&buf[1..18])?;
        self.unit_count = parse_numeric_field(&buf[18..35])?;
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, EfidError> {
        let mut v = Vec::with_capacity(EFID_LEN);
        v.push(EFIDCMD);
        v.extend_from_slice(&encode_numeric_field(self.record_count, 17)?);
        v.extend_from_slice(&encode_numeric_field(self.unit_count, 17)?);
        Ok(v)
    }
}
