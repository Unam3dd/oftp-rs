//! Transport TCP : Stream Transmission Header (STH) + OEB.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use thiserror::Error;
use tracing::{debug, warn};

use super::commands::{OftpExchangeBuffer, PduDecodeError, PduEncodeError};

const STH_SIZE: u32 = 4;
const STB_LEN_MIN: u32 = 5;
const STB_LEN_MAX: u32 = 100_003;

#[derive(Debug, Error)]
pub enum StreamTransmissionHeaderError {
    #[error("failed to decode STH: {0}")]
    DecodeError(#[from] std::io::Error),
}

const STH_VERSION: u8 = 1;

#[derive(Debug, Error)]
pub enum PduWriteError {
    #[error("invalid STB length: {0}")]
    InvalidStbLength(u32),

    #[error("failed to write STB: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Encode(#[from] PduEncodeError),
}

#[derive(Debug, Error)]
pub enum PduReadError {
    #[error("invalid STB length: {0}")]
    InvalidStbLength(u32),

    #[error("failed to read STB: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    StbHeader(#[from] StreamTransmissionHeaderError),

    #[error(transparent)]
    Decode(#[from] PduDecodeError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StreamTransmissionHeader {
    pub version: u8,
    pub flags: u8,
    pub length: u32,
}

#[derive(Debug, Default)]
pub struct StreamTransmission {
    sth: StreamTransmissionHeader,
    oeb: OftpExchangeBuffer,
}

impl StreamTransmission {
    pub fn sth(&self) -> &StreamTransmissionHeader {
        &self.sth
    }

    pub fn pdu(&self) -> &OftpExchangeBuffer {
        &self.oeb
    }

    pub async fn read_pdu(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<(StreamTransmissionHeader, OftpExchangeBuffer), PduReadError> {
        self.sth.decode(stream).await?;
        debug!(
            version = self.sth.version,
            flags = self.sth.flags,
            length = self.sth.length,
            "STH reçu"
        );

        let stb_len = self.sth.length;
        if !(STB_LEN_MIN..=STB_LEN_MAX).contains(&stb_len) {
            warn!(stb_len, "longueur STB invalide");
            return Err(PduReadError::InvalidStbLength(stb_len));
        }

        let oeb_len = stb_len - STH_SIZE;
        let mut buf = vec![0u8; oeb_len as usize];
        stream.read_exact(&mut buf).await?;
        debug!(
            oeb_len,
            hex = %super::debug::hex_preview(&buf, 96),
            "OEB reçu"
        );

        self.oeb = OftpExchangeBuffer::decode(&buf).map_err(|e| {
            warn!(error = %e, "échec décodage OEB");
            e
        })?;
        Ok((self.sth, self.oeb.clone()))
    }

    pub async fn write_pdu(
        &mut self,
        stream: &mut TcpStream,
        pdu: &OftpExchangeBuffer,
    ) -> Result<StreamTransmissionHeader, PduWriteError> {
        let oeb = pdu.encode()?;
        let stb_len = STH_SIZE + oeb.len() as u32;

        if !(STB_LEN_MIN..=STB_LEN_MAX).contains(&stb_len) {
            return Err(PduWriteError::InvalidStbLength(stb_len));
        }

        let sth = StreamTransmissionHeader {
            version: STH_VERSION,
            flags: 0,
            length: stb_len,
        };

        let mut out = Vec::with_capacity(stb_len as usize);
        out.extend_from_slice(&sth.to_bytes());
        out.extend_from_slice(&oeb);
        debug!(
            stb_len,
            hex = %super::debug::hex_preview(&out, 96),
            "STB envoyé"
        );
        stream.write_all(&out).await?;

        self.sth = sth;
        self.oeb = pdu.clone();

        Ok(sth)
    }
}

impl StreamTransmissionHeader {
    pub async fn decode(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<&Self, StreamTransmissionHeaderError> {
        let mut buf = [0u8; 4];

        self.version = 0;
        self.flags = 0;
        self.length = 0;

        stream.read_exact(&mut buf).await?;

        self.version = (buf[0] >> 4) & 0xF;
        self.flags = buf[0] & 0xF;
        self.length = (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32);

        Ok(self)
    }

    pub fn encode(
        &mut self,
        version: Option<u8>,
        flags: Option<u8>,
        length: Option<u32>,
    ) -> &Self {
        if let Some(value) = version {
            self.version = value;
        }

        if let Some(value) = flags {
            self.flags = value;
        }

        if let Some(value) = length {
            self.length = value;
        }

        self
    }

    /// Octets STH sur le fil (version 1, RFC §8.2).
    pub fn to_bytes(self) -> [u8; 4] {
        let b0 = (self.version & 0xF) << 4 | (self.flags & 0xF);
        [
            b0,
            ((self.length >> 16) & 0xFF) as u8,
            ((self.length >> 8) & 0xFF) as u8,
            (self.length & 0xFF) as u8,
        ]
    }
}
