//! Paquets ODETTE (OEB) : encode / decode par type de commande.

pub mod cd;
pub mod cdt;
pub mod data;
pub mod eerp;
pub mod efid;
pub mod efna;
pub mod efpa;
pub mod esid;
pub mod rtr;
pub mod sfid;
pub mod sfna;
pub mod sfpa;
pub mod ssid;
pub mod ssrm;
pub mod util;

pub use cd::Cd;
pub use cdt::Cdt;
pub use data::Data;
pub use eerp::Eerp;
pub use efid::Efid;
pub use efna::Efna;
pub use efpa::Efpa;
pub use esid::Esid;
pub use rtr::Rtr;
pub use sfid::{Sfid, SfidFieldError, SfidFormat};
pub use sfna::Sfna;
pub use sfpa::Sfpa;
pub use ssid::{Ssid, SsidFieldError};
pub use ssrm::Ssrm;
use ssrm::Ssrm as SsrmPdu;
use tracing::{debug, warn};

use crate::event::InputEvent;

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

    #[error(transparent)]
    Sfpa(#[from] sfpa::SfpaError),

    #[error(transparent)]
    Sfna(#[from] sfna::SfnaError),

    #[error(transparent)]
    Data(#[from] data::DataError),

    #[error(transparent)]
    Cdt(#[from] cdt::CdtError),

    #[error(transparent)]
    Efid(#[from] efid::EfidError),

    #[error(transparent)]
    Efpa(#[from] efpa::EfpaError),

    #[error(transparent)]
    Efna(#[from] efna::EfnaError),

    #[error(transparent)]
    Esid(#[from] esid::EsidError),

    #[error(transparent)]
    Cd(#[from] cd::CdError),

    #[error(transparent)]
    Eerp(#[from] eerp::EerpError),

    #[error(transparent)]
    Rtr(#[from] rtr::RtrError),
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

    #[error(transparent)]
    Sfpa(#[from] sfpa::SfpaError),

    #[error(transparent)]
    Sfna(#[from] sfna::SfnaError),

    #[error(transparent)]
    Data(#[from] data::DataError),

    #[error(transparent)]
    Cdt(#[from] cdt::CdtError),

    #[error(transparent)]
    Efid(#[from] efid::EfidError),

    #[error(transparent)]
    Efpa(#[from] efpa::EfpaError),

    #[error(transparent)]
    Efna(#[from] efna::EfnaError),

    #[error(transparent)]
    Esid(#[from] esid::EsidError),

    #[error(transparent)]
    Cd(#[from] cd::CdError),

    #[error(transparent)]
    Eerp(#[from] eerp::EerpError),

    #[error(transparent)]
    Rtr(#[from] rtr::RtrError),
}

#[derive(Debug, Clone)]
pub enum OftpExchangeBuffer {
    None,
    Ssrm(SsrmPdu),
    Ssid(Ssid),
    Sfid(Sfid),
    Sfpa(Sfpa),
    Sfna(Sfna),
    Data(Data),
    Cdt(Cdt),
    Efid(Efid),
    Efpa(Efpa),
    Efna(Efna),
    Esid(Esid),
    Cd(Cd),
    Eerp(Eerp),
    Rtr(Rtr),
}

impl Default for OftpExchangeBuffer {
    fn default() -> Self {
        Self::None
    }
}

impl OftpExchangeBuffer {
    pub fn encode(&self) -> Result<Vec<u8>, PduEncodeError> {
        match self {
            Self::None => Err(PduEncodeError::EmptyBuffer),
            Self::Ssrm(ssrm) => {
                let mut ssrm = *ssrm;
                Ok(ssrm.encode(ssrm.cr)?)
            }
            Self::Ssid(ssid) => Ok(ssid.clone().encode()?),
            Self::Sfid(sfid) => Ok(sfid.encode()?),
            Self::Sfpa(sfpa) => Ok(sfpa.encode()?),
            Self::Sfna(sfna) => Ok(sfna.encode()?),
            Self::Data(data) => Ok(data.encode()?),
            Self::Cdt(cdt) => Ok(cdt.encode()?),
            Self::Efid(efid) => Ok(efid.encode()?),
            Self::Efpa(efpa) => Ok(efpa.encode()?),
            Self::Efna(efna) => Ok(efna.encode()?),
            Self::Esid(esid) => Ok(esid.encode()?),
            Self::Cd(cd) => Ok(cd.encode()?),
            Self::Eerp(eerp) => Ok(eerp.encode()?),
            Self::Rtr(rtr) => Ok(rtr.encode()?),
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
                let mut ssrm = SsrmPdu { cr: 0 };
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
            sfpa::SFPACMD => {
                let mut sfpa = Sfpa::default();
                sfpa.decode(buf)?;
                Ok(Self::Sfpa(sfpa))
            }
            sfna::SFNACMD => {
                let mut sfna = Sfna::default();
                sfna.decode(buf)?;
                Ok(Self::Sfna(sfna))
            }
            data::DATACMD => {
                let mut data = Data::default();
                data.decode(buf)?;
                Ok(Self::Data(data))
            }
            cdt::CDTCMD => {
                let mut cdt = Cdt;
                cdt.decode(buf)?;
                Ok(Self::Cdt(cdt))
            }
            efid::EFIDCMD => {
                let mut efid = Efid::default();
                efid.decode(buf)?;
                Ok(Self::Efid(efid))
            }
            efpa::EFPACMD => {
                let mut efpa = Efpa::default();
                efpa.decode(buf)?;
                Ok(Self::Efpa(efpa))
            }
            efna::EFNACMD => {
                let mut efna = Efna::default();
                efna.decode(buf)?;
                Ok(Self::Efna(efna))
            }
            esid::ESIDCMD => {
                let mut esid = Esid::default();
                esid.decode(buf)?;
                Ok(Self::Esid(esid))
            }
            cd::CDCMD => {
                let mut cd = Cd;
                cd.decode(buf)?;
                Ok(Self::Cd(cd))
            }
            eerp::EERPCMD => {
                let mut eerp = Eerp {
                    dsn: [b' '; 26],
                    date: *b"00000000",
                    time: *b"0000000000",
                    user_data: [b' '; 8],
                    dest: [b' '; 25],
                    orig: [b' '; 25],
                };
                eerp.decode(buf)?;
                Ok(Self::Eerp(eerp))
            }
            rtr::RTRCMD => {
                let mut rtr = Rtr;
                rtr.decode(buf)?;
                Ok(Self::Rtr(rtr))
            }
            _ => {
                let label = crate::debug::command_label(cmd);
                warn!(cmd, label, "commande OEB inconnue");
                Err(PduDecodeError::UnknownCommand { cmd, label })
            }
        }
    }

    /// Mappe un PDU reçu vers un [`InputEvent`] RFC.
    pub fn to_input_event(&self) -> Result<InputEvent, PduToEventError> {
        match self {
            Self::None => Err(PduToEventError::Empty),
            Self::Ssrm(_) => Ok(InputEvent::Ssrm),
            Self::Ssid(_) => Ok(InputEvent::Ssid),
            Self::Sfid(_) => Ok(InputEvent::Sfid),
            Self::Sfpa(_) => Ok(InputEvent::Sfpa),
            Self::Sfna(_) => Ok(InputEvent::Sfna),
            Self::Data(_) => Ok(InputEvent::Data),
            Self::Cdt(_) => Ok(InputEvent::Cdt),
            Self::Efid(_) => Ok(InputEvent::Efid),
            Self::Efpa(_) => Ok(InputEvent::Efpa),
            Self::Efna(_) => Ok(InputEvent::Efna),
            Self::Esid(_) => Ok(InputEvent::Esid),
            Self::Cd(_) => Ok(InputEvent::Cd),
            Self::Eerp(_) => Ok(InputEvent::Eerp),
            Self::Rtr(_) => Ok(InputEvent::Rtr),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PduToEventError {
    #[error("PDU vide")]
    Empty,
}
