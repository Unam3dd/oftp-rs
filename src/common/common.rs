//! Bibliothèque OFTP — organisation par couche.
//!
//! ```text
//! oftp-client / oftp-server (binaires)
//!        ↓
//!   session::OftpSession     ← orchestration (TCP + handshake)
//!        ↓
//!   stream                   ← STH + read/write PDU
//!   handshake / state        ← machine applicative + états RFC
//!   commands                 ← SSRM, SSID, … (encode/decode)
//!   event                    ← InputEvent / OutputEvent RFC (handshake SSRM/SSID)
//! ```

pub mod commands;
pub mod debug;
pub mod event;
pub mod session;
pub mod state;
pub mod stream;
pub mod version;
pub mod macros;

pub use commands::{Sfid, SfidFieldError, SsidFieldError};
pub use session::{ConnectOptions, OftpSession, Role, SessionError};
pub use state::{SessionPhase, State};
