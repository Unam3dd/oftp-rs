use super::sfid::Sfid;
use super::util::NumericError;
use thiserror::Error;

pub const EERPCMD: u8 = b'E';
/// Taille fixe sans hash ni signature (EERPHSHL = 0, EERPSIGL = 0).
pub const EERP_FIXED_LEN: usize = 108;

#[derive(Debug, Error)]
pub enum EerpError {
    #[error("failed to decode EERP: Bad Command !")]
    BadCommandError,
    #[error("failed to decode EERP: Invalid EERP packet size !")]
    InvalidSizeError,
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

#[derive(Debug, Clone)]
pub struct Eerp {
    pub dsn: [u8; 26],
    pub date: [u8; 8],
    pub time: [u8; 10],
    pub user_data: [u8; 8],
    pub dest: [u8; 25],
    pub orig: [u8; 25],
}

impl Eerp {
    pub fn from_sfid(sfid: &Sfid) -> Self {
        Self {
            dsn: sfid.dsn,
            date: sfid.date,
            time: sfid.time,
            user_data: sfid.user_data,
            dest: sfid.dest,
            orig: sfid.orig,
        }
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, EerpError> {
        if buf.len() < EERP_FIXED_LEN || buf[0] != EERPCMD {
            return Err(if buf.first() == Some(&EERPCMD) {
                EerpError::InvalidSizeError
            } else {
                EerpError::BadCommandError
            });
        }
        self.dsn.copy_from_slice(&buf[1..27]);
        self.date.copy_from_slice(&buf[30..38]);
        self.time.copy_from_slice(&buf[38..48]);
        self.user_data.copy_from_slice(&buf[48..56]);
        self.dest.copy_from_slice(&buf[56..81]);
        self.orig.copy_from_slice(&buf[81..106]);
        let hash_len = u16::from_be_bytes([buf[106], buf[107]]);
        let mut pos = 108;
        pos += hash_len as usize;
        if buf.len() < pos + 2 {
            return Err(EerpError::InvalidSizeError);
        }
        let sig_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        pos += 2 + sig_len as usize;
        if buf.len() != pos {
            return Err(EerpError::InvalidSizeError);
        }
        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, EerpError> {
        let mut v = Vec::with_capacity(EERP_FIXED_LEN);
        v.push(EERPCMD);
        v.extend_from_slice(&self.dsn);
        v.extend_from_slice(b"   ");
        v.extend_from_slice(&self.date);
        v.extend_from_slice(&self.time);
        v.extend_from_slice(&self.user_data);
        v.extend_from_slice(&self.dest);
        v.extend_from_slice(&self.orig);
        v.extend_from_slice(&0u16.to_be_bytes());
        v.extend_from_slice(&0u16.to_be_bytes());
        Ok(v)
    }
}
