//! Framing STB sur un flux synchrone (`Read` / `Write`) — sans Tokio.

use std::io::{Read, Write};

use crate::codec::oeb::OftpExchangeBuffer;
use crate::codec::stb::{StreamTransmissionBuffer, StreamTransmissionBufferError};
use crate::codec::sth::{StreamTransmissionHeader, STH_SIZE};

/// Lit un STB complet depuis un flux bloquant.
pub fn read_stb(stream: &mut impl Read) -> Result<StreamTransmissionBuffer, StreamTransmissionBufferError> {
    let mut sth_bytes = [0u8; STH_SIZE as usize];
    stream
        .read_exact(&mut sth_bytes)
        .map_err(|e| StreamTransmissionBufferError::Header(e.into()))?;

    let mut header = StreamTransmissionHeader::default();
    header.decode(&sth_bytes)?;

    let total = header.length as usize;
    if total < STH_SIZE as usize {
        return Err(StreamTransmissionBufferError::BufferTooShort);
    }

    let mut buf = vec![0u8; total];
    buf[..STH_SIZE as usize].copy_from_slice(&sth_bytes);
    if total > STH_SIZE as usize {
        stream
            .read_exact(&mut buf[STH_SIZE as usize..])
            .map_err(|e| StreamTransmissionBufferError::Header(e.into()))?;
    }

    let mut stb = StreamTransmissionBuffer {
        header,
        oeb: OftpExchangeBuffer::default(),
    };
    stb.decode(&buf)?;
    Ok(stb)
}

/// Encode et écrit un STB sur un flux bloquant.
pub fn write_stb(
    stream: &mut impl Write,
    oeb: OftpExchangeBuffer,
) -> Result<(), StreamTransmissionBufferError> {
    let mut stb = StreamTransmissionBuffer {
        header: StreamTransmissionHeader {
            version: 1,
            flags: 0,
            length: 0,
        },
        oeb,
    };
    let bytes = stb.encode()?;
    stream
        .write_all(&bytes)
        .map_err(|e| StreamTransmissionBufferError::Header(e.into()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::codec::pdu::ssrm::Ssrm;

    #[test]
    fn roundtrip_ssrm_on_cursor() {
        let oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        let mut buf = Cursor::new(Vec::new());
        write_stb(&mut buf, oeb.clone()).unwrap();
        buf.set_position(0);
        let stb = read_stb(&mut buf).unwrap();
        assert_eq!(stb.oeb, oeb);
    }
}
