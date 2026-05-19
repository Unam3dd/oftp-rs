//! Événements ODETTE-FTP formels pour les tables d'états RFC, §9.4–9.5.
//!
//! Le handshake SSRM/SSID utilise [`InputEvent`] / [`OutputEvent`] via
//! [`crate::session::handshake`]. Les autres événements serviront pour les
//! tables complètes (F_CONNECT_RQ, N_CON_CF, …).

/// Événements d'entrée, RFC 5024 §9.4.
///
/// Paramètres notés `I.Event-name.Parameter-name` ; leur valeur peut être lue
/// mais pas modifiée par l'entité OFTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputEvent {
    // --- Moniteur utilisateur (§3) ---

    /// Requête de données vers le pair (`F_DATA_RQ`).
    FDataRq,
    /// Requête d'établissement de session (`F_CONNECT_RQ`).
    FConnectRq,
    /// Requête de démarrage de fichier (`F_START_FILE_RQ`).
    FStartFileRq,
    /// Requête de fermeture de fichier (`F_CLOSE_FILE_RQ`).
    FCloseFileRq,
    /// Requête de réponse de fin de bout en bout positive (`F_EERP_RQ`).
    FEerpRq,
    /// Réponse à `F_CONNECT_IND` (`F_CONNECT_RS`).
    FConnectRs,
    /// Réponse positive à `F_START_FILE_IND` (`F_START_FILE_RS(+)`).
    FStartFileRsPos,
    /// Réponse positive à `F_CLOSE_FILE_IND` (`F_CLOSE_FILE_RS(+)`).
    FCloseFileRsPos,
    /// Requête de réponse de fin négative (`F_NERP_RQ`).
    FNerpRq,
    /// Requête d'abandon de session ou de fichier (`F_ABORT_RQ`).
    FAbortRq,
    /// Réponse négative à `F_START_FILE_IND` (`F_START_FILE_RS(-)`).
    FStartFileRsNeg,
    /// Réponse négative à `F_CLOSE_FILE_IND` (`F_CLOSE_FILE_RS(-)`).
    FCloseFileRsNeg,
    /// Requête de changement de sens (`F_CD_RQ`).
    FCdRq,
    /// Requête de libération de la session (`F_RELEASE_RQ`).
    FReleaseRq,
    /// Réponse à `F_RTR_IND` (`F_RTR_RS`).
    FRtrRs,

    // --- Service réseau (§2.2) ---

    /// Indication de connexion entrante (`N_CON_IND`).
    NConInd,
    /// Confirmation de connexion établie (`N_CON_CF`).
    NConCf,
    /// Indication de données reçues du réseau (`N_DATA_IND`).
    NDataInd,
    /// Indication de déconnexion (`N_DISC_IND`).
    NDiscInd,
    /// Indication de réinitialisation de connexion (`N_RST_IND`).
    NRstInd,

    // --- Pair ODETTE-FTP (§4) ---

    /// Start Session (`SSID`).
    Ssid,
    /// Start File (`SFID`).
    Sfid,
    /// Start File Positive Answer (`SFPA`).
    Sfpa,
    /// Start File Negative Answer (`SFNA`).
    Sfna,
    /// End File (`EFID`).
    Efid,
    /// End File Positive Answer (`EFPA`).
    Efpa,
    /// End File Negative Answer (`EFNA`).
    Efna,
    /// Tampon d'échange de données (`DATA`).
    Data,
    /// End Session (`ESID`).
    Esid,
    /// End to End Response positive (`EERP`).
    Eerp,
    /// Ready To Receive (`RTR`).
    Rtr,
    /// Change Direction (`CD`).
    Cd,
    /// Set Credit (`CDT`).
    Cdt,
    /// Start Session Ready Message (`SSRM`).
    Ssrm,
    /// Negative End Response (`NERP`).
    Nerp,
    /// Security Change Direction (`SECD`).
    Secd,
    /// Authentication Challenge (`AUCH`).
    Auch,
    /// Authentication Response (`AURP`).
    Aurp,

    // --- Interne ---

    /// Expiration du temporisateur d'inactivité interne (`TIME-OUT`).
    TimeOut,
}

/// Événements de sortie, RFC 5024 §9.5.
///
/// Paramètres notés `O.Event-name.Parameter-name` ; leur valeur peut être lue
/// et modifiée par l'entité OFTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputEvent {
    // --- Moniteur utilisateur (§3) ---

    /// Indication de données reçues du pair (`F_DATA_IND`).
    FDataInd,
    /// Indication de demande de connexion (`F_CONNECT_IND`).
    FConnectInd,
    /// Indication de démarrage de fichier (`F_START_FILE_IND`).
    FStartFileInd,
    /// Indication de fermeture de fichier (`F_CLOSE_FILE_IND`).
    FCloseFileInd,
    /// Indication de réponse de fin positive (`F_EERP_IND`).
    FEerpInd,
    /// Confirmation de connexion établie (`F_CONNECT_CF`).
    FConnectCf,
    /// Confirmation positive de démarrage de fichier (`F_START_FILE_CF(+)`).
    FStartFileCfPos,
    /// Confirmation positive de fermeture de fichier (`F_CLOSE_FILE_CF(+)`).
    FCloseFileCfPos,
    /// Indication de changement de sens (`F_CD_IND`).
    FCdInd,
    /// Indication d'abandon (`F_ABORT_IND`).
    FAbortInd,
    /// Confirmation négative de démarrage de fichier (`F_START_FILE_CF(-)`).
    FStartFileCfNeg,
    /// Confirmation négative de fermeture de fichier (`F_CLOSE_FILE_CF(-)`).
    FCloseFileCfNeg,
    /// Indication de réponse de fin négative (`F_NERP_IND`).
    FNerpInd,
    /// Indication de libération de session (`F_RELEASE_IND`).
    FReleaseInd,
    /// Confirmation d'émission de données (`F_DATA_CF`).
    FDataCf,
    /// Confirmation Ready To Receive (`F_RTR_CF`).
    FRtrCf,

    // --- Service réseau (§2.2) ---

    /// Requête de connexion réseau (`N_CON_RQ`).
    NConRq,
    /// Réponse à une connexion entrante (`N_CON_RS`).
    NConRs,
    /// Requête d'émission de données réseau (`N_DATA_RQ`).
    NDataRq,
    /// Requête de déconnexion (`N_DISC_RQ`).
    NDiscRq,

    // --- Pair ODETTE-FTP (§4) ---

    /// Start Session (`SSID`).
    Ssid,
    /// Start File (`SFID`).
    Sfid,
    /// Start File Positive Answer (`SFPA`).
    Sfpa,
    /// Start File Negative Answer (`SFNA`).
    Sfna,
    /// End File (`EFID`).
    Efid,
    /// End File Positive Answer (`EFPA`).
    Efpa,
    /// End File Negative Answer (`EFNA`).
    Efna,
    /// Tampon d'échange de données (`DATA`).
    Data,
    /// End Session (`ESID`).
    Esid,
    /// End to End Response positive (`EERP`).
    Eerp,
    /// Ready To Receive (`RTR`).
    Rtr,
    /// Change Direction (`CD`).
    Cd,
    /// Set Credit (`CDT`).
    Cdt,
    /// Start Session Ready Message (`SSRM`).
    Ssrm,
    /// Negative End Response (`NERP`).
    Nerp,
    /// Security Change Direction (`SECD`).
    Secd,
    /// Authentication Challenge (`AUCH`).
    Auch,
    /// Authentication Response (`AURP`).
    Aurp,
}

impl InputEvent {
    /// Nom de l'événement tel que défini dans la RFC 5024.
    pub const fn as_rfc_str(self) -> &'static str {
        match self {
            Self::FDataRq => "F_DATA_RQ",
            Self::FConnectRq => "F_CONNECT_RQ",
            Self::FStartFileRq => "F_START_FILE_RQ",
            Self::FCloseFileRq => "F_CLOSE_FILE_RQ",
            Self::FEerpRq => "F_EERP_RQ",
            Self::FConnectRs => "F_CONNECT_RS",
            Self::FStartFileRsPos => "F_START_FILE_RS(+)",
            Self::FCloseFileRsPos => "F_CLOSE_FILE_RS(+)",
            Self::FNerpRq => "F_NERP_RQ",
            Self::FAbortRq => "F_ABORT_RQ",
            Self::FStartFileRsNeg => "F_START_FILE_RS(-)",
            Self::FCloseFileRsNeg => "F_CLOSE_FILE_RS(-)",
            Self::FCdRq => "F_CD_RQ",
            Self::FReleaseRq => "F_RELEASE_RQ",
            Self::FRtrRs => "F_RTR_RS",
            Self::NConInd => "N_CON_IND",
            Self::NConCf => "N_CON_CF",
            Self::NDataInd => "N_DATA_IND",
            Self::NDiscInd => "N_DISC_IND",
            Self::NRstInd => "N_RST_IND",
            Self::Ssid => "SSID",
            Self::Sfid => "SFID",
            Self::Sfpa => "SFPA",
            Self::Sfna => "SFNA",
            Self::Efid => "EFID",
            Self::Efpa => "EFPA",
            Self::Efna => "EFNA",
            Self::Data => "DATA",
            Self::Esid => "ESID",
            Self::Eerp => "EERP",
            Self::Rtr => "RTR",
            Self::Cd => "CD",
            Self::Cdt => "CDT",
            Self::Ssrm => "SSRM",
            Self::Nerp => "NERP",
            Self::Secd => "SECD",
            Self::Auch => "AUCH",
            Self::Aurp => "AURP",
            Self::TimeOut => "TIME-OUT",
        }
    }
}

impl OutputEvent {
    /// Nom de l'événement tel que défini dans la RFC 5024.
    pub const fn as_rfc_str(self) -> &'static str {
        match self {
            Self::FDataInd => "F_DATA_IND",
            Self::FConnectInd => "F_CONNECT_IND",
            Self::FStartFileInd => "F_START_FILE_IND",
            Self::FCloseFileInd => "F_CLOSE_FILE_IND",
            Self::FEerpInd => "F_EERP_IND",
            Self::FConnectCf => "F_CONNECT_CF",
            Self::FStartFileCfPos => "F_START_FILE_CF(+)",
            Self::FCloseFileCfPos => "F_CLOSE_FILE_CF(+)",
            Self::FCdInd => "F_CD_IND",
            Self::FAbortInd => "F_ABORT_IND",
            Self::FStartFileCfNeg => "F_START_FILE_CF(-)",
            Self::FCloseFileCfNeg => "F_CLOSE_FILE_CF(-)",
            Self::FNerpInd => "F_NERP_IND",
            Self::FReleaseInd => "F_RELEASE_IND",
            Self::FDataCf => "F_DATA_CF",
            Self::FRtrCf => "F_RTR_CF",
            Self::NConRq => "N_CON_RQ",
            Self::NConRs => "N_CON_RS",
            Self::NDataRq => "N_DATA_RQ",
            Self::NDiscRq => "N_DISC_RQ",
            Self::Ssid => "SSID",
            Self::Sfid => "SFID",
            Self::Sfpa => "SFPA",
            Self::Sfna => "SFNA",
            Self::Efid => "EFID",
            Self::Efpa => "EFPA",
            Self::Efna => "EFNA",
            Self::Data => "DATA",
            Self::Esid => "ESID",
            Self::Eerp => "EERP",
            Self::Rtr => "RTR",
            Self::Cd => "CD",
            Self::Cdt => "CDT",
            Self::Ssrm => "SSRM",
            Self::Nerp => "NERP",
            Self::Secd => "SECD",
            Self::Auch => "AUCH",
            Self::Aurp => "AURP",
        }
    }
}

impl core::fmt::Display for InputEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_rfc_str())
    }
}

impl core::fmt::Display for OutputEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_rfc_str())
    }
}
