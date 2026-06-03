//! Lignes §9.8.2 — ordre : cas spécifiques avant fallbacks.

use crate::protocol::config::SessionRole;
use crate::protocol::state::ProtocolState;

use super::apply::{
    apply_auth_auch_i, apply_auth_aurp_k, apply_auth_secd_j, apply_b, apply_d, apply_e, apply_e_g,
    apply_f_abort, apply_g, apply_h, apply_peer_release, apply_release, apply_release_complete,
    ApplyFn,
};
use super::event::{EventFilter, PeerKind};
use super::predicates::{always, auto_accept, not_auto_accept, Pred};

/// Filtre d'état : exact ou ensemble (ex. `IdleSp` | `IdleLi`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateFilter {
    Exact(ProtocolState),
    AnyOf(&'static [ProtocolState]),
}

impl StateFilter {
    pub fn matches(&self, state: ProtocolState) -> bool {
        match self {
            Self::Exact(s) => state == *s,
            Self::AnyOf(states) => states.contains(&state),
        }
    }
}

/// Une ligne de la table 9.8 (sous-ensemble implémenté).
pub struct ConnectionRow {
    /// Code RFC §9.8.2 (`'B'`, `'H'`, …) — documentation / debug.
    #[allow(dead_code)]
    pub code: char,
    pub state: StateFilter,
    pub role: Option<SessionRole>,
    pub event: EventFilter,
    pub pred: Pred,
    pub apply: ApplyFn,
}

pub const CONNECTION_TABLE: &[ConnectionRow] = &[
    // **B** — `N_CON_IND`, IDLE, Responder
    ConnectionRow {
        code: 'B',
        state: StateFilter::Exact(ProtocolState::Idle),
        role: Some(SessionRole::Responder),
        event: EventFilter::AcceptConnection,
        pred: always,
        apply: apply_b,
    },
    // **G** — `F_CONNECT_RS`, A_WF_CONRS, Responder
    ConnectionRow {
        code: 'G',
        state: StateFilter::Exact(ProtocolState::RespWaitConRs),
        role: Some(SessionRole::Responder),
        event: EventFilter::ConfirmPartner,
        pred: always,
        apply: apply_g,
    },
    // Fin session locale — `F_RELEASE_RQ`
    ConnectionRow {
        code: 'E',
        state: StateFilter::AnyOf(&[ProtocolState::IdleSp, ProtocolState::IdleLi]),
        role: None,
        event: EventFilter::EndSession,
        pred: always,
        apply: apply_release,
    },
    // **H** — SSRM, I_WF_RM, Initiator
    ConnectionRow {
        code: 'H',
        state: StateFilter::Exact(ProtocolState::InitWaitRm),
        role: Some(SessionRole::Initiator),
        event: EventFilter::Peer(PeerKind::Ssrm),
        pred: always,
        apply: apply_h,
    },
    // **D** — SSID, I_WF_SSID, Initiator
    ConnectionRow {
        code: 'D',
        state: StateFilter::Exact(ProtocolState::InitWaitSsid),
        role: Some(SessionRole::Initiator),
        event: EventFilter::Peer(PeerKind::Ssid),
        pred: always,
        apply: apply_d,
    },
    // **J** — SECD, WF_SECD
    ConnectionRow {
        code: 'J',
        state: StateFilter::Exact(ProtocolState::WfSecd),
        role: None,
        event: EventFilter::Peer(PeerKind::Secd),
        pred: always,
        apply: apply_auth_secd_j,
    },
    // **I** — AUCH, WF_AUCH
    ConnectionRow {
        code: 'I',
        state: StateFilter::Exact(ProtocolState::WfAuch),
        role: None,
        event: EventFilter::Peer(PeerKind::Auch),
        pred: always,
        apply: apply_auth_auch_i,
    },
    // **K** — AURP, WF_AURP
    ConnectionRow {
        code: 'K',
        state: StateFilter::Exact(ProtocolState::WfAurp),
        role: None,
        event: EventFilter::Peer(PeerKind::Aurp),
        pred: always,
        apply: apply_auth_aurp_k,
    },
    // **F** — ESID en phase auth
    ConnectionRow {
        code: 'F',
        state: StateFilter::AnyOf(&[
            ProtocolState::WfSecd,
            ProtocolState::WfAuch,
            ProtocolState::WfAurp,
        ]),
        role: None,
        event: EventFilter::Peer(PeerKind::Esid),
        pred: always,
        apply: apply_f_abort,
    },
    // **F** — ESID, I_WF_SSID, Initiator
    ConnectionRow {
        code: 'F',
        state: StateFilter::Exact(ProtocolState::InitWaitSsid),
        role: Some(SessionRole::Initiator),
        event: EventFilter::Peer(PeerKind::Esid),
        pred: always,
        apply: apply_f_abort,
    },
    // **E+G** — SSID, A_NC_ONLY, auto_accept
    ConnectionRow {
        code: 'G',
        state: StateFilter::Exact(ProtocolState::RespNcOnly),
        role: Some(SessionRole::Responder),
        event: EventFilter::Peer(PeerKind::Ssid),
        pred: auto_accept,
        apply: apply_e_g,
    },
    // **E** — SSID, A_NC_ONLY, ¬auto_accept
    ConnectionRow {
        code: 'E',
        state: StateFilter::Exact(ProtocolState::RespNcOnly),
        role: Some(SessionRole::Responder),
        event: EventFilter::Peer(PeerKind::Ssid),
        pred: not_auto_accept,
        apply: apply_e,
    },
    // ESID pair en session établie
    ConnectionRow {
        code: 'E',
        state: StateFilter::AnyOf(&[ProtocolState::IdleSp, ProtocolState::IdleLi]),
        role: None,
        event: EventFilter::Peer(PeerKind::Esid),
        pred: always,
        apply: apply_peer_release,
    },
    // ESID pair après notre ESID
    ConnectionRow {
        code: 'F',
        state: StateFilter::Exact(ProtocolState::WaitNDisc),
        role: None,
        event: EventFilter::Peer(PeerKind::Esid),
        pred: always,
        apply: apply_release_complete,
    },
];
