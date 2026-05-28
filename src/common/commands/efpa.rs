use super::util::{parse_yn, yn_to_byte, YnError};
use thiserror::Error;

pub const EFPACMD: u8 = b'4';
pub const EFPA_LEN: usize = 2;

#[derive(Debug, Error)]
pub enum EfpaError {
    #[error("failed to decode EFPA: Bad Command !")]
    BadCommandError,
    #[error("failed to decode EFPA: Invalid EFPA packet size !")]
    InvalidSizeError,
    #[error(transparent)]
    Yn(#[from] YnError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Efpa {
    pub change_direction: bool,
}

impl Efpa {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, EfpaError> {
        if buf.len() != EFPA_LEN {
            return Err(EfpaError::InvalidSizeError);
        }
        if buf[0] != EFPACMD {
            return Err(EfpaError::BadCommandError);
        }
        self.change_direction = parse_yn(buf[1])?;
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, EfpaError> {
        Ok(vec![EFPACMD, yn_to_byte(self.change_direction)])
    }
}
