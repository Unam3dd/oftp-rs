pub mod constant;
pub mod error;
pub mod fields;
mod rfc_display;

#[cfg(test)]
mod tests;

pub use constant::*;
pub use error::EsidError;

use fields::{encode_numeric_field, parse_numeric_field};

/// Fin de session (ESIDREAS + texte optionnel).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Esid {
    pub reason: EsidReason,
    pub reason_text: Vec<u8>,
    pub cr: u8,
}

impl Esid {
    pub fn normal() -> Self {
        Self {
            reason: EsidReason::Normal,
            reason_text: Vec::new(),
            cr: ESID_CR,
        }
    }

    pub fn with_reason(reason: EsidReason) -> Self {
        Self {
            reason,
            reason_text: Vec::new(),
            cr: ESID_CR,
        }
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), EsidError> {
        if buf.len() < ESID_MIN_WIRE_LEN || buf[0] != ESIDCMD {
            return Err(if buf.first() == Some(&ESIDCMD) {
                EsidError::InvalidSizeError
            } else {
                EsidError::BadCommandError
            });
        }
        self.reason = EsidReason::from_wire(parse_numeric_field(&buf[ESID_REASON_OFFSET..ESID_TEXT_LEN_OFFSET])? as u8);
        let text_len = parse_numeric_field(&buf[ESID_TEXT_LEN_OFFSET..ESID_TEXT_OFFSET])? as usize;
        let expected = esid_wire_len(text_len);
        if buf.len() != expected {
            return Err(EsidError::DescriptionLengthMismatch);
        }
        if text_len > ESID_EMPTY_TEXT_LEN {
            self.reason_text.clear();
            self.reason_text
                .extend_from_slice(&buf[ESID_TEXT_OFFSET..ESID_TEXT_OFFSET + text_len]);
        } else {
            self.reason_text.clear();
        }
        self.cr = *buf
            .get(ESID_TEXT_OFFSET + text_len)
            .ok_or(EsidError::InvalidSizeError)?;
        if !is_valid_esid_cr(self.cr) {
            return Err(EsidError::BadControlReturnError);
        }
        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, EsidError> {

        if self.reason_text.len() > ESID_TEXT_MAX {
            return Err(EsidError::InvalidSizeError);
        }

        let mut v = Vec::with_capacity(esid_wire_len(self.reason_text.len()));

        v.push(ESIDCMD);
        v.extend_from_slice(&encode_numeric_field(
            self.reason.to_wire() as u64,
            ESID_REASON_LEN,
        )?);
        v.extend_from_slice(&encode_numeric_field(
            self.reason_text.len() as u64,
            ESID_TEXT_LEN_FIELD,
        )?);

        v.extend_from_slice(&self.reason_text);
        v.push(self.cr);

        Ok(v)
    }
}
