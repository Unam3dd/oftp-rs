use crate::oeb::{OftpExchangeBuffer, OftpExchangeBufferError};
use crate::sth::{StreamTransmissionHeader, StreamTransmissionHeaderError, STH_SIZE};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct StreamTransmissionBuffer {
    pub header: StreamTransmissionHeader,
    pub oeb: OftpExchangeBuffer,
}

#[derive(Debug, Error)]
pub enum StreamTransmissionBufferError {
    #[error("STH error: {0}")]
    Header(#[from] StreamTransmissionHeaderError),

    #[error("OEB error: {0}")]
    Oeb(#[from] OftpExchangeBufferError),
    
    #[error("STB buffer too short")]
    BufferTooShort,

    #[error("failed to decode STB: Empty buffer !")]
    EmptyBufferError,
    
    #[error("STB length mismatch: expected {expected}, got {got}")]
    LengthMismatch { expected: usize, got: usize },
}

impl Default for StreamTransmissionBuffer {
    fn default() -> Self {
        Self {
            header: StreamTransmissionHeader::default(),
            oeb: OftpExchangeBuffer::default(),
        }
    }
}

impl StreamTransmissionBuffer {
    
    pub fn encode(&mut self) -> Result<Vec<u8>, StreamTransmissionBufferError> {
        let oeb = self.oeb.encode()?;
        let stb_len = STH_SIZE + oeb.len() as u32;
        let sth = self.header.encode(Some(stb_len))?;

        let mut buf = Vec::with_capacity(stb_len as usize);
        buf.extend_from_slice(&sth);
        buf.extend_from_slice(&oeb);
        Ok(buf)
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), StreamTransmissionBufferError> {

        if buf.is_empty() {
            return Err(StreamTransmissionBufferError::EmptyBufferError);
        }

        if buf.len() < STH_SIZE as usize {
            return Err(StreamTransmissionBufferError::BufferTooShort);
        }

        let sth: [u8; 4] = buf[..4]
            .try_into()
            .map_err(|_| StreamTransmissionBufferError::BufferTooShort)?;

        self.header.decode(&sth)?;

        let expected = self.header.length as usize;
        
        if buf.len() != expected {
            return Err(StreamTransmissionBufferError::LengthMismatch {
                expected,
                got: buf.len(),
            });
        }

        self.oeb.decode(&buf[STH_SIZE as usize..expected])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdu::ssrm::{Ssrm, SSRM_LEN, SSRMCMD};
    use crate::pdu::ssid::{Ssid, SSID_LEN, SSIDCMD};

    fn test_header() -> StreamTransmissionHeader {
        StreamTransmissionHeader {
            version: 1,
            flags: 0,
            length: 0,
        }
    }

    fn ssrm_wire() -> [u8; SSRM_LEN] {
        [
            SSRMCMD,
            b'O', b'D', b'E', b'T', b'T', b'E', b' ',
            b'F', b'T', b'P', b' ',
            b'R', b'E', b'A', b'D', b'Y', b' ',
            0x0D,
        ]
    }

    #[test]
    fn roundtrip_ssrm() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }),
        };

        let wire = stb.encode().unwrap();
        println!("wire ({}): {:02X?}", wire.len(), wire);
        assert_eq!(wire.len(), STH_SIZE as usize + SSRM_LEN);
        assert_eq!(stb.header.length, STH_SIZE + SSRM_LEN as u32);

        let mut decoded = StreamTransmissionBuffer::default();
        decoded.decode(&wire).unwrap();

        match decoded.oeb {
            OftpExchangeBuffer::Ssrm(s) => assert_eq!(s.cr, 0x0D),
            other => panic!("expected Ssrm, got {other:?}"),
        }
    }

    #[test]
    fn roundtrip_ssid() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssid(Ssid::default()),
        };

        let wire = stb.encode().unwrap();
        assert_eq!(wire.len(), STH_SIZE as usize + SSID_LEN);
        assert_eq!(wire[4], SSIDCMD);

        let mut decoded = StreamTransmissionBuffer::default();
        decoded.decode(&wire).unwrap();
        assert!(matches!(decoded.oeb, OftpExchangeBuffer::Ssid(_)));
    }

    #[test]
    fn encode_sth_length_matches_stb_size() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }),
        };
        let wire = stb.encode().unwrap();

        let payload_len =
            u32::from_be_bytes([0, wire[1], wire[2], wire[3]]) as usize;
        assert_eq!(payload_len, wire.len());
        assert_eq!(wire[0] >> 4, 1); // version
    }

    #[test]
    fn decode_empty_buffer() {
        let mut stb = StreamTransmissionBuffer::default();
        let err = stb.decode(&[]).unwrap_err();
        assert!(matches!(
            err,
            StreamTransmissionBufferError::EmptyBufferError
        ));
    }

    #[test]
    fn decode_buffer_too_short() {
        let mut stb = StreamTransmissionBuffer::default();
        let err = stb.decode(&[0x10, 0x00]).unwrap_err();
        assert!(matches!(
            err,
            StreamTransmissionBufferError::BufferTooShort
        ));
    }

    #[test]
    fn decode_length_mismatch() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }),
        };
        let mut wire = stb.encode().unwrap();
        wire.pop();

        let mut decoded = StreamTransmissionBuffer::default();
        let err = decoded.decode(&wire).unwrap_err();
        assert!(matches!(
            err,
            StreamTransmissionBufferError::LengthMismatch { .. }
        ));
    }

    #[test]
    fn decode_invalid_oeb_propagates() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }),
        };
        let mut wire = stb.encode().unwrap();
        wire[4] = b'Z';

        let mut decoded = StreamTransmissionBuffer::default();
        let err = decoded.decode(&wire).unwrap_err();
        assert!(matches!(err, StreamTransmissionBufferError::Oeb(_)));
    }

    #[test]
    fn roundtrip_decode_encode_decode() {
        let mut stb = StreamTransmissionBuffer {
            header: test_header(),
            oeb: OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x8D }),
        };
        let wire = stb.encode().unwrap();

        let mut mid = StreamTransmissionBuffer::default();
        mid.decode(&wire).unwrap();
        let again = mid.encode().unwrap();
        assert_eq!(again, wire);

        let mut end = StreamTransmissionBuffer::default();
        end.decode(&again).unwrap();
        match end.oeb {
            OftpExchangeBuffer::Ssrm(s) => assert_eq!(s.cr, 0x8D),
            other => panic!("expected Ssrm, got {other:?}"),
        }
    }

    #[test]
    fn decode_manual_stb_bytes() {
        let oeb = ssrm_wire();
        let stb_len = STH_SIZE + oeb.len() as u32;
        let mut header = test_header();
        let sth = header.encode(Some(stb_len)).unwrap();

        let mut wire = Vec::new();
        wire.extend_from_slice(&sth);
        wire.extend_from_slice(&oeb);

        let mut stb = StreamTransmissionBuffer::default();
        stb.decode(&wire).unwrap();
        assert!(matches!(stb.oeb, OftpExchangeBuffer::Ssrm(_)));
    }
}
