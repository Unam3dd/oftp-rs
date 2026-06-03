use thiserror::Error;

use super::state::ProtocolState;

/// Erreur retournée par `Session::transition`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("invalid transition: state={state:?}, event={event}")]
    InvalidTransition {
        state: ProtocolState,
        event: &'static str,
    },

    #[error("transition not implemented")]
    NotImplemented,
}
