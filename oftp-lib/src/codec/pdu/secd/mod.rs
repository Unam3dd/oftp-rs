pub mod constant;
pub mod error;
mod rfc_display;

#[cfg(test)]
mod tests;

pub use constant::*;
pub use error::SecdError;

/// Security Change Direction (RFC 5024 §5.3.16) — octet de commande `'J'` uniquement.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Secd;

impl Secd {
    pub fn decode(&mut self, buf: &[u8]) -> Result<(), SecdError> {
        
        if buf.len() != SECD_LEN {
            return Err(SecdError::InvalidSecdSizeError);
        }
        
        if buf[0] != SECDCMD {
            return Err(SecdError::BadCommandError);
        }

        Ok(())
    }

    pub fn encode(&mut self) -> Result<Vec<u8>, SecdError> {
        Ok(vec![SECDCMD])
    }
}
