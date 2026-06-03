//! Événements §9 pour la FSM — uniquement les PDU reçus du partenaire OFTP.
//!
//! Le TCP (`N_CON_IND`, `N_CON_CF`), le démarrage Initiator (`F_CONNECT_RQ`) et
//! l'acceptation partenaire (`F_CONNECT_RS`) sont hors `TransitionInput` : le runtime crée
//! la session au bon rôle/état, et la validation app (si nécessaire) se fait
//! avant d'alimenter la FSM ou via auto-accept (transitions E+G fusionnées).

use crate::codec::oeb::OftpExchangeBuffer;

/// Événement consommé par `transition(session, input)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionInput {
    /// PDU OFTP reçu du partenaire (SSRM, SSID, ESID, SFID, …).
    Peer(OftpExchangeBuffer),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::pdu::ssrm::Ssrm;

    #[test]
    fn input_peer_ssrm() {
        let input = TransitionInput::Peer(OftpExchangeBuffer::Ssrm(Ssrm { cr: 0x0D }));
        assert!(matches!(
            input,
            TransitionInput::Peer(OftpExchangeBuffer::Ssrm(_))
        ));
    }
}
