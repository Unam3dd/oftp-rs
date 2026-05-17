/// États du mode commande ODETTE-FTP (demi-duplex), RFC 5024 §9.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Idle,
    /// Auditeur inactif (`IDLELI`).
    IdleLi,
    /// Auditeur inactif après `F_CD_RQ` ; ESID valide (`IDLELICD`).
    IdleLiCd,
    /// Émetteur inactif (`IDLESP`).
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

impl State {
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
