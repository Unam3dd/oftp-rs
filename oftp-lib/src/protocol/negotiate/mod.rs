//! Négociation SSID — RFC §5.3.2, action 5 table 9.8.
//!
//! Compare le SSID **local** (déjà émis) et le SSID **peer** (reçu), remplit [`NegotiatedParams`]
//! ou retourne une erreur convertible en ESID.
//!
//! ```text
//! negotiate_ssid(input)
//!   1. cap_mode     — P4 Responder : le mode du peer est-il accepté par notre site ?
//!   2. partner      — code / mot de passe du peer (config app)
//!   3. level        — SSIDLEV identiques et supportés
//!   4. auth         — SSIDAUTH identiques (Y/Y ou N/N)
//!   5. transport    — SSIDSPEC : pas de Y sur TCP
//!   6. mode         — SSIDSR : pas S+S ni R+R
//!   7. merge        — min buffer, min credit, AND compression/restart
//! ```

mod partner;

use crate::codec::pdu::esid::EsidReason;
use crate::codec::pdu::ssid::{ProtocolLevel, Ssid, SsidMode};

use super::vars::SessionVars;

pub use partner::PartnerPolicy;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Transport {
    #[default]
    Tcp,
    X25,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegotiationInput<'a> {
    pub local: &'a Ssid,
    pub peer: &'a Ssid,
    pub cap_mode: SsidMode,
    /// `true` sur transition E (Responder) : applique P4.
    pub check_cap_mode: bool,
    pub transport: Transport,
    pub partner_policy: &'a PartnerPolicy,
    pub supported_levels: &'a [ProtocolLevel],
}

impl<'a> NegotiationInput<'a> {
    pub fn for_initiator(local: &'a Ssid, peer: &'a Ssid, policy: &'a PartnerPolicy) -> Self {
        Self {
            local,
            peer,
            cap_mode: SsidMode::Both,
            check_cap_mode: false,
            transport: Transport::Tcp,
            partner_policy: policy,
            supported_levels: &[ProtocolLevel::Rev20],
        }
    }

    pub fn for_responder(
        local: &'a Ssid,
        peer: &'a Ssid,
        cap_mode: SsidMode,
        policy: &'a PartnerPolicy,
    ) -> Self {
        Self {
            local,
            peer,
            cap_mode,
            check_cap_mode: true,
            transport: Transport::Tcp,
            partner_policy: policy,
            supported_levels: &[ProtocolLevel::Rev20],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegotiatedParams {
    pub peer_ssid: Ssid,
    pub level: ProtocolLevel,
    pub mode: SsidMode,
    pub buf_size: u32,
    pub window: u16,
    pub compression: bool,
    pub restart: bool,
    pub special_logic: bool,
    pub authentication: bool,
}

impl NegotiatedParams {
    pub fn apply_to(&self, vars: &mut SessionVars) {
        vars.peer_ssid = Some(self.peer_ssid.clone());
        vars.level = self.level;
        vars.mode = self.mode;
        vars.buf_size = self.buf_size;
        vars.window = self.window;
        vars.credit_s = self.window;
        vars.credit_l = self.window;
        vars.compression = self.compression;
        vars.restart = self.restart;
        vars.special_logic = self.special_logic;
        vars.authentication = self.authentication;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsidNegotiationError {
    ModeIncompatible,
    AuthMismatch,
    ProtocolLevelIncompatible,
    SpecialLogicNotSupported,
    PartnerRejected(EsidReason),
}

impl SsidNegotiationError {
    pub fn to_esid_reason(self) -> EsidReason {
        match self {
            Self::ModeIncompatible | Self::ProtocolLevelIncompatible | Self::SpecialLogicNotSupported => {
                EsidReason::ModeOrCapabilitiesIncompatible
            }
            Self::AuthMismatch => EsidReason::SecureAuthRequirementsIncompatible,
            Self::PartnerRejected(reason) => reason,
        }
    }
}

// ---------------------------------------------------------------------------
// Entrée
// ---------------------------------------------------------------------------

pub fn negotiate_ssid(
    input: &NegotiationInput<'_>,
) -> Result<NegotiatedParams, SsidNegotiationError> {
    if input.check_cap_mode && !peer_mode_compatible_with_cap(input.peer.mode, input.cap_mode) {
        return Err(SsidNegotiationError::ModeIncompatible);
    }

    input
        .partner_policy
        .validate_peer(input.peer)
        .map_err(SsidNegotiationError::PartnerRejected)?;

    check_level(input)?;
    check_auth(input)?;
    check_special_logic(input)?;
    let mode = check_mode(input)?;
    Ok(merge_options(input, mode))
}

/// P4 — utilisé aussi par la FSM (`apply_e`) avant d'appeler `negotiate_ssid`.
pub fn peer_mode_compatible_with_cap(peer_mode: SsidMode, cap_mode: SsidMode) -> bool {
    match cap_mode {
        SsidMode::Both => true,
        SsidMode::SendOnly => !matches!(peer_mode, SsidMode::SendOnly),
        SsidMode::ReceiveOnly => !matches!(peer_mode, SsidMode::ReceiveOnly),
    }
}

// ---------------------------------------------------------------------------
// Vérifications (privées, dans l'ordre d'appel ci-dessus)
// ---------------------------------------------------------------------------

fn check_level(input: &NegotiationInput<'_>) -> Result<(), SsidNegotiationError> {
    if input.local.level != input.peer.level {
        return Err(SsidNegotiationError::ProtocolLevelIncompatible);
    }
    if input.supported_levels.contains(&input.local.level) {
        Ok(())
    } else {
        Err(SsidNegotiationError::ProtocolLevelIncompatible)
    }
}

fn check_auth(input: &NegotiationInput<'_>) -> Result<(), SsidNegotiationError> {
    if input.local.auth == input.peer.auth {
        Ok(())
    } else {
        Err(SsidNegotiationError::AuthMismatch)
    }
}

fn check_special_logic(input: &NegotiationInput<'_>) -> Result<(), SsidNegotiationError> {
    if input.transport == Transport::Tcp && (input.local.special_logic || input.peer.special_logic) {
        return Err(SsidNegotiationError::SpecialLogicNotSupported);
    }
    Ok(())
}

fn check_mode(input: &NegotiationInput<'_>) -> Result<SsidMode, SsidNegotiationError> {
    match (input.local.mode, input.peer.mode) {
        (SsidMode::SendOnly, SsidMode::SendOnly) | (SsidMode::ReceiveOnly, SsidMode::ReceiveOnly) => {
            Err(SsidNegotiationError::ModeIncompatible)
        }
        (SsidMode::SendOnly, SsidMode::ReceiveOnly)
        | (SsidMode::ReceiveOnly, SsidMode::SendOnly) => Ok(SsidMode::Both),
        (SsidMode::Both, _) | (_, SsidMode::Both) => Ok(SsidMode::Both),
    }
}

fn merge_options(input: &NegotiationInput<'_>, mode: SsidMode) -> NegotiatedParams {
    NegotiatedParams {
        peer_ssid: input.peer.clone(),
        level: input.local.level,
        mode,
        buf_size: input.local.buffer_size.min(input.peer.buffer_size),
        window: input.local.credit.min(input.peer.credit),
        compression: input.local.compression && input.peer.compression,
        restart: input.local.restart && input.peer.restart,
        special_logic: input.local.special_logic && input.peer.special_logic,
        authentication: input.local.auth,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::vars::SessionVars;

    #[test]
    fn happy_path_defaults() {
        let local = Ssid::default();
        let peer = Ssid::default();
        let policy = PartnerPolicy::AcceptAny;
        let params = negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        assert_eq!(params.buf_size, 2048);
        assert_eq!(params.window, 50);
        assert_eq!(params.mode, SsidMode::Both);
    }

    #[test]
    fn reject_send_send() {
        let mut local = Ssid::default();
        local.mode = SsidMode::SendOnly;
        let mut peer = Ssid::default();
        peer.mode = SsidMode::SendOnly;
        let policy = PartnerPolicy::AcceptAny;
        assert_eq!(
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap_err(),
            SsidNegotiationError::ModeIncompatible
        );
    }

    #[test]
    fn min_buffer_and_credit() {
        let mut local = Ssid::default();
        local.buffer_size = 4096;
        local.credit = 100;
        let mut peer = Ssid::default();
        peer.buffer_size = 2048;
        peer.credit = 50;
        let policy = PartnerPolicy::AcceptAny;
        let params = negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        assert_eq!(params.buf_size, 2048);
        assert_eq!(params.window, 50);
    }

    #[test]
    fn apply_to_session_vars() {
        let local = Ssid::default();
        let peer = Ssid::default();
        let policy = PartnerPolicy::AcceptAny;
        let params =
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        let mut vars = SessionVars::default();
        params.apply_to(&mut vars);
        assert_eq!(vars.buf_size, 2048);
        assert!(vars.peer_ssid.is_some());
    }

    #[test]
    fn complementary_modes_yield_both() {
        let mut local = Ssid::default();
        local.mode = SsidMode::SendOnly;
        let mut peer = Ssid::default();
        peer.mode = SsidMode::ReceiveOnly;
        let policy = PartnerPolicy::AcceptAny;
        let params =
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        assert_eq!(params.mode, SsidMode::Both);
    }

    #[test]
    fn compression_requires_both_y() {
        let mut local = Ssid::default();
        local.compression = true;
        let mut peer = Ssid::default();
        peer.compression = false;
        let policy = PartnerPolicy::AcceptAny;
        let params =
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        assert!(!params.compression);

        peer.compression = true;
        let params =
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap();
        assert!(params.compression);
    }

    #[test]
    fn auth_mismatch_fails() {
        let mut local = Ssid::default();
        local.auth = true;
        let mut peer = Ssid::default();
        peer.auth = false;
        let policy = PartnerPolicy::AcceptAny;
        assert_eq!(
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap_err(),
            SsidNegotiationError::AuthMismatch
        );
    }

    #[test]
    fn cap_mode_p4_rejects_incompatible_peer() {
        let local = Ssid::default();
        let mut peer = Ssid::default();
        peer.mode = SsidMode::SendOnly;
        let policy = PartnerPolicy::AcceptAny;
        let input = NegotiationInput::for_responder(
            &local,
            &peer,
            SsidMode::SendOnly,
            &policy,
        );
        assert_eq!(
            negotiate_ssid(&input).unwrap_err(),
            SsidNegotiationError::ModeIncompatible
        );
    }

    #[test]
    fn partner_policy_rejects_unknown_code() {
        let local = Ssid::default();
        let mut peer = Ssid::default();
        peer.set_code("UNKNOWN").unwrap();
        let policy = PartnerPolicy::RequireCode("EXPECTED".into());
        assert!(matches!(
            negotiate_ssid(&NegotiationInput::for_initiator(&local, &peer, &policy)).unwrap_err(),
            SsidNegotiationError::PartnerRejected(EsidReason::UserCodeNotKnown)
        ));
    }
}
