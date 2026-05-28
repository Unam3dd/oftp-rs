use thiserror::Error;

pub const RTRCMD: u8 = b'P';
pub const RTR_LEN: usize = 1;

#[derive(Debug, Error)]
pub enum RtrError {
    #[error("failed to decode RTR: Bad Command !")]
    BadCommandError,
    #[error("failed to decode RTR: Invalid RTR packet size !")]
    InvalidSizeError,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rtr;

impl Rtr {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, RtrError> {
        if buf.len() != RTR_LEN {
            return Err(RtrError::InvalidSizeError);
        }
        if buf[0] != RTRCMD {
            return Err(RtrError::BadCommandError);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, RtrError> {
        Ok(vec![RTRCMD])
    }
}
