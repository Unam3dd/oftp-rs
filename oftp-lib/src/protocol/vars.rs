//! Variables de session négociées (§9.6).

use crate::codec::pdu::ssid::{ProtocolLevel, Ssid, SsidMode};

/// Variables de session négociées (§9.6) — remplies par [`super::negotiate::NegotiatedParams::apply_to`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionVars {
    pub buf_size: u32,
    pub window: u16,
    pub credit_s: u16,
    pub credit_l: u16,
    /// `true` si Initiator (caller).
    pub caller: bool,
    pub level: ProtocolLevel,
    pub mode: SsidMode,
    pub compression: bool,
    pub restart: bool,
    pub special_logic: bool,
    pub authentication: bool,
    /// Défi du tour auth en cours (`V.Challenge` — stub ou CMS).
    pub auth_challenge: Option<Vec<u8>>,
    /// SSID reçu du partenaire, une fois négocié.
    pub peer_ssid: Option<Ssid>,
}

impl Default for SessionVars {
    fn default() -> Self {
        Self {
            buf_size: 0,
            window: 0,
            credit_s: 0,
            credit_l: 0,
            caller: false,
            level: ProtocolLevel::Rev20,
            mode: SsidMode::Both,
            compression: false,
            restart: false,
            special_logic: false,
            authentication: false,
            auth_challenge: None,
            peer_ssid: None,
        }
    }
}
