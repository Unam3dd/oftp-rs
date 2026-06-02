//! Couche protocole — FSM RFC 5024 §9 (sans I/O réseau).

pub mod error;
pub mod input;
pub mod output;
pub mod session;
pub mod state;

pub use error::ProtocolError;
pub use input::TransitionInput;
pub use output::TransitionOutput;
pub use session::{Session, SessionConfig, SessionRole, SessionVars};
pub use state::ProtocolState;
