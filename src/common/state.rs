//! États du mode commande ODETTE-FTP (demi-duplex), RFC 5024 §9.3.
//!
//! Référence normative — la machine applicative du handshake vit dans
//! [`crate::session::handshake`], pas ici.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum State {
    /// Répondeur : connexion réseau ouverte ; SSRM envoyé, en attente du SSID (`A_NC_ONLY`).
    ANcOnly,
    /// Répondeur : SSID reçu, en attente de `F_CONNECT_RS` (`A_WF_CONRS`).
    AWfConrs,
    /// Requête `F_CD_RQ` reçue avant le CD protocolaire (`CDSTWFCD`).
    CdstWfcd,
    /// Auditeur : EFID reçu, en attente de `F_CLOSE_FILE_RS` (`CLIP`).
    Clip,
    /// Émetteur : EFID envoyé, en attente de EFPA ou EFNA (`CLOP`).
    Clop,
    /// Requête `F_EERP_RQ` reçue avant le CD protocolaire (`ERSTWFCD`).
    ErstWfcd,
    /// Connexion inactive (`IDLE`).
    #[default]
    Idle,
    /// Auditeur inactif (`IDLELI`).
    IdleLi,
    /// Auditeur inactif après `F_CD_RQ` ; ESID valide (`IDLELICD`).
    IdleLiCd,
    /// Émetteur inactif (`IDLESP`) — session **établie**, aucun transfert en cours.
    IdleSp,
    /// Émetteur inactif après `F_CD_IND` (`IDLESPCD`).
    IdleSpCd,
    /// Initiateur : en attente de `N_CON_CF` (`I_WF_NC`).
    IWfNc,
    /// Initiateur : en attente du SSRM (`I_WF_RM`).
    IWfRm,
    /// Initiateur : SSID envoyé, en attente du SSID pair (`I_WF_SSID`).
    IWfSsid,
    /// Requête `F_NERP_RQ` reçue avant le CD protocolaire (`NRSTWFCD`).
    NrstWfcd,
    /// Entrée ouverte : auditeur en attente de données (`OPI`).
    Opi,
    /// Auditeur : SFID reçu, en attente de `F_START_FILE_RS` (`OPIP`).
    Opip,
    /// Sortie ouverte : en attente de `F_DATA_RQ` ou `F_CLOSE_FILE_RQ` (`OPO`).
    Opo,
    /// Émetteur : SFID envoyé, en attente de SFPA ou SFNA (`OPOP`).
    Opop,
    /// Émetteur : en attente de CDT (`OPOWFC`).
    Opowfc,
    /// Auditeur : EERP ou NERP reçu, en attente de `F_RTR_RS` (`RTRP`).
    Rtrp,
    /// Requête `F_START_FILE_RQ` reçue avant le CD protocolaire (`SFSTWFCD`).
    SfstWfcd,
    /// Auditeur : en attente du CD (`WF_CD`).
    WfCd,
    /// Émetteur : en attente de RTR (`WF_RTR`).
    WfRtr,
    /// ESID envoyé, en attente de `N_DISC_IND` (`WF_NDISC`).
    WfNdisc,
    /// Émetteur : en attente de SECD (`WF_SECD`).
    WfSecd,
    /// Émetteur : en attente de AUCH (`WF_AUCH`).
    WfAuch,
    /// Émetteur : en attente de AURP (`WF_AURP`).
    WfAurp,
}

/// Phase lisible pour l'application (complète l'état RFC détaillé).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionPhase {
    /// Pas de session (TCP fermé).
    Disconnected,
    /// Connexion / négociation SSRM–SSID en cours.
    Handshaking,
    /// Session OFTP ouverte, prête pour SFID / DATA / etc.
    Established,
    /// Fermeture en cours.
    Closing,
}

impl SessionPhase {
    pub const fn label_fr(self) -> &'static str {
        match self {
            Self::Disconnected => "déconnecté",
            Self::Handshaking => "handshake en cours",
            Self::Established => "session établie",
            Self::Closing => "fermeture en cours",
        }
    }
}

impl core::fmt::Display for SessionPhase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.label_fr())
    }
}

impl State {
    /// Regroupe l'état RFC en une phase pour le client / les logs.
    pub const fn phase(self) -> SessionPhase {
        match self {
            Self::Idle => SessionPhase::Disconnected,
            Self::WfNdisc => SessionPhase::Closing,
            Self::IWfNc | Self::IWfRm | Self::IWfSsid | Self::ANcOnly | Self::AWfConrs => {
                SessionPhase::Handshaking
            }
            _ => SessionPhase::Established,
        }
    }

    /// Session OFTP négociée (après échange SSID, avant ESID / fermeture).
    pub const fn is_session_established(self) -> bool {
        matches!(self.phase(), SessionPhase::Established)
    }

    /// Nom de l'état tel que défini dans la RFC 5024.
    pub const fn as_rfc_str(self) -> &'static str {
        match self {
            Self::ANcOnly => "A_NC_ONLY",
            Self::AWfConrs => "A_WF_CONRS",
            Self::CdstWfcd => "CDSTWFCD",
            Self::Clip => "CLIP",
            Self::Clop => "CLOP",
            Self::ErstWfcd => "ERSTWFCD",
            Self::Idle => "IDLE",
            Self::IdleLi => "IDLELI",
            Self::IdleLiCd => "IDLELICD",
            Self::IdleSp => "IDLESP",
            Self::IdleSpCd => "IDLESPCD",
            Self::IWfNc => "I_WF_NC",
            Self::IWfRm => "I_WF_RM",
            Self::IWfSsid => "I_WF_SSID",
            Self::NrstWfcd => "NRSTWFCD",
            Self::Opi => "OPI",
            Self::Opip => "OPIP",
            Self::Opo => "OPO",
            Self::Opop => "OPOP",
            Self::Opowfc => "OPOWFC",
            Self::Rtrp => "RTRP",
            Self::SfstWfcd => "SFSTWFCD",
            Self::WfCd => "WF_CD",
            Self::WfRtr => "WF_RTR",
            Self::WfNdisc => "WF_NDISC",
            Self::WfSecd => "WF_SECD",
            Self::WfAuch => "WF_AUCH",
            Self::WfAurp => "WF_AURP",
        }
    }
}

impl core::fmt::Display for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_rfc_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idlesp_is_established_not_disconnected() {
        assert_eq!(State::IdleSp.phase(), SessionPhase::Established);
        assert!(State::IdleSp.is_session_established());
        assert_ne!(State::IdleSp.phase(), SessionPhase::Disconnected);
    }

    #[test]
    fn handshake_states() {
        assert_eq!(State::IWfRm.phase(), SessionPhase::Handshaking);
        assert_eq!(State::IWfSsid.phase(), SessionPhase::Handshaking);
    }
}
