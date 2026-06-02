//! États protocole RFC 5024 §9.3 — table 9.8 (connexion session) pour l'instant.

/// État courant de la machine à états OFTP (connexion + handshake).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtocolState {
    /// `IDLE` — pas de session ODETTE.
    #[default]
    Idle,

    /// `I_WF_NC` — Initiator : attente confirmation connexion TCP (`N_CON_CF`).
    InitWaitNc,

    /// `I_WF_RM` — Initiator : SSRM attendu du Responder.
    InitWaitRm,

    /// `I_WF_SSID` — Initiator : SSID envoyé, SSID pair attendu.
    InitWaitSsid,

    /// `A_NC_ONLY` — Responder : SSRM envoyé, SSID Initiator attendu.
    RespNcOnly,

    /// `A_WF_CONRS` — Responder : SSID reçu, validation app en cours (RFC E→G).
    /// Peut être ignoré si auto-accept : `Peer(Ssid)` enchaîne directement l'envoi SSID.
    RespWaitConRs,

    /// `IDLESP` — session établie, tour Speaker (peut envoyer un fichier).
    IdleSp,

    /// `IDLELI` — session établie, tour Listener (peut recevoir un fichier).
    IdleLi,
}
