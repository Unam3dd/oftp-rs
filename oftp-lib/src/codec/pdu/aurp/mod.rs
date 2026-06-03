pub mod constant;
pub mod error;

mod rfc_display;

#[cfg(test)]
mod tests;

pub use constant::*;
pub use error::AurpError;

/// Authentication Response (RFC 5024 §5.3.18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aurp {
    pub response: [u8; AURP_RESPONSE_LEN],
}

impl Default for Aurp {
    fn default() -> Self {
        Self {
            response: [0u8; AURP_RESPONSE_LEN],
        }
    }
}

impl Aurp {
    pub fn decode(&mut self, buf: &[u8]) -> Result<(), AurpError> {
        if buf.len() != AURP_LEN {
            return Err(AurpError::InvalidSizeError);
        }
        if buf[0] != AURPCMD {
            return Err(AurpError::BadCommandError);
        }
        self.response.copy_from_slice(&buf[1..AURP_LEN]);
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, AurpError> {
        let mut v = Vec::with_capacity(AURP_LEN);
        v.push(AURPCMD);
        v.extend_from_slice(&self.response);
        Ok(v)
    }
}
