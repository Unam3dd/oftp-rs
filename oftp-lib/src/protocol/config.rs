//! Configuration locale et rôle session (§9.7).

use crate::codec::pdu::ssid::{Ssid, SsidFieldError, SsidMode};

use super::negotiate::PartnerPolicy;
use crate::codec::pdu::ssid::protocol_level::ProtocolLevel;

/// Rôle TCP de l'entité (Initiator = client, Responder = serveur).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRole {
    Initiator,
    Responder,
}

/// Capacités locales et identité — sert à construire le SSID émis (§9.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionConfig {
    /// Template SSID local (code, password, buffer_size, mode, …).
    pub local_ssid: Ssid,
    /// `C.Cap-mode` — contrainte locale (P4 transition E).
    pub cap_mode: SsidMode,
    /// Validation du SSID reçu (code / mot de passe).
    pub partner_policy: PartnerPolicy,
    /// Responder : accepter le partenaire sans pause app (transitions E+G fusionnées).
    pub auto_accept: bool,
    /// Vérification AURP en clair (tests / labo) — remplacé par CMS en production.
    pub auth_plaintext_stub: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            local_ssid: Ssid::default(),
            cap_mode: SsidMode::Both,
            partner_policy: PartnerPolicy::AcceptAny,
            auto_accept: true,
            auth_plaintext_stub: false,
        }
    }
}

impl SessionConfig {
    pub fn with_code(mut self, code: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_code(code)?;
        Ok(self)
    }

    pub fn with_password(mut self, password: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_password(password)?;
        Ok(self)
    }

    pub fn with_level(mut self, level: ProtocolLevel) -> Self {
        self.local_ssid.set_level(level);
        self
    }

    pub fn with_cap_mode(mut self, mode: SsidMode) -> Self {
        self.cap_mode = mode;
        self
    }
    
    pub fn with_buffer_size(mut self, size: u32) -> Result<Self, SsidFieldError> {

        self.local_ssid.set_buffer_size(size)?;

        Ok(self)
    }

    pub fn with_compression(mut self, compression: bool) -> Self {
        self.local_ssid.set_compression(compression);
        self
    }

    pub fn with_restart(mut self, restart: bool) -> Self {
        self.local_ssid.set_restart(restart);
        self
    }

    pub fn with_special_logic(mut self, special_logic: bool) -> Self {
        self.local_ssid.set_special_logic(special_logic);
        self
    }

    pub fn with_mode(mut self, mode: SsidMode) -> Self {
        self.local_ssid.set_mode(mode);
        self
    }

    pub fn with_credit(mut self, credit: u16) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_credit(credit)?;
        Ok(self)
    }

    pub fn with_authentication(mut self, authentication: bool) -> Self {
        self.local_ssid.set_authentication(authentication);
        self
    }

    /// Active la vérification AURP « défi → 20 premiers octets » (hors CMS).
    pub fn with_auth_plaintext_stub(mut self, enabled: bool) -> Self {
        self.auth_plaintext_stub = enabled;
        self
    }

    pub fn with_partner_policy(mut self, policy: PartnerPolicy) -> Self {
        self.partner_policy = policy;
        self
    }

    /// SSID prêt à encoder pour l'émission.
    pub fn to_ssid(&self) -> Ssid {
        self.local_ssid.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::pdu::ssid::constant::CREDIT_MAX;

    fn ssid_code(ssid: &Ssid) -> &str {
        std::str::from_utf8(&ssid.code).unwrap().trim_end()
    }

    #[test]
    fn builder_chain_sets_local_ssid_fields() {
        let config = SessionConfig::default()
            .with_code("CLIENT99").unwrap()
            .with_password("PASS").unwrap()
            .with_level(ProtocolLevel::Rev20)
            .with_mode(SsidMode::SendOnly)
            .with_buffer_size(4096).unwrap()
            .with_credit(100).unwrap()
            .with_compression(true)
            .with_restart(false)
            .with_authentication(false);

        assert_eq!(ssid_code(&config.local_ssid), "CLIENT99");
        assert_eq!(config.local_ssid.mode, SsidMode::SendOnly);
        assert_eq!(config.local_ssid.buffer_size, 4096);
        assert_eq!(config.local_ssid.credit, 100);
        assert!(config.local_ssid.compression);
        assert!(!config.local_ssid.restart);
        assert!(!config.local_ssid.auth);
    }

    #[test]
    fn with_cap_mode_does_not_change_local_ssid_mode() {
        let config = SessionConfig::default()
            .with_mode(SsidMode::SendOnly)
            .with_cap_mode(SsidMode::ReceiveOnly);

        assert_eq!(config.local_ssid.mode, SsidMode::SendOnly);
        assert_eq!(config.cap_mode, SsidMode::ReceiveOnly);
    }

    #[test]
    fn with_partner_policy_is_independent_of_ssid() {
        let policy = PartnerPolicy::RequireCode("PEER01".into());
        let config = SessionConfig::default()
            .with_code("LOCAL").unwrap()
            .with_partner_policy(policy.clone());

        assert_eq!(config.partner_policy, policy);
        assert_eq!(ssid_code(&config.local_ssid), "LOCAL");
    }

    #[test]
    fn to_ssid_clones_template() {
        let config = SessionConfig::default()
            .with_code("A").unwrap()
            .with_mode(SsidMode::Both);
        let emitted = config.to_ssid();
        assert_eq!(ssid_code(&emitted), "A");
        assert_eq!(emitted.mode, SsidMode::Both);
    }

    #[test]
    fn builder_rejects_invalid_buffer_and_credit() {
        assert!(matches!(
            SessionConfig::default().with_buffer_size(50),
            Err(SsidFieldError::BufferSizeOutOfRange)
        ));
        assert!(matches!(
            SessionConfig::default().with_credit(CREDIT_MAX + 1),
            Err(SsidFieldError::CreditOutOfRange)
        ));
    }
}
