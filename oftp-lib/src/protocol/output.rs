//! Résultat d'une transition §9 — Output Events + Next State (RFC §9.5).
//!
//! La FSM ne fait pas d'I/O : le runtime envoie `to_peer` et applique `next`.

use crate::codec::oeb::OftpExchangeBuffer;

use super::state::ProtocolState;

/// Résultat de `transition(session, input)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionOutput {
    /// Prochain état protocole (colonne « Next State » des tables §9).
    pub next: ProtocolState,
    /// PDU OFTP à envoyer au partenaire, s'il y en a un (`None` = pas d'émission).
    pub to_peer: Option<OftpExchangeBuffer>,
}

impl TransitionOutput {
    /// Transition sans émission vers le pair (changement d'état seul).
    pub fn state_only(next: ProtocolState) -> Self {
        Self {
            next,
            to_peer: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::pdu::ssrm::Ssrm;

    #[test]
    fn output_state_only() {
        let out = TransitionOutput::state_only(ProtocolState::RespNcOnly);
        assert_eq!(out.next, ProtocolState::RespNcOnly);
        assert!(out.to_peer.is_none());
    }

    #[test]
    fn output_with_ssrm() {
        let out = TransitionOutput {
            next: ProtocolState::RespNcOnly,
            to_peer: Some(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D })),
        };
        assert!(matches!(
            out.to_peer,
            Some(OftpExchangeBuffer::Ssrm(_))
        ));
    }
}
