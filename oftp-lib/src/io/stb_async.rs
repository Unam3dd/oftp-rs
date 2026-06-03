//! Framing STB async (Tokio) — sync dans [`super::stb`].

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::codec::oeb::OftpExchangeBuffer;
use crate::codec::stb::{StreamTransmissionBuffer, StreamTransmissionBufferError};
use crate::codec::sth::{StreamTransmissionHeader, STH_SIZE};

/// Lit un STB complet depuis un flux async.
pub async fn read_stb_async(
    stream: &mut (impl AsyncRead + Unpin),
) -> Result<StreamTransmissionBuffer, StreamTransmissionBufferError> {
    let mut sth_bytes = [0u8; STH_SIZE as usize];
    stream
        .read_exact(&mut sth_bytes)
        .await
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
            .await
            .map_err(|e| StreamTransmissionBufferError::Header(e.into()))?;
    }

    let mut stb = StreamTransmissionBuffer {
        header,
        oeb: OftpExchangeBuffer::default(),
    };
    
    stb.decode(&buf)?;

    Ok(stb)
}

/// Encode et écrit un STB sur un flux async.
pub async fn write_stb_async(
    stream: &mut (impl AsyncWrite + Unpin),
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
        .await
        .map_err(|e| StreamTransmissionBufferError::Header(e.into()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::pdu::ssrm::Ssrm;

    #[tokio::test]
    async fn roundtrip_ssrm_on_duplex() {
        let (mut client, mut server) = tokio::io::duplex(256);
        let oeb = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });

        write_stb_async(&mut client, oeb.clone()).await.unwrap();
        let stb = read_stb_async(&mut server).await.unwrap();
        assert_eq!(stb.oeb, oeb);
    }
}
