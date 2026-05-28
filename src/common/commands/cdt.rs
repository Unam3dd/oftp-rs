use thiserror::Error;

pub const CDTCMD: u8 = b'C';
pub const CDT_LEN: usize = 3;

#[derive(Debug, Error)]
pub enum CdtError {
    #[error("failed to decode CDT: Bad Command !")]
    BadCommandError,
    #[error("failed to decode CDT: Invalid CDT packet size !")]
    InvalidSizeError,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Cdt;

impl Cdt {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, CdtError> {
        if buf.len() != CDT_LEN {
            return Err(CdtError::InvalidSizeError);
        }
        if buf[0] != CDTCMD {
            return Err(CdtError::BadCommandError);
        }
        if &buf[1..3] != b"  " {
            return Err(CdtError::InvalidSizeError);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, CdtError> {
        Ok(vec![CDTCMD, b' ', b' '])
    }
}
