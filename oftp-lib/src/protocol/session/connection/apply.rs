//! Actions §9.8 — effets de transition (état, variables, PDU sortantes).

use crate::codec::oeb::OftpExchangeBuffer;
use crate::codec::pdu::auch::Auch;
use crate::codec::pdu::esid::{Esid, EsidReason};
use crate::codec::pdu::secd::Secd;
use crate::codec::pdu::ssrm::Ssrm;
use super::auth::{
    new_auth_challenge, outbound_aurp_from_stored_challenge, store_auth_challenge,
    verify_aurp_response,
};
use super::super::Session;
use super::event::ConnectionEvent;
use crate::protocol::negotiate::{
    negotiate_ssid, NegotiationInput, SsidNegotiationError,
};
use crate::protocol::negotiate::peer_mode_compatible_with_cap;
use crate::protocol::output::TransitionOutput;
use crate::protocol::state::ProtocolState;
use crate::protocol::ProtocolError;

pub type ApplyFn = for<'a> fn(&mut Session, &ConnectionEvent<'a>) -> Result<TransitionOutput, ProtocolError>;

/// **B** — SSRM → `RespNcOnly`.
pub fn apply_b(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::RespNcOnly;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D })),
    })
}

/// **G** — SSID local → `IdleLi` ou `WfSecd` si auth.
pub fn apply_g(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = if session.vars.authentication {
        ProtocolState::WfSecd
    } else {
        ProtocolState::IdleLi
    };
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Ssid(session.config.to_ssid())),
    })
}

/// Fin normale — ESID(00) → `WaitNDisc`.
pub fn apply_release(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::WaitNDisc;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Esid(Esid::normal())),
    })
}

/// **H** — SSID local → `InitWaitSsid`.
pub fn apply_h(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::InitWaitSsid;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Ssid(session.config.to_ssid())),
    })
}

/// **D** — négociation SSID initiator → `IdleSp` ou **SECD** + `WfAuch`.
pub fn apply_d(session: &mut Session, event: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let ConnectionEvent::Peer(OftpExchangeBuffer::Ssid(peer)) = event else {
        return Err(ProtocolError::NotImplemented);
    };
    let local = session.config.local_ssid.clone();
    let policy = session.config.partner_policy.clone();
    let input = NegotiationInput::for_initiator(&local, peer, &policy);
    match finish_ssid_negotiation(session, input) {
        Ok(()) => {
            if session.vars.authentication {
                let next = ProtocolState::WfAuch;
                session.state = next;
                Ok(TransitionOutput {
                    next,
                    to_peer: Some(OftpExchangeBuffer::Secd(Secd)),
                })
            } else {
                let next = ProtocolState::IdleSp;
                session.state = next;
                Ok(TransitionOutput::state_only(next))
            }
        }
        Err(out) => Ok(out),
    }
}

/// **F** — ESID reçu pendant handshake → `Idle`.
pub fn apply_f_abort(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::Idle;
    session.state = next;
    Ok(TransitionOutput::state_only(next))
}

/// **L** — violation protocole en phase auth → ESID(02) + `WaitNDisc`.
pub fn apply_l_protocol_violation(
    session: &mut Session,
    _: &ConnectionEvent<'_>,
) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::WaitNDisc;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Esid(Esid::with_reason(
            EsidReason::ProtocolViolation,
        ))),
    })
}

/// **J** — `WfSecd` + SECD → **AUCH** → `WfAurp`.
pub fn apply_auth_secd_j(
    session: &mut Session,
    _: &ConnectionEvent<'_>,
) -> Result<TransitionOutput, ProtocolError> {
    let challenge = new_auth_challenge();
    store_auth_challenge(session, &challenge);
    let next = ProtocolState::WfAurp;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Auch(Auch::with_challenge(challenge))),
    })
}

/// **I** — `WfAuch` + AUCH → **AURP** → `WfSecd` (caller) ou `IdleLi`.
pub fn apply_auth_auch_i(
    session: &mut Session,
    event: &ConnectionEvent<'_>,
) -> Result<TransitionOutput, ProtocolError> {
    let ConnectionEvent::Peer(OftpExchangeBuffer::Auch(auch)) = event else {
        return Err(ProtocolError::NotImplemented);
    };
    store_auth_challenge(session, &auch.challenge);
    let aurp = outbound_aurp_from_stored_challenge(session)
        .ok_or(ProtocolError::NotImplemented)?;
    let next = if session.vars.caller {
        ProtocolState::WfSecd
    } else {
        ProtocolState::IdleLi
    };
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Aurp(aurp)),
    })
}

/// **K** — `WfAurp` + AURP → `IdleSp` / tour inverse **SECD** / ESID(11).
pub fn apply_auth_aurp_k(
    session: &mut Session,
    event: &ConnectionEvent<'_>,
) -> Result<TransitionOutput, ProtocolError> {
    let ConnectionEvent::Peer(OftpExchangeBuffer::Aurp(aurp)) = event else {
        return Err(ProtocolError::NotImplemented);
    };
    if !verify_aurp_response(session, aurp) {
        let out = esid_reason_output(EsidReason::InvalidChallengeResponse);
        session.state = out.next;
        return Ok(out);
    }
    if session.vars.caller {
        let next = ProtocolState::IdleSp;
        session.state = next;
        Ok(TransitionOutput::state_only(next))
    } else {
        let challenge = new_auth_challenge();
        store_auth_challenge(session, &challenge);
        let next = ProtocolState::WfAuch;
        session.state = next;
        Ok(TransitionOutput {
            next,
            to_peer: Some(OftpExchangeBuffer::Secd(Secd)),
        })
    }
}

/// **E** — SSID pair stocké, attente `F_CONNECT_RS` (¬auto_accept).
pub fn apply_e(session: &mut Session, event: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let ConnectionEvent::Peer(OftpExchangeBuffer::Ssid(peer)) = event else {
        return Err(ProtocolError::NotImplemented);
    };
    if !peer_mode_compatible_with_cap(peer.mode, session.config.cap_mode) {
        session.state = ProtocolState::Idle;
        return Ok(TransitionOutput::state_only(ProtocolState::Idle));
    }
    let local = session.config.local_ssid.clone();
    let policy = session.config.partner_policy.clone();
    let cap_mode = session.config.cap_mode;
    let input = NegotiationInput::for_responder(&local, peer, cap_mode, &policy);
    match finish_ssid_negotiation(session, input) {
        Ok(()) => {
            let next = ProtocolState::RespWaitConRs;
            session.state = next;
            Ok(TransitionOutput::state_only(next))
        }
        Err(out) => Ok(out),
    }
}

/// **E+G** — négociation + SSID local → `IdleLi` ou `WfSecd`.
pub fn apply_e_g(session: &mut Session, event: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let ConnectionEvent::Peer(OftpExchangeBuffer::Ssid(peer)) = event else {
        return Err(ProtocolError::NotImplemented);
    };
    if !peer_mode_compatible_with_cap(peer.mode, session.config.cap_mode) {
        session.state = ProtocolState::Idle;
        return Ok(TransitionOutput::state_only(ProtocolState::Idle));
    }
    let local = session.config.local_ssid.clone();
    let policy = session.config.partner_policy.clone();
    let cap_mode = session.config.cap_mode;
    let input = NegotiationInput::for_responder(&local, peer, cap_mode, &policy);
    match finish_ssid_negotiation(session, input) {
        Ok(()) => {
            let next = if session.vars.authentication {
                ProtocolState::WfSecd
            } else {
                ProtocolState::IdleLi
            };
            session.state = next;
            Ok(TransitionOutput {
                next,
                to_peer: Some(OftpExchangeBuffer::Ssid(session.config.to_ssid())),
            })
        }
        Err(out) => Ok(out),
    }
}

/// ESID pair en session établie → répondre ESID, `WaitNDisc`.
pub fn apply_peer_release(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::WaitNDisc;
    session.state = next;
    Ok(TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Esid(Esid::normal())),
    })
}

/// ESID pair après notre ESID → `Idle`.
pub fn apply_release_complete(session: &mut Session, _: &ConnectionEvent<'_>) -> Result<TransitionOutput, ProtocolError> {
    let next = ProtocolState::Idle;
    session.state = next;
    Ok(TransitionOutput::state_only(next))
}

/// Négociation SSID (action 5) — met à jour `session.vars` ; l'appelant fixe l'état suivant.
fn finish_ssid_negotiation(
    session: &mut Session,
    input: NegotiationInput<'_>,
) -> Result<(), TransitionOutput> {
    match negotiate_ssid(&input) {
        Ok(params) => {
            params.apply_to(&mut session.vars);
            Ok(())
        }
        Err(err) => {
            let out = esid_abort_output(err);
            session.state = out.next;
            Err(out)
        }
    }
}

/// P2 échoué — ESID + `WaitNDisc` (branches D/E else table 9.8).
fn esid_abort_output(err: SsidNegotiationError) -> TransitionOutput {
    let next = ProtocolState::WaitNDisc;
    TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Esid(Esid::with_reason(err.to_esid_reason()))),
    }
}

fn esid_reason_output(reason: EsidReason) -> TransitionOutput {
    let next = ProtocolState::WaitNDisc;
    TransitionOutput {
        next,
        to_peer: Some(OftpExchangeBuffer::Esid(Esid::with_reason(reason))),
    }
}
