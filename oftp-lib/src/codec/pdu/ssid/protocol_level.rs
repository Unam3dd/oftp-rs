#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProtocolLevel {
    Rev12 = b'1',
    Rev13 = b'2',
    Rev14 = b'4',
    Rev20 = b'5',
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[error("invalid protocol level")]
pub struct ProtocolLevelError;

impl TryFrom<u8> for ProtocolLevel {
    type Error = ProtocolLevelError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'1' => Ok(Self::Rev12),
            b'2' => Ok(Self::Rev13),
            b'4' => Ok(Self::Rev14),
            b'5' => Ok(Self::Rev20),
            _ => Err(ProtocolLevelError),
        }
    }
}

impl ProtocolLevel {
    #[inline]
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}
