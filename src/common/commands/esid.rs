use super::util::{encode_numeric_field, parse_numeric_field, NumericError};
use thiserror::Error;

pub const ESIDCMD: u8 = b'F';

#[derive(Debug, Error)]
pub enum EsidError {
    #[error("failed to decode ESID: Bad Command !")]
    BadCommandError,
    #[error("failed to decode ESID: Invalid ESID packet size !")]
    InvalidSizeError,
    #[error("failed to decode ESID: Bad Control return !")]
    BadControlReturnError,
    #[error("failed to decode ESID: description length mismatch !")]
    DescriptionLengthMismatch,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

/// Fin de session normale (ESIDREAS = 00), sans texte de raison.
#[derive(Debug, Clone, Default)]
pub struct Esid {
    pub reason: u8,
    pub reason_text: Vec<u8>,
    pub cr: u8,
}

impl Esid {
    pub fn normal() -> Self {
        Self {
            reason: 0,
            reason_text: Vec::new(),
            cr: 0x0D,
        }
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, EsidError> {
        if buf.len() < 7 || buf[0] != ESIDCMD {
            return Err(if buf.first() == Some(&ESIDCMD) {
                EsidError::InvalidSizeError
            } else {
                EsidError::BadCommandError
            });
        }
        self.reason = parse_numeric_field(&buf[1..3])? as u8;
        let text_len = parse_numeric_field(&buf[3..6])? as usize;
        let expected = 7 + text_len;
        if buf.len() != expected {
            return Err(EsidError::DescriptionLengthMismatch);
        }
        if text_len > 0 {
            self.reason_text.clear();
            self.reason_text.extend_from_slice(&buf[6..6 + text_len]);
        } else {
            self.reason_text.clear();
        }
        self.cr = *buf.get(6 + text_len).ok_or(EsidError::InvalidSizeError)?;
        if self.cr != 0x0D && self.cr != 0x8D {
            return Err(EsidError::BadControlReturnError);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, EsidError> {
        if self.reason_text.len() > 999 {
            return Err(EsidError::InvalidSizeError);
        }
        let mut v = Vec::with_capacity(7 + self.reason_text.len());
        v.push(ESIDCMD);
        v.extend_from_slice(&encode_numeric_field(self.reason as u64, 2)?);
        v.extend_from_slice(&encode_numeric_field(self.reason_text.len() as u64, 3)?);
        v.extend_from_slice(&self.reason_text);
        v.push(self.cr);
        Ok(v)
    }
}
