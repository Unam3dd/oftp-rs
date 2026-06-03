#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsidMode {
    SendOnly,
    ReceiveOnly,
    Both,
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[error("invalid send/receive mode")]
pub struct SsidModeError;

impl TryFrom<u8> for SsidMode {
    type Error = SsidModeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'S' => Ok(Self::SendOnly),
            b'R' => Ok(Self::ReceiveOnly),
            b'B' => Ok(Self::Both),
            _ => Err(SsidModeError),
        }
    }
}

impl SsidMode {
    #[inline]
    pub fn to_byte(self) -> u8 {
        match self {
            Self::SendOnly => b'S',
            Self::ReceiveOnly => b'R',
            Self::Both => b'B',
        }
    }
}