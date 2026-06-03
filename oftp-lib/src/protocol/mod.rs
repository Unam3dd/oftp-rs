//! Couche protocole — FSM RFC 5024 §9 (sans I/O réseau).

pub mod config;
pub mod error;
pub mod input;
pub mod negotiate;
pub mod output;
pub mod session;
pub mod state;
pub mod vars;

pub use config::SessionRole;
pub use error::ProtocolError;
pub use input::TransitionInput;
pub use negotiate::{
    negotiate_ssid, NegotiatedParams, NegotiationInput, PartnerPolicy,
    peer_mode_compatible_with_cap, SsidNegotiationError, Transport,
};
pub use output::TransitionOutput;
pub use session::{Session, SessionConfig};
pub use state::ProtocolState;
pub use vars::SessionVars;
