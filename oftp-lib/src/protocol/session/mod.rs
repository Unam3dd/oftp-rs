//! Session OFTP — contexte FSM §9.6 / §9.8 (sans I/O réseau).

mod connection;

#[cfg(test)]
mod tests;

pub use super::config::{SessionConfig, SessionRole};
use super::error::ProtocolError;
use super::input::TransitionInput;
use super::output::TransitionOutput;
use super::state::ProtocolState;
use super::vars::SessionVars;

/// Contexte mutable de la FSM protocole pour une session TCP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub state: ProtocolState,
    pub role: SessionRole,
    pub config: SessionConfig,
    pub vars: SessionVars,
}

impl Session {
    /// Initiator après `N_CON_CF` — attend SSRM (`I_WF_RM`).
    pub fn initiator(config: SessionConfig) -> Self {
        Self {
            state: ProtocolState::InitWaitRm,
            role: SessionRole::Initiator,
            config,
            vars: SessionVars {
                caller: true,
                ..SessionVars::default()
            },
        }
    }

    /// Responder après `N_CON_IND` — prêt à émettre SSRM (transition **B**).
    pub fn responder(config: SessionConfig) -> Self {
        Self {
            state: ProtocolState::Idle,
            role: SessionRole::Responder,
            config,
            vars: SessionVars::default(),
        }
    }

    /// Transition **B** — Responder : `N_CON_IND` → SSRM → `RespNcOnly`.
    pub fn accept_connection(&mut self) -> Result<TransitionOutput, ProtocolError> {
        connection::accept_connection(self)
    }

    /// Transition **G** — Responder : `F_CONNECT_RS` depuis `RespWaitConRs`.
    pub fn confirm_partner(&mut self) -> Result<TransitionOutput, ProtocolError> {
        connection::confirm_partner(self)
    }

    /// Fin normale — ESID(00) depuis `IdleSp` / `IdleLi`.
    pub fn end_session(&mut self) -> Result<TransitionOutput, ProtocolError> {
        connection::end_session(self)
    }

    /// Consomme un événement (PDU pair, …) et retourne le résultat sans I/O.
    pub fn transition(
        &mut self,
        input: TransitionInput,
    ) -> Result<TransitionOutput, ProtocolError> {
        connection::transition(self, input)
    }
}
