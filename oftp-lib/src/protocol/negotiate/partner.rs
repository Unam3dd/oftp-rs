//! Validation identité partenaire — SSIDCODE / SSIDPSWD (ESID 03/04).

use crate::codec::pdu::esid::EsidReason;
use crate::codec::pdu::ssid::Ssid;

/// Politique de validation du partenaire (prédicat P2 métier / config app).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PartnerPolicy {
    #[default]
    AcceptAny,
    RequireCode(String),
    RequireCodeAndPassword { code: String, password: String },
}

impl PartnerPolicy {
    pub fn validate_peer(&self, peer: &Ssid) -> Result<(), EsidReason> {
        match self {
            Self::AcceptAny => Ok(()),
            Self::RequireCode(expected) => {
                if field_eq(&peer.code, expected) {
                    Ok(())
                } else {
                    Err(EsidReason::UserCodeNotKnown)
                }
            }
            Self::RequireCodeAndPassword { code, password } => {
                if !field_eq(&peer.code, code) {
                    return Err(EsidReason::UserCodeNotKnown);
                }
                if field_eq(&peer.password, password) {
                    Ok(())
                } else {
                    Err(EsidReason::InvalidPassword)
                }
            }
        }
    }
}

fn field_eq(field: &[u8], expected: &str) -> bool {
    let trimmed = std::str::from_utf8(field)
        .ok()
        .map(str::trim_end)
        .unwrap_or("");
    trimmed == expected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::pdu::ssid::Ssid;

    #[test]
    fn require_code_rejects_unknown() {
        let mut peer = Ssid::default();
        peer.set_code("PEER01").unwrap();
        let policy = PartnerPolicy::RequireCode("OTHER".into());
        assert_eq!(
            policy.validate_peer(&peer).unwrap_err(),
            EsidReason::UserCodeNotKnown
        );
    }

    #[test]
    fn require_code_accepts_match() {
        let mut peer = Ssid::default();
        peer.set_code("PEER01").unwrap();
        let policy = PartnerPolicy::RequireCode("PEER01".into());
        assert!(policy.validate_peer(&peer).is_ok());
    }
}
