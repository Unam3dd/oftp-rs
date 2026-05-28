use thiserror::Error;

pub const CDCMD: u8 = b'R';
pub const CD_LEN: usize = 1;

#[derive(Debug, Error)]
pub enum CdError {
    #[error("failed to decode CD: Bad Command !")]
    BadCommandError,
    #[error("failed to decode CD: Invalid CD packet size !")]
    InvalidSizeError,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Cd;

impl Cd {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, CdError> {
        if buf.len() != CD_LEN {
            return Err(CdError::InvalidSizeError);
        }
        if buf[0] != CDCMD {
            return Err(CdError::BadCommandError);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, CdError> {
        Ok(vec![CDCMD])
    }
}
