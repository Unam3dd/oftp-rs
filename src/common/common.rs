//! Bibliothèque OFTP — organisation par couche.
//!
//! ```text
//! oftp-client / oftp-server (binaires)
//!        ↓
//!   session::OftpSession     ← orchestration (TCP + handshake + transfert)
//!        ↓
//!   stream                   ← STH + read/write PDU
//!   handshake / transfer       ← machines applicatives
//!   commands                 ← SSRM, SSID, SFID, DATA, … (encode/decode)
//!   event                    ← InputEvent / OutputEvent RFC
//! ```

pub mod commands;
pub mod debug;
pub mod event;
pub mod session;
pub mod state;
pub mod stream;
pub mod version;
pub mod macros;

pub use commands::{Sfid, SfidFieldError, Ssid, SsidFieldError};
pub use session::{ConnectOptions, OftpSession, Role, SessionError, TransferError};
