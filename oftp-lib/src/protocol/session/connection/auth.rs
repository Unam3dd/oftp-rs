//! Auth §4.2.3 / §9.8 — stub plaintext et vérification AURP (CMS à venir).

use crate::codec::pdu::auch::AUCH_CHALLENGE_LEN;
use crate::codec::pdu::aurp::{Aurp, AURP_RESPONSE_LEN};

use super::super::Session;

/// Défi sortant pour un tour (20 octets RFC avant CMS).
pub fn new_auth_challenge() -> Vec<u8> {
    vec![0xA5u8; AUCH_CHALLENGE_LEN]
}

pub fn store_auth_challenge(session: &mut Session, challenge: &[u8]) {
    session.vars.auth_challenge = Some(challenge.to_vec());
}

/// Réponse AURP stub : 20 premiers octets du défi stocké (sans déchiffrement CMS).
pub fn plaintext_stub_response(challenge: &[u8]) -> [u8; AURP_RESPONSE_LEN] {
    let mut response = [0u8; AURP_RESPONSE_LEN];
    let n = challenge.len().min(AURP_RESPONSE_LEN);
    response[..n].copy_from_slice(&challenge[..n]);
    response
}

/// P6 / P7 — signature AURP vérifiable (stub ou CMS selon config).
pub fn verify_aurp_response(session: &Session, aurp: &Aurp) -> bool {
    if !session.config.auth_plaintext_stub {
        return false;
    }
    let Some(challenge) = &session.vars.auth_challenge else {
        return false;
    };
    aurp.response == plaintext_stub_response(challenge)
}

pub fn outbound_aurp_from_stored_challenge(session: &Session) -> Option<Aurp> {
    let challenge = session.vars.auth_challenge.as_ref()?;
    Some(Aurp {
        response: plaintext_stub_response(challenge),
    })
}
