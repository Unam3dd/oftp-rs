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

    /// `WF_SECD` — tour auth : attente ou émission selon rôle (§9.8).
    WfSecd,

    /// `WF_AUCH` — attente **AUCH** du pair.
    WfAuch,

    /// `WF_AURP` — attente **AURP** du pair.
    WfAurp,

    /// `WF_NDISC` — ESID envoyé, attente fermeture réseau / ESID pair.
    WaitNDisc,
}

impl ProtocolState {
    /// États §9.8 où seule une PDU auth attendue est valide (sinon transition **L**).
    pub fn is_auth_phase(self) -> bool {
        matches!(self, Self::WfSecd | Self::WfAuch | Self::WfAurp)
    }
}
