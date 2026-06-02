pub mod constant;
pub mod error;

#[cfg(test)]
mod tests;

pub use constant::*;
pub use error::SsrmError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ssrm {
    pub cr: u8,
}

impl Ssrm {
    #[inline]
    fn is_valid_cr(cr: u8) -> bool {
        matches!(cr, 0x0D | 0x8D)
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), SsrmError> {
        if buf.len() != SSRM_LEN {
            return Err(SsrmError::InvalidSsrmSizeError);
        }

        let command = buf[0];
        let message = &buf[1..18];
        let cr = buf[18];

        if command != SSRMCMD {
            return Err(SsrmError::BadCommandError);
        }

        if message != SSRMMSG {
            return Err(SsrmError::BadSsrmMessageError);
        }

        if !Self::is_valid_cr(cr) {
            return Err(SsrmError::BadControlReturnError);
        }

        self.cr = cr;

        Ok(())
    }

    pub fn set_cr(&mut self, cr: u8) -> Result<(), SsrmError> {
        if !Self::is_valid_cr(cr) {
            return Err(SsrmError::BadControlReturnError);
        }

        self.cr = cr;

        Ok(())
    }

    pub fn encode(&mut self) -> Result<Vec<u8>, SsrmError> {
        if !Self::is_valid_cr(self.cr) {
            return Err(SsrmError::BadControlReturnError);
        }

        let mut v = Vec::with_capacity(SSRM_LEN);

        v.push(SSRMCMD);
        v.extend_from_slice(SSRMMSG);
        v.push(self.cr);

        Ok(v)
    }
}

impl Default for Ssrm {
    fn default() -> Self {
        Self { cr: 0x0D }
    }
}
