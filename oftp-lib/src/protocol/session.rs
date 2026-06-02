//! Session OFTP — contexte FSM §9.6 / §9.8 (sans I/O réseau).

use crate::codec::pdu::ssid::{Ssid, SsidFieldError};

use super::error::ProtocolError;
use super::input::TransitionInput;
use super::output::TransitionOutput;
use super::state::ProtocolState;

/// Rôle TCP de l'entité (Initiator = client, Responder = serveur).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRole {
    Initiator,
    Responder,
}

/// Capacités locales et identité — sert à construire le SSID émis (§9.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionConfig {
    /// Template SSID local (code, password, buffer_size, mode, …).
    pub local_ssid: Ssid,
    /// Responder : accepter le partenaire sans pause app (transitions E+G fusionnées).
    pub auto_accept: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            local_ssid: Ssid::default(),
            auto_accept: true,
        }
    }
}

impl SessionConfig {
    pub fn with_code(mut self, code: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_code(code)?;
        Ok(self)
    }

    pub fn with_password(mut self, password: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_password(password)?;
        Ok(self)
    }

    /// SSID prêt à encoder pour l'émission.
    pub fn to_ssid(&self) -> Ssid {
        self.local_ssid.clone()
    }
}

/// Variables de session négociées (§9.6) — remplies au fil des transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionVars {
    pub buf_size: u32,
    pub window: u16,
    pub credit_s: u16,
    pub credit_l: u16,
    /// `true` si Initiator (caller).
    pub caller: bool,
    pub compression: bool,
    pub restart: bool,
    pub authentication: bool,
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
            compression: false,
            restart: false,
            authentication: false,
            peer_ssid: None,
        }
    }
}

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

    /// Consomme un événement et retourne le résultat sans I/O.
    pub fn transition(
        &mut self,
        _input: TransitionInput,
    ) -> Result<TransitionOutput, ProtocolError> {
        Err(ProtocolError::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiator_starts_in_init_wait_rm() {
        let session = Session::initiator(SessionConfig::default());
        assert_eq!(session.state, ProtocolState::InitWaitRm);
        assert_eq!(session.role, SessionRole::Initiator);
        assert!(session.vars.caller);
    }

    #[test]
    fn responder_starts_idle() {
        let session = Session::responder(SessionConfig::default());
        assert_eq!(session.state, ProtocolState::Idle);
        assert_eq!(session.role, SessionRole::Responder);
        assert!(!session.vars.caller);
    }

    #[test]
    fn config_with_code() {
        let config = SessionConfig::default().with_code("CLIENT01").unwrap();
        let code = std::str::from_utf8(&config.local_ssid.code)
            .unwrap()
            .trim_end();
        assert_eq!(code, "CLIENT01");
    }

    #[test]
    fn transition_not_implemented_yet() {
        use crate::codec::oeb::OftpExchangeBuffer;
        use crate::codec::pdu::ssrm::Ssrm;

        let mut session = Session::initiator(SessionConfig::default());
        let input = TransitionInput::Peer(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }));
        assert_eq!(
            session.transition(input).unwrap_err(),
            ProtocolError::NotImplemented
        );
    }
}
