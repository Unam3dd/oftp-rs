//! Événements table 9.8 — pair, réseau simulé, primitives app.

use crate::codec::oeb::OftpExchangeBuffer;

/// Événement consommé par le dispatch §9.8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionEvent<'a> {
    /// `N_CON_IND` — transition **B**.
    AcceptConnection,
    /// `F_CONNECT_RS` — transition **G**.
    ConfirmPartner,
    /// `F_RELEASE_RQ` — fin normale (ESID).
    EndSession,
    /// PDU OFTP reçu du partenaire.
    Peer(&'a OftpExchangeBuffer),
}

/// Discriminant PDU pour filtrer les lignes de table (sans emprunter le payload).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerKind {
    Ssrm,
    Ssid,
    Esid,
    Secd,
    Auch,
    Aurp,
}

/// Filtre d'événement sur une ligne de table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventFilter {
    AcceptConnection,
    ConfirmPartner,
    EndSession,
    Peer(PeerKind),
}

impl EventFilter {
    pub fn matches(&self, event: &ConnectionEvent<'_>) -> bool {
        match (self, event) {
            (Self::AcceptConnection, ConnectionEvent::AcceptConnection) => true,
            (Self::ConfirmPartner, ConnectionEvent::ConfirmPartner) => true,
            (Self::EndSession, ConnectionEvent::EndSession) => true,
            (Self::Peer(kind), ConnectionEvent::Peer(oeb)) => event_peer_kind(oeb) == Some(*kind),
            _ => false,
        }
    }
}

fn event_peer_kind(oeb: &OftpExchangeBuffer) -> Option<PeerKind> {
    match oeb {
        OftpExchangeBuffer::Ssrm(_) => Some(PeerKind::Ssrm),
        OftpExchangeBuffer::Ssid(_) => Some(PeerKind::Ssid),
        OftpExchangeBuffer::Esid(_) => Some(PeerKind::Esid),
        OftpExchangeBuffer::Secd(_) => Some(PeerKind::Secd),
        OftpExchangeBuffer::Auch(_) => Some(PeerKind::Auch),
        OftpExchangeBuffer::Aurp(_) => Some(PeerKind::Aurp),
        OftpExchangeBuffer::None => None,
    }
}
