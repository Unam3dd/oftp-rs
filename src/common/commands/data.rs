use thiserror::Error;

pub const DATACMD: u8 = b'D';

#[derive(Debug, Error)]
pub enum DataError {
    #[error("failed to decode DATA: Bad Command !")]
    BadCommandError,
    #[error("failed to decode DATA: empty buffer")]
    EmptyBuffer,
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub payload: Vec<u8>,
}

impl Data {
    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, DataError> {
        if buf.is_empty() {
            return Err(DataError::EmptyBuffer);
        }
        if buf[0] != DATACMD {
            return Err(DataError::BadCommandError);
        }
        self.payload.clear();
        self.payload.extend_from_slice(&buf[1..]);
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, DataError> {
        let mut v = Vec::with_capacity(1 + self.payload.len());
        v.push(DATACMD);
        v.extend_from_slice(&self.payload);
        Ok(v)
    }
}
