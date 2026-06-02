//! Session OFTP — contexte FSM §9.6 / §9.8 (sans I/O réseau).

use crate::codec::oeb::OftpExchangeBuffer;
use crate::codec::pdu::ssrm::Ssrm;
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

    /// Transition **B** — Responder : `N_CON_IND` → SSRM → `RespNcOnly`.
    pub fn accept_connection(&mut self) -> Result<TransitionOutput, ProtocolError> {
        if self.role != SessionRole::Responder {
            return Err(ProtocolError::InvalidTransition {
                state: self.state,
                event: "N_CON_IND",
            });
        }
        if self.state != ProtocolState::Idle {
            return Err(ProtocolError::InvalidTransition {
                state: self.state,
                event: "N_CON_IND",
            });
        }

        let next = ProtocolState::RespNcOnly;
        self.state = next;
        Ok(TransitionOutput {
            next,
            to_peer: Some(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D })),
        })
    }

    /// Consomme un PDU pair et retourne le résultat sans I/O.
    pub fn transition(
        &mut self,
        input: TransitionInput,
    ) -> Result<TransitionOutput, ProtocolError> {
        let TransitionInput::Peer(oeb) = input;

        match (self.role, self.state, oeb) {
            // **H** — Initiator : SSRM reçu → SSID émis.
            (
                SessionRole::Initiator,
                ProtocolState::InitWaitRm,
                OftpExchangeBuffer::Ssrm(_),
            ) => {
                let next = ProtocolState::InitWaitSsid;
                self.state = next;
                Ok(TransitionOutput {
                    next,
                    to_peer: Some(OftpExchangeBuffer::Ssid(self.config.to_ssid())),
                })
            }

            // **D** — Initiator : SSID pair reçu → session établie (Speaker).
            (
                SessionRole::Initiator,
                ProtocolState::InitWaitSsid,
                OftpExchangeBuffer::Ssid(peer),
            ) => {
                apply_peer_ssid(self, &peer);
                let next = ProtocolState::IdleSp;
                self.state = next;
                Ok(TransitionOutput::state_only(next))
            }

            // **E+G** — Responder (auto_accept) : SSID Initiator → SSID émis → Listener.
            (
                SessionRole::Responder,
                ProtocolState::RespNcOnly,
                OftpExchangeBuffer::Ssid(peer),
            ) if self.config.auto_accept => {
                apply_peer_ssid(self, &peer);
                let next = ProtocolState::IdleLi;
                self.state = next;
                Ok(TransitionOutput {
                    next,
                    to_peer: Some(OftpExchangeBuffer::Ssid(self.config.to_ssid())),
                })
            }

            // **E** seule — Responder sans auto_accept : attend validation app.
            (
                SessionRole::Responder,
                ProtocolState::RespNcOnly,
                OftpExchangeBuffer::Ssid(peer),
            ) => {
                apply_peer_ssid(self, &peer);
                let next = ProtocolState::RespWaitConRs;
                self.state = next;
                Ok(TransitionOutput::state_only(next))
            }

            (role, state, oeb) => Err(ProtocolError::InvalidTransition {
                state,
                event: invalid_peer_event(role, state, &oeb),
            }),
        }
    }
}

/// Négociation minimale SSID (min buffer_size, min credit, flags AND).
fn apply_peer_ssid(session: &mut Session, peer: &Ssid) {
    let local = &session.config.local_ssid;

    session.vars.peer_ssid = Some(peer.clone());
    session.vars.buf_size = local.buffer_size.min(peer.buffer_size);
    session.vars.window = local.credit.min(peer.credit);
    session.vars.compression = local.compression && peer.compression;
    session.vars.restart = local.restart && peer.restart;
    session.vars.authentication = local.auth && peer.auth;
    session.vars.credit_s = session.vars.window;
    session.vars.credit_l = session.vars.window;
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
        OftpExchangeBuffer::None => "None",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ssid_code(ssid: &Ssid) -> &str {
        std::str::from_utf8(&ssid.code).unwrap().trim_end()
    }

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
        assert_eq!(ssid_code(&config.local_ssid), "CLIENT01");
    }

    #[test]
    fn transition_b_send_ssrm() {
        let mut server = Session::responder(SessionConfig::default());

        let out = server.accept_connection().unwrap();

        assert_eq!(server.state, ProtocolState::RespNcOnly);
        assert_eq!(out.next, ProtocolState::RespNcOnly);
        assert!(matches!(out.to_peer, Some(OftpExchangeBuffer::Ssrm(_))));
    }

    #[test]
    fn accept_connection_rejects_initiator() {
        let mut client = Session::initiator(SessionConfig::default());
        assert!(matches!(
            client.accept_connection().unwrap_err(),
            ProtocolError::InvalidTransition {
                state: ProtocolState::InitWaitRm,
                event: "N_CON_IND",
            }
        ));
    }

    #[test]
    fn handshake_fsm_ssrm_and_two_ssid() {
        let server_config = SessionConfig::default().with_code("SERVER01").unwrap();
        let client_config = SessionConfig::default().with_code("CLIENT01").unwrap();

        let mut server = Session::responder(server_config);
        let mut client = Session::initiator(client_config);

        // B — serveur envoie SSRM
        let out_b = server.accept_connection().unwrap();
        let ssrm = out_b.to_peer.expect("SSRM");

        // H — client envoie SSID
        let out_h = client
            .transition(TransitionInput::Peer(ssrm))
            .expect("transition H");
        assert_eq!(out_h.next, ProtocolState::InitWaitSsid);
        let client_ssid = out_h.to_peer.expect("SSID client");

        // E+G — serveur répond SSID
        let out_g = server
            .transition(TransitionInput::Peer(client_ssid))
            .expect("transition E+G");
        assert_eq!(server.state, ProtocolState::IdleLi);
        assert_eq!(out_g.next, ProtocolState::IdleLi);
        let server_ssid = out_g.to_peer.expect("SSID serveur");

        // D — client reçoit SSID serveur
        let out_d = client
            .transition(TransitionInput::Peer(server_ssid))
            .expect("transition D");
        assert_eq!(client.state, ProtocolState::IdleSp);
        assert_eq!(out_d.next, ProtocolState::IdleSp);
        assert!(out_d.to_peer.is_none());

        assert_eq!(ssid_code(server.vars.peer_ssid.as_ref().unwrap()), "CLIENT01");
        assert_eq!(ssid_code(client.vars.peer_ssid.as_ref().unwrap()), "SERVER01");
        assert_eq!(server.vars.window, 50);
        assert_eq!(client.vars.buf_size, 2048);
    }

    #[test]
    fn responder_without_auto_accept_waits_after_peer_ssid() {
        let config = SessionConfig {
            auto_accept: false,
            ..SessionConfig::default().with_code("SERVER01").unwrap()
        };
        let mut server = Session::responder(config);
        server.accept_connection().unwrap();

        let mut client = Session::initiator(SessionConfig::default().with_code("CLIENT01").unwrap());
        let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        let client_ssid = client
            .transition(TransitionInput::Peer(ssrm))
            .unwrap()
            .to_peer
            .unwrap();

        let out = server
            .transition(TransitionInput::Peer(client_ssid))
            .unwrap();

        assert_eq!(server.state, ProtocolState::RespWaitConRs);
        assert_eq!(out.next, ProtocolState::RespWaitConRs);
        assert!(out.to_peer.is_none());
    }

    #[test]
    fn invalid_transition_wrong_pdu() {
        let mut client = Session::initiator(SessionConfig::default());
        let err = client
            .transition(TransitionInput::Peer(OftpExchangeBuffer::Ssid(Ssid::default())))
            .unwrap_err();
        assert!(matches!(
            err,
            ProtocolError::InvalidTransition {
                state: ProtocolState::InitWaitRm,
                ..
            }
        ));
    }
}
