pub mod constant;
pub mod error;

mod rfc_display;

#[cfg(test)]
mod tests;

pub use constant::*;
pub use error::AuchError;

/// Authentication Challenge (RFC 5024 §5.3.17).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Auch {
    /// AUCHCHAL — défi (souvent enveloppe CMS sur le fil).
    pub challenge: Vec<u8>,
}

impl Auch {
    pub fn with_challenge(challenge: impl Into<Vec<u8>>) -> Self {
        Self {
            challenge: challenge.into(),
        }
    }

    pub fn wire_len(challenge_len: usize) -> usize {
        AUCH_MIN_WIRE_LEN + challenge_len
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), AuchError> {
        if buf.len() < AUCH_MIN_WIRE_LEN || buf[0] != AUCHCMD {
            return Err(if buf.first() == Some(&AUCHCMD) {
                AuchError::InvalidSizeError
            } else {
                AuchError::BadCommandError
            });
        }
        let chal_len = u16::from_be_bytes([buf[1], buf[2]]) as usize;
        let expected = Self::wire_len(chal_len);
        if buf.len() != expected || chal_len > AUCH_CHALLENGE_MAX {
            return Err(AuchError::ChallengeLengthMismatch);
        }
        self.challenge = buf[AUCH_MIN_WIRE_LEN..expected].to_vec();
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, AuchError> {
        let len = self.challenge.len();
        if len > AUCH_CHALLENGE_MAX {
            return Err(AuchError::ChallengeLengthMismatch);
        }
        let mut v = Vec::with_capacity(Self::wire_len(len));
        v.push(AUCHCMD);
        v.extend_from_slice(&(len as u16).to_be_bytes());
        v.extend_from_slice(&self.challenge);
        Ok(v)
    }
}
