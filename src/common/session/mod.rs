//! Session OFTP : connexion TCP, lecture/écriture PDU, handshake.
//!
//! Couches du crate :
//! - [`crate::commands`] — format des paquets (SSID, SSRM, …)
//! - [`crate::stream`] — STH + I/O TCP
//! - [`crate::state`] / [`crate::event`] — référence RFC (états / événements formels)
//! - [`handshake`] — machine applicative (SSRM → SSID)
//! - [`OftpSession`] — orchestration

mod config;
mod error;
mod handshake;

pub use config::{ConnectOptions, Role};
pub use error::SessionError;
pub use handshake::{input_event_from_pdu, transition, ProtocolError};

use std::io;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tracing::info;

use crate::commands::OftpExchangeBuffer;
use crate::state::{SessionPhase, State};
use crate::stream::{
    PduReadError, PduWriteError, StreamTransmission, StreamTransmissionHeader,
};

/// Point d'entrée : une session TCP + état protocole + options locales.
#[derive(Debug)]
pub struct OftpSession {
    options: ConnectOptions,
    transport: StreamTransmission,
    state: State,
    stream: Option<TcpStream>,
}

impl OftpSession {
    pub fn new(options: ConnectOptions) -> Self {
        Self {
            options,
            transport: StreamTransmission::default(),
            state: State::Idle,
            stream: None,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// Phase courante (déconnecté, handshake, session établie, …).
    pub fn phase(&self) -> SessionPhase {
        self.state.phase()
    }

    /// `true` après handshake réussi (`IDLESP` côté initiateur, etc.).
    pub fn is_established(&self) -> bool {
        self.state.is_session_established()
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.options
    }

    pub async fn connect(&mut self, target: &str) -> Result<(), SessionError> {
        info!(%target, role = ?self.options.role, "connexion TCP");
        self.stream = Some(TcpStream::connect(target).await?);
        self.state = match self.options.role {
            Role::Initiator => State::IWfRm,
            Role::Responder => State::ANcOnly,
        };
        Ok(())
    }

    pub fn stream(&self) -> Option<&TcpStream> {
        self.stream.as_ref()
    }

    pub async fn read_pdu(
        &mut self,
    ) -> Result<(StreamTransmissionHeader, OftpExchangeBuffer), PduReadError> {
        let stream = self.stream.as_mut().ok_or(PduReadError::Io(io::Error::new(
            io::ErrorKind::NotConnected,
            "session not connected",
        )))?;
        self.transport.read_pdu(stream).await
    }

    pub async fn write_pdu(
        &mut self,
        pdu: &OftpExchangeBuffer,
    ) -> Result<StreamTransmissionHeader, PduWriteError> {
        let stream = self.stream.as_mut().ok_or(PduWriteError::Io(io::Error::new(
            io::ErrorKind::NotConnected,
            "session not connected",
        )))?;
        self.transport.write_pdu(stream, pdu).await
    }

    /// Handshake selon le rôle configuré dans [`ConnectOptions`].
    pub async fn run_handshake(&mut self) -> Result<(), SessionError> {
        match self.options.role {
            Role::Initiator => self.run_initiator_handshake().await,
            Role::Responder => Err(SessionError::UnsupportedRole(Role::Responder)),
        }
    }

    pub async fn close(&mut self) -> Result<(), SessionError> {
        self.state = State::WfNdisc;

        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await?;
        }

        self.state = State::Idle;
        Ok(())
    }
}
