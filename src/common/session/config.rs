use crate::commands::{Ssid, SsidFieldError};

/// Rôle de l'entité sur la session TCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    /// Client : connecte, reçoit SSRM, envoie SSID.
    #[default]
    Initiator,
    /// Serveur : à implémenter (accept, envoi SSRM).
    Responder,
}

/// Paramètres **locaux** avant connexion (identité, capacités négociées).
///
/// Distinct de [`crate::state::State`] (état protocole RFC) et de
/// [`crate::event::InputEvent`] (événements formels RFC).
#[derive(Debug, Clone)]
pub struct ConnectOptions {
    pub role: Role,
    pub local_ssid: Ssid,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        Self {
            role: Role::Initiator,
            local_ssid: Ssid::default(),
        }
    }
}

impl ConnectOptions {
    pub fn with_role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    /// Code identifiant OFTP (SSIDCODE) pour la session.
    pub fn with_ssid_code(mut self, code: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_code(code)?;
        Ok(self)
    }

    /// Mot de passe OFTP (SSIDPSWD) pour la session.
    pub fn with_password(mut self, password: &str) -> Result<Self, SsidFieldError> {
        self.local_ssid.set_password(password)?;
        Ok(self)
    }
}
