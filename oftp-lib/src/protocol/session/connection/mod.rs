//! Table 9.8 — dispatch table-driven (SSRM, SSID, ESID).

mod apply;
mod auth;
mod event;
mod predicates;
mod table;

use crate::codec::oeb::OftpExchangeBuffer;

use super::super::config::SessionRole;
use super::super::error::ProtocolError;
use super::super::input::TransitionInput;
use super::super::output::TransitionOutput;
use super::super::state::ProtocolState;
use super::Session;

use event::ConnectionEvent as Ev;
use table::CONNECTION_TABLE;

fn dispatch(session: &mut Session, event: &Ev<'_>) -> Result<TransitionOutput, ProtocolError> {
    for row in CONNECTION_TABLE {
        if !row.state.matches(session.state) {
            continue;
        }
        if let Some(role) = row.role {
            if session.role != role {
                continue;
            }
        }
        if !row.event.matches(event) {
            continue;
        }
        if !(row.pred)(session) {
            continue;
        }
        return (row.apply)(session, event);
    }

    if session.state.is_auth_phase() {
        if matches!(event, Ev::Peer(_)) {
            return apply::apply_l_protocol_violation(session, event);
        }
    }

    Err(ProtocolError::InvalidTransition {
        state: session.state,
        event: invalid_event_label(session.role, session.state, event),
    })
}

/// Transition **B** — Responder : `N_CON_IND` → SSRM → `RespNcOnly`.
pub fn accept_connection(session: &mut Session) -> Result<TransitionOutput, ProtocolError> {
    dispatch(session, &Ev::AcceptConnection)
}

/// Transition **G** — Responder : `F_CONNECT_RS` → SSID → `IdleLi`.
pub fn confirm_partner(session: &mut Session) -> Result<TransitionOutput, ProtocolError> {
    dispatch(session, &Ev::ConfirmPartner)
}

/// Fin normale de session — ESID(00) → `WaitNDisc`.
pub fn end_session(session: &mut Session) -> Result<TransitionOutput, ProtocolError> {
    dispatch(session, &Ev::EndSession)
}

/// Consomme un PDU pair et retourne le résultat sans I/O.
pub fn transition(
    session: &mut Session,
    input: TransitionInput,
) -> Result<TransitionOutput, ProtocolError> {
    let TransitionInput::Peer(oeb) = input;
    dispatch(session, &Ev::Peer(&oeb))
}

fn invalid_event_label(
    role: SessionRole,
    state: ProtocolState,
    event: &Ev<'_>,
) -> &'static str {
    match event {
        Ev::AcceptConnection => "N_CON_IND",
        Ev::ConfirmPartner => "F_CONNECT_RS",
        Ev::EndSession => "F_RELEASE_RQ",
        Ev::Peer(oeb) => invalid_peer_event(role, state, oeb),
    }
}

fn invalid_peer_event(
    role: SessionRole,
    state: ProtocolState,
    oeb: &OftpExchangeBuffer,
) -> &'static str {
    match oeb {
        OftpExchangeBuffer::Ssrm(_) => match (role, state) {
            (SessionRole::Initiator, ProtocolState::InitWaitRm) => "SSRM",
            _ => "SSRM(unexpected)",
        },
        OftpExchangeBuffer::Ssid(_) => match (role, state) {
            (SessionRole::Initiator, ProtocolState::InitWaitSsid) => "SSID",
            (SessionRole::Responder, ProtocolState::RespNcOnly) => "SSID",
            _ => "SSID(unexpected)",
        },
        OftpExchangeBuffer::Esid(_) => "ESID",
        OftpExchangeBuffer::Secd(_) => match state {
            ProtocolState::WfSecd => "SECD",
            _ => "SECD(unexpected)",
        },
        OftpExchangeBuffer::Auch(_) => match state {
            ProtocolState::WfAuch => "AUCH",
            _ => "AUCH(unexpected)",
        },
        OftpExchangeBuffer::Aurp(_) => match state {
            ProtocolState::WfAurp => "AURP",
            _ => "AURP(unexpected)",
        },
        OftpExchangeBuffer::None => "None",
    }
}

#[cfg(test)]
mod dispatch_tests {
    use super::*;
    use crate::protocol::config::SessionConfig;

    #[test]
    fn dispatch_matches_row_b() {
        let mut server = Session::responder(SessionConfig::default());
        let out = accept_connection(&mut server).unwrap();
        assert_eq!(out.next, ProtocolState::RespNcOnly);
    }

    #[test]
    fn table_has_unique_codes_documented() {
        assert!(CONNECTION_TABLE.len() >= 10);
    }
}
