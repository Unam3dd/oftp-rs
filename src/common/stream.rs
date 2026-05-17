use std::{io::Read, net::TcpStream};
use thiserror::Error;

use super::commands::OftpExchangeBuffer;

#[derive(Debug, Error)]
pub enum StreamTransmissionHeaderError {
    #[error("failed to decode STH: {0}")]
    DecodeError(#[from] std::io::Error),
}

#[derive(Debug,Clone,Copy)]
pub struct StreamTransmissionHeader {
    pub version: u8,
    pub flags: u8,
    pub length: u32
}

#[derive(Debug)]
pub struct StreamTransmission {
    sth: StreamTransmissionHeader,
    oeb: OftpExchangeBuffer
}

impl StreamTransmissionHeader {

    pub fn decode(&mut self, stream: &mut TcpStream) -> Result<&Self, StreamTransmissionHeaderError> {
        let mut buf = [0u8; 4];

        self.version = 0;
        self.flags = 0;
        self.length = 0;

        stream.read_exact(&mut buf)?;

        self.version = (buf[0] >> 4) & 0xF;
        self.flags = buf[0] & 0xF;
        self.length = (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32);

        Ok(self)
    }

    pub fn encode(&mut self, version: Option<u8>, flags: Option<u8>, length: Option<u32>) -> &Self {

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

}
