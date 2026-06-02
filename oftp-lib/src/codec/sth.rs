use thiserror::Error;

pub const STH_SIZE: u32 = 4;
pub const STB_LEN_MIN: u32 = 5;
pub const STB_LEN_MAX: u32 = 100_003;

#[derive(Debug, Error)]
pub enum StreamTransmissionHeaderError {
    #[error("failed to decode STH: {0}")]
    DecodeError(#[from] std::io::Error),

    #[error("failed to decode STH: Invalid version !")]
    InvalidVersionError,

    #[error("failed to decode STH: Invalid flags !")]
    InvalidFlagsError,

    #[error("failed to decode STH: Invalid length !")]
    InvalidLengthError,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StreamTransmissionHeader {
    pub version: u8,
    pub flags: u8,
    pub length: u32,
}

impl StreamTransmissionHeader {
    
    pub fn decode(
        &mut self,
        buf: &[u8; 4],
    ) -> Result<&Self, StreamTransmissionHeaderError> {

        let version = (buf[0] >> 4) & 0xF;
        let flags = buf[0] & 0xF;
        let length = u32::from_be_bytes([0, buf[1], buf[2], buf[3]]);

        if version != 1 {
            return Err(StreamTransmissionHeaderError::InvalidVersionError);
        }

        if flags != 0 {
            return Err(StreamTransmissionHeaderError::InvalidFlagsError);
        }

        if !(STB_LEN_MIN..=STB_LEN_MAX).contains(&length) {
            return Err(StreamTransmissionHeaderError::InvalidLengthError);
        }

        self.version = version;
        self.flags = flags;
        self.length = length;

        Ok(self)
    }

    pub fn encode(
        &mut self,
        length: Option<u32>,
    ) -> Result<[u8; 4], StreamTransmissionHeaderError> {

        if let Some(l) = length {
            self.length = l;
        }
        
        if !(STB_LEN_MIN..=STB_LEN_MAX).contains(&self.length) {
            return Err(StreamTransmissionHeaderError::InvalidLengthError);
        }

        Ok(self.to_bytes())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_valid_header() {
        let mut sth = StreamTransmissionHeader::default();
        let buf = [0x10, 0x00, 0x00, 0x11];

        let decoded = sth.decode(&buf).expect("valid STH should decode");

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.flags, 0);
        assert_eq!(decoded.length, 17);
    }

    #[test]
    fn decode_rejects_invalid_version() {
        let mut sth = StreamTransmissionHeader::default();
        let buf = [0x20, 0x00, 0x00, 0x11];

        let err = sth
            .decode(&buf)
            .expect_err("version != 1 must be rejected");

        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidVersionError
        ));
    }

    #[test]
    fn decode_rejects_invalid_flags() {
        let mut sth = StreamTransmissionHeader::default();
        let buf = [0x11, 0x00, 0x00, 0x11];

        let err = sth
            .decode(&buf)
            .expect_err("flags != 0 must be rejected");

        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidFlagsError
        ));
    }

    #[test]
    fn decode_rejects_length_below_min() {
        let mut sth = StreamTransmissionHeader::default();
        let buf = [0x10, 0x00, 0x00, 0x04];

        let err = sth
            .decode(&buf)
            .expect_err("length below minimum must be rejected");

        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidLengthError
        ));
    }

    #[test]
    fn decode_rejects_length_above_max() {
        let mut sth = StreamTransmissionHeader::default();
        let too_large = STB_LEN_MAX + 1;
        let buf = [
            0x10,
            ((too_large >> 16) & 0xFF) as u8,
            ((too_large >> 8) & 0xFF) as u8,
            (too_large & 0xFF) as u8,
        ];

        let err = sth
            .decode(&buf)
            .expect_err("length above maximum must be rejected");

        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidLengthError
        ));
    }

    #[test]
    fn encode_and_decode_round_trip() {
        let mut sth = StreamTransmissionHeader {
            version: 1,
            flags: 0,
            length: 0,
        };
        let encoded = sth
            .encode(Some(42))
            .expect("length in range should encode");
        let mut decoded = StreamTransmissionHeader::default();

        decoded
            .decode(&encoded)
            .expect("encoded buffer should decode back");

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.flags, 0);
        assert_eq!(decoded.length, 42);
    }

    #[test]
    fn encode_rejects_out_of_range_lengths() {
        let mut sth = StreamTransmissionHeader {
            version: 1,
            flags: 0,
            length: STB_LEN_MIN,
        };

        let err = sth
            .encode(Some(STB_LEN_MIN - 1))
            .expect_err("length below min should fail");
        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidLengthError
        ));

        let err = sth
            .encode(Some(STB_LEN_MAX + 1))
            .expect_err("length above max should fail");
        assert!(matches!(
            err,
            StreamTransmissionHeaderError::InvalidLengthError
        ));
    }
}