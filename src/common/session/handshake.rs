//! Machine à états applicative du handshake (SSRM / SSID uniquement).

use crate::commands::OftpExchangeBuffer;
use crate::event::{InputEvent, OutputEvent};
use crate::state::State;
use thiserror::Error;
use tracing::{debug, warn};

use super::OftpSession;

/// Dérive un [`InputEvent`] RFC à partir d'un PDU reçu (SSRM / SSID seulement).
pub fn input_event_from_pdu(pdu: &OftpExchangeBuffer) -> Result<InputEvent, ProtocolError> {
    match pdu {
        OftpExchangeBuffer::Ssrm(_) => Ok(InputEvent::Ssrm),
        OftpExchangeBuffer::Ssid(_) => Ok(InputEvent::Ssid),
        OftpExchangeBuffer::Sfid(_) => Ok(InputEvent::Sfid),
        OftpExchangeBuffer::None => Err(ProtocolError::UnexpectedPdu),
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("transition invalide: état {state}, événement {event}")]
    InvalidTransition {
        state: State,
        event: InputEvent,
    },

    #[error("PDU inattendu ou vide")]
    UnexpectedPdu,

    #[error("événement de sortie non géré pour le handshake: {event}")]
    UnsupportedOutput { event: OutputEvent },
}

pub fn transition(
    state: State,
    event: InputEvent,
) -> Result<(Vec<OutputEvent>, State), ProtocolError> {
    match (state, event) {
        (State::IWfRm, InputEvent::Ssrm) => Ok((vec![OutputEvent::Ssid], State::IWfSsid)),
        (State::IWfSsid, InputEvent::Ssid) => Ok((vec![], State::IdleSp)),
        (s, e) => Err(ProtocolError::InvalidTransition { state: s, event: e }),
    }
}

impl OftpSession {
    
    pub(super) async fn run_initiator_handshake(&mut self) -> Result<(), super::error::SessionError> {
        debug!(role = ?self.options.role, "démarrage handshake initiateur");

        loop {
            match self.state {
                State::IdleSp => {
                    debug!(
                        state = %self.state,
                        phase = %self.state.phase(),
                        "handshake terminé — session OFTP établie (IDLESP = prêt, pas déconnecté)"
                    );
                    return Ok(());
                }
                State::IWfRm | State::IWfSsid => {
                    let (_sth, pdu) = self.read_pdu().await?;
                    let event = input_event_from_pdu(&pdu)?;
                    debug!(state = %self.state, %event, "PDU reçu");

                    let (outputs, next) = transition(self.state, event)?;
                    self.state = next;

                    for output in outputs {
                        self.execute_handshake_output(output).await?;
                    }
                }
                other => {
                    warn!(%other, "handshake bloqué");
                    return Err(super::error::SessionError::HandshakeStalled(other));
                }
            }
        }
    }

    async fn execute_handshake_output(
        &mut self,
        output: OutputEvent,
    ) -> Result<(), super::error::SessionError> {
        match output {
            OutputEvent::Ssid => {
                debug!("envoi SSID local");
                let pdu = OftpExchangeBuffer::Ssid(self.options.local_ssid.clone());
                self.write_pdu(&pdu).await?;
            }
            other => return Err(ProtocolError::UnsupportedOutput { event: other }.into()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::ssrm::Ssrm;

    #[test]
    fn ssrm_then_ssid_flow() {
        let (outputs, next) = transition(State::IWfRm, InputEvent::Ssrm).unwrap();
        assert_eq!(outputs, vec![OutputEvent::Ssid]);
        assert_eq!(next, State::IWfSsid);

        let (outputs, next) = transition(State::IWfSsid, InputEvent::Ssid).unwrap();
        assert!(outputs.is_empty());
        assert_eq!(next, State::IdleSp);
    }

    #[test]
    fn reject_ssid_in_iwf_rm() {
        let err = transition(State::IWfRm, InputEvent::Ssid).unwrap_err();
        assert!(matches!(err, ProtocolError::InvalidTransition { .. }));
    }

    #[test]
    fn input_event_from_ssrm_pdu() {
        let ssrm = OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D });
        assert_eq!(input_event_from_pdu(&ssrm).unwrap(), InputEvent::Ssrm);
    }
}
