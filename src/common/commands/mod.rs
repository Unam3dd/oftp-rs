//! Paquets ODETTE (OEB) : encode / decode par type de commande.

pub mod sfid;
pub mod ssid;
pub mod ssrm;

pub use sfid::{Sfid, SfidFieldError};
pub use ssid::{Ssid, SsidFieldError};
use ssrm::Ssrm;
use tracing::{debug, warn};

#[derive(Debug, thiserror::Error)]
pub enum PduEncodeError {
    #[error("cannot encode empty OEB")]
    EmptyBuffer,

    #[error(transparent)]
    Ssrm(#[from] ssrm::SsrmError),

    #[error(transparent)]
    Ssid(#[from] ssid::SsidError),

    #[error(transparent)]
    Sfid(#[from] sfid::SfidError),
}

#[derive(Debug, thiserror::Error)]
pub enum PduDecodeError {
    #[error("empty OEB buffer")]
    EmptyBuffer,

    #[error("unknown command: {cmd:#04x} ({label})")]
    UnknownCommand { cmd: u8, label: &'static str },

    #[error(transparent)]
    Ssrm(#[from] ssrm::SsrmError),

    #[error(transparent)]
    Ssid(#[from] ssid::SsidError),

    #[error(transparent)]
    Sfid(#[from] sfid::SfidError),
}

#[derive(Debug, Clone, Default)]
pub enum OftpExchangeBuffer {
    #[default]
    None,
    Ssrm(Ssrm),
    Ssid(Ssid),
    Sfid(Sfid),
}

impl OftpExchangeBuffer {
    pub fn encode(&self) -> Result<Vec<u8>, PduEncodeError> {
        match self {
            Self::None => Err(PduEncodeError::EmptyBuffer),
            Self::Ssrm(ssrm) => {
                let mut ssrm = *ssrm;
                Ok(ssrm.encode(ssrm.cr)?)
            }
            Self::Ssid(ssid) => {
                let mut ssid = ssid.clone();
                Ok(ssid.encode()?)
            }
            Self::Sfid(sfid) => Ok(sfid.encode()?),
        }
    }

    pub fn decode(buf: &[u8]) -> Result<Self, PduDecodeError> {
        debug!(
            len = buf.len(),
            hex = %crate::debug::hex_preview(buf, 80),
            "décodage OEB"
        );

        let cmd = *buf.first().ok_or(PduDecodeError::EmptyBuffer)?;

        match cmd {
            ssrm::SSRMCMD => {
                let mut ssrm = Ssrm { cr: 0 };
                ssrm.decode(buf)?;
                debug!(cr = ssrm.cr, "SSRM OK");
                Ok(Self::Ssrm(ssrm))
            }
            ssid::SSIDCMD => {
                let mut ssid = Ssid::default();
                ssid.decode(buf)?;
                debug!(
                    level = ssid.level,
                    buffer_size = ssid.buffer_size,
                    "SSID OK"
                );
                Ok(Self::Ssid(ssid))
            }
            sfid::SFIDCMD => {
                let mut sfid = Sfid::default();
                sfid.decode(buf)?;
                debug!(
                    format = ?sfid.format,
                    file_size_k = sfid.file_size_k,
                    desc_len = sfid.description.len(),
                    "SFID OK"
                );
                Ok(Self::Sfid(sfid))
            }
            _ => {
                let label = crate::debug::command_label(cmd);
                warn!(cmd, label, "commande OEB inconnue");
                Err(PduDecodeError::UnknownCommand { cmd, label })
            }
        }
    }
}
