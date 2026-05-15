use thiserror::Error;

pub const SSRMCMD: u8 = b'I';
pub const SSRMMSG: &[u8] = b"ODETTE FTP READY ";

#[derive(Debug, Error)]
pub enum SsrmError {
    #[error("failed to decode SSRM is empty !")]
    EmptySsrmError,

    #[error("failed to decode SSRM: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SSRM: Bad Message !")]
    BadSsrmMessageError,

    #[error("failed to decode SSRM: Bad Control return !")]
    BadControlReturnError,

    #[error("failed to decode SSRM: {0} !")]
    DecodeError(#[from] std::io::Error),

    #[error("failed to decode SSRM: Ssrm packet is too long !")]
    SsrmTooLongError
}

#[derive(Debug, Copy, Clone)]
pub struct Ssrm {
    pub cr: u8
}

impl Ssrm {

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SsrmError> {

        if buf.len() == 0 {
            return Err(SsrmError::EmptySsrmError);
        }

        if buf.len() >= 19 {
            return Err(SsrmError::SsrmTooLongError);
        }

        let command = buf[0];
        let message = &buf[1..17];
        let cr = buf[18];

        if command != SSRMCMD {
            return Err(SsrmError::BadCommandError);
        }

        if message != SSRMMSG {
            return Err(SsrmError::BadSsrmMessageError);
        }

        if cr != 0x0D && cr != 0x8D {
            return Err(SsrmError::BadControlReturnError);
        }

        dbg!(buf);

        Ok(self)
    }

}
