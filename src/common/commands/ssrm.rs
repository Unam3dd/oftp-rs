use thiserror::Error;

pub const SSRMCMD: u8 = b'I';
pub const SSRMMSG: &[u8] = b"ODETTE FTP READY ";
pub const SSRM_LEN: usize = 0x13;

#[derive(Debug, Error)]
pub enum SsrmError {
    #[error("failed to decode SSRM: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SSRM: Bad Message !")]
    BadSsrmMessageError,

    #[error("failed to decode SSRM: Bad Control return !")]
    BadControlReturnError,

    #[error("failed to decode SSRM: Invalid SSRM packet size !")]
    InvalidSsrmSizeError,

    #[error("failed to decode SSRM: {0} !")]
    DecodeError(#[from] std::io::Error),
}

#[derive(Debug, Copy, Clone)]
pub struct Ssrm {
    pub cr: u8
}

impl Ssrm {

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SsrmError> {

        if buf.len() != SSRM_LEN {
            return Err(SsrmError::InvalidSsrmSizeError);
        }

        let command= buf[0];
        let message = &buf[1..18];
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

        self.cr = cr;

        Ok(self)
    }

    pub fn encode(&mut self, cr: u8) -> Result<Vec<u8>, SsrmError> {
        let mut v = Vec::with_capacity(SSRMMSG.len() + 2);

        v.push(SSRMCMD);

        v.extend_from_slice(SSRMMSG);

        if cr != 0x0D && cr != 0x8D {
            return Err(SsrmError::BadControlReturnError);
        }

        v.push(cr);

        Ok(v)
    }

}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut ssrm = Ssrm { cr: 0 };
        assert!(ssrm.encode(0x0D).is_ok());
    }

    #[test]
    fn test_encode_bad_cr() {
        let mut ssrm = Ssrm { cr: 0 };

        let res = ssrm.encode(0x0A).unwrap_err();

        assert!(matches!(res, SsrmError::BadControlReturnError));
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn test_decode_ok() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        assert!(buf.len() == SSRM_LEN);
        assert!(ssrm.decode(&buf).is_ok());
    }

    #[test]
    fn test_decode_cr() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        assert!(buf.len() == SSRM_LEN);

        let res = ssrm.decode(&buf).unwrap();

        assert!(res.cr != 0);
    }

    #[test]
    fn test_decode_bad_cr() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0xAA
        ];

        assert!(buf.len() == SSRM_LEN);

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadControlReturnError));
    }

    #[test]
    fn test_decode_bad_len() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::InvalidSsrmSizeError));
    }

    #[test]
    fn test_decode_bad_command() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            b'E',
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadCommandError));
    }

    #[test]
    fn test_decode_bad_message() {
        let mut ssrm = Ssrm { cr: 0 };

        let buf = [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'P', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D
        ];

        let res = ssrm.decode(&buf).unwrap_err();

        assert!(matches!(res, SsrmError::BadSsrmMessageError));
    }
}
