//! Prédicats §9.8.3 — sous-ensemble implémenté (P2 simplifié via `apply_peer_ssid`).

use super::super::Session;

pub type Pred = fn(&Session) -> bool;

pub fn always(_: &Session) -> bool {
    true
}

/// Responder avec validation app différée (transition **E** seule).
pub fn not_auto_accept(session: &Session) -> bool {
    !session.config.auto_accept
}

/// Responder : fusion **E** + **G** (émission SSID immédiate).
pub fn auto_accept(session: &Session) -> bool {
    session.config.auto_accept
}
