use std::io;

use crate::state::State;
use crate::stream::{PduReadError, PduWriteError};

use super::handshake::ProtocolError;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    PduRead(#[from] PduReadError),

    #[error(transparent)]
    PduWrite(#[from] PduWriteError),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),

    #[error("rôle {0:?} non pris en charge pour le handshake")]
    UnsupportedRole(super::config::Role),

    #[error("handshake interrompu en état {0}")]
    HandshakeStalled(State),
}
