pub const ESIDCMD: u8 = b'F';

pub const ESID_CR: u8 = 0x0D;
pub const ESID_CR_ALT: u8 = 0x8D;

/// Code raison ESIDREAS — RFC 5024 §5.3.11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EsidReason {
    /// `00` — fin de session normale.
    #[default]
    Normal,
    /// `01` — commande inconnue (1er octet du buffer).
    CommandNotRecognised,
    /// `02` — violation protocole (PDU invalide pour l'état courant).
    ProtocolViolation,
    /// `03` — SSIDCODE inconnu ou invalide.
    UserCodeNotKnown,
    /// `04` — mot de passe SSID invalide.
    InvalidPassword,
    /// `05` — fermeture d'urgence du site local.
    LocalEmergencyCloseDown,
    /// `06` — donnée invalide dans une commande.
    InvalidCommandData,
    /// `07` — taille buffer incohérente (STH vs commande).
    ExchangeBufferSizeError,
    /// `08` — ressources indisponibles (retry plus tard).
    ResourcesNotAvailable,
    /// `09` — timeout.
    Timeout,
    /// `10` — mode ou capacités incompatibles.
    ModeOrCapabilitiesIncompatible,
    /// `11` — réponse au challenge invalide (auth).
    InvalidChallengeResponse,
    /// `12` — exigences d'authentification sécurisée incompatibles.
    SecureAuthRequirementsIncompatible,
    /// `99` — abort non spécifié.
    Unspecified,
    /// Code hors liste RFC (13–98, etc.).
    Unknown(u8),
}

impl EsidReason {
    pub fn from_wire(code: u8) -> Self {
        match code {
            0 => Self::Normal,
            1 => Self::CommandNotRecognised,
            2 => Self::ProtocolViolation,
            3 => Self::UserCodeNotKnown,
            4 => Self::InvalidPassword,
            5 => Self::LocalEmergencyCloseDown,
            6 => Self::InvalidCommandData,
            7 => Self::ExchangeBufferSizeError,
            8 => Self::ResourcesNotAvailable,
            9 => Self::Timeout,
            10 => Self::ModeOrCapabilitiesIncompatible,
            11 => Self::InvalidChallengeResponse,
            12 => Self::SecureAuthRequirementsIncompatible,
            99 => Self::Unspecified,
            other => Self::Unknown(other),
        }
    }

    pub fn to_wire(self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::CommandNotRecognised => 1,
            Self::ProtocolViolation => 2,
            Self::UserCodeNotKnown => 3,
            Self::InvalidPassword => 4,
            Self::LocalEmergencyCloseDown => 5,
            Self::InvalidCommandData => 6,
            Self::ExchangeBufferSizeError => 7,
            Self::ResourcesNotAvailable => 8,
            Self::Timeout => 9,
            Self::ModeOrCapabilitiesIncompatible => 10,
            Self::InvalidChallengeResponse => 11,
            Self::SecureAuthRequirementsIncompatible => 12,
            Self::Unspecified => 99,
            Self::Unknown(code) => code,
        }
    }

    pub fn is_normal(self) -> bool {
        matches!(self, Self::Normal)
    }
}

impl core::fmt::Display for EsidReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let label = match self {
            Self::Normal => "normal session termination",
            Self::CommandNotRecognised => "command not recognised",
            Self::ProtocolViolation => "protocol violation",
            Self::UserCodeNotKnown => "user code not known",
            Self::InvalidPassword => "invalid password",
            Self::LocalEmergencyCloseDown => "local site emergency close down",
            Self::InvalidCommandData => "command contained invalid data",
            Self::ExchangeBufferSizeError => "exchange buffer size error",
            Self::ResourcesNotAvailable => "resources not available",
            Self::Timeout => "time out",
            Self::ModeOrCapabilitiesIncompatible => "mode or capabilities incompatible",
            Self::InvalidChallengeResponse => "invalid challenge response",
            Self::SecureAuthRequirementsIncompatible => {
                "secure authentication requirements incompatible"
            }
            Self::Unspecified => "unspecified abort code",
            Self::Unknown(code) => return write!(f, "unknown reason code {code:02}"),
        };
        write!(f, "{label} ({:02})", self.to_wire())
    }
}

/// Alias legacy — préférer [`EsidReason::Normal`].
pub const ESID_REASON_NORMAL: u8 = 0;

pub const ESID_REASON_LEN: usize = 2;
pub const ESID_TEXT_LEN_FIELD: usize = 3;
pub const ESID_TEXT_MAX: usize = 999;
pub const ESID_EMPTY_TEXT_LEN: usize = 0;

pub const ESID_CMD_LEN: usize = 1;
pub const ESID_REASON_OFFSET: usize = ESID_CMD_LEN;
pub const ESID_TEXT_LEN_OFFSET: usize = ESID_REASON_OFFSET + ESID_REASON_LEN;
pub const ESID_TEXT_OFFSET: usize = ESID_TEXT_LEN_OFFSET + ESID_TEXT_LEN_FIELD;
pub const ESID_HEADER_LEN: usize = ESID_TEXT_OFFSET;

/// En-tête + CR, sans ESIDREAST.
pub const ESID_MIN_WIRE_LEN: usize = ESID_HEADER_LEN + 1;

#[inline]
pub fn esid_wire_len(text_len: usize) -> usize {
    ESID_MIN_WIRE_LEN + text_len
}

#[inline]
pub fn is_valid_esid_cr(cr: u8) -> bool {
    matches!(cr, ESID_CR | ESID_CR_ALT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Esid;

    /// Tous les codes RFC §5.3.11 (hors `Unknown`).
    fn all_rfc_reasons() -> [EsidReason; 14] {
        [
            EsidReason::Normal,
            EsidReason::CommandNotRecognised,
            EsidReason::ProtocolViolation,
            EsidReason::UserCodeNotKnown,
            EsidReason::InvalidPassword,
            EsidReason::LocalEmergencyCloseDown,
            EsidReason::InvalidCommandData,
            EsidReason::ExchangeBufferSizeError,
            EsidReason::ResourcesNotAvailable,
            EsidReason::Timeout,
            EsidReason::ModeOrCapabilitiesIncompatible,
            EsidReason::InvalidChallengeResponse,
            EsidReason::SecureAuthRequirementsIncompatible,
            EsidReason::Unspecified,
        ]
    }

    #[test]
    fn layout_constants_match_esid_structure() {
        assert_eq!(ESIDCMD, b'F');
        assert_eq!(ESID_CMD_LEN, 1);
        assert_eq!(ESID_REASON_OFFSET, 1);
        assert_eq!(ESID_TEXT_LEN_OFFSET, 3);
        assert_eq!(ESID_TEXT_OFFSET, 6);
        assert_eq!(ESID_HEADER_LEN, 6);
        assert_eq!(ESID_MIN_WIRE_LEN, 7);
        assert_eq!(esid_wire_len(0), ESID_MIN_WIRE_LEN);
        assert_eq!(esid_wire_len(2), ESID_MIN_WIRE_LEN + 2);
        assert_eq!(ESID_REASON_NORMAL, EsidReason::Normal.to_wire());
    }

    #[test]
    fn is_valid_esid_cr_accepts_rfc_values() {
        assert!(is_valid_esid_cr(ESID_CR));
        assert!(is_valid_esid_cr(ESID_CR_ALT));
        assert!(!is_valid_esid_cr(0x0A));
        assert!(!is_valid_esid_cr(0xAA));
    }

    #[test]
    fn all_rfc_codes_roundtrip_wire() {
        for code in [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 99,
        ] {
            let reason = EsidReason::from_wire(code);
            assert_eq!(reason.to_wire(), code);
        }
    }

    #[test]
    fn unknown_code_preserved() {
        let reason = EsidReason::from_wire(42);
        assert_eq!(reason, EsidReason::Unknown(42));
        assert_eq!(reason.to_wire(), 42);
    }

    #[test]
    fn pdu_encode_decode_all_rfc_reasons_without_text() {
        for reason in all_rfc_reasons() {
            let original = Esid {
                reason,
                reason_text: Vec::new(),
                cr: ESID_CR,
            };
            let wire = original.encode().expect("encode ESID");
            assert_eq!(wire[0], ESIDCMD);
            assert_eq!(wire.len(), ESID_MIN_WIRE_LEN);
            assert_eq!(
                &wire[ESID_REASON_OFFSET..ESID_TEXT_LEN_OFFSET],
                format!("{:02}", reason.to_wire()).as_bytes()
            );
            assert_eq!(&wire[ESID_TEXT_LEN_OFFSET..ESID_TEXT_OFFSET], b"000");
            assert_eq!(wire.last().copied(), Some(ESID_CR));

            let mut decoded = Esid::default();
            decoded.decode(&wire).expect("decode ESID");
            assert_eq!(decoded.reason, reason);
            assert!(decoded.reason_text.is_empty());
            assert_eq!(decoded.cr, ESID_CR);
        }
    }

    #[test]
    fn pdu_encode_decode_all_rfc_reasons_with_reason_text() {
        for reason in all_rfc_reasons() {
            let original = Esid {
                reason,
                reason_text: b"RFC5024".to_vec(),
                cr: ESID_CR_ALT,
            };
            let wire = original.encode().unwrap();
            assert_eq!(wire.len(), esid_wire_len(original.reason_text.len()));
            assert_eq!(
                &wire[ESID_TEXT_OFFSET..ESID_TEXT_OFFSET + original.reason_text.len()],
                b"RFC5024"
            );
            assert_eq!(wire.last().copied(), Some(ESID_CR_ALT));

            let mut decoded = Esid::default();
            decoded.decode(&wire).unwrap();
            assert_eq!(decoded, original);
        }
    }

    #[test]
    fn pdu_encode_decode_unknown_reason_with_text() {
        let original = Esid {
            reason: EsidReason::Unknown(42),
            reason_text: b"custom".to_vec(),
            cr: ESID_CR,
        };
        let wire = original.encode().unwrap();
        assert_eq!(
            &wire[ESID_REASON_OFFSET..ESID_TEXT_LEN_OFFSET],
            b"42"
        );

        let mut decoded = Esid::default();
        decoded.decode(&wire).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn pdu_roundtrip_idempotent_for_each_rfc_reason() {
        for reason in all_rfc_reasons() {
            let original = Esid::with_reason(reason);
            let wire = original.encode().unwrap();

            let mut mid = Esid::default();
            mid.decode(&wire).unwrap();
            let again = mid.encode().unwrap();
            assert_eq!(again, wire, "reason {:?}", reason);

            let mut end = Esid::default();
            end.decode(&again).unwrap();
            assert_eq!(end, original);
        }
    }

    #[test]
    fn legacy_esid_reason_normal_matches_pdu_normal() {
        assert_eq!(
            Esid::normal().reason.to_wire(),
            ESID_REASON_NORMAL
        );
    }
}
