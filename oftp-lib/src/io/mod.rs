//! I/O STB — sync (`Read` / `Write`) et async Tokio (feature `tokio`).

mod stb;

pub use stb::{read_stb, write_stb};

#[cfg(feature = "async")]
mod stb_async;

#[cfg(feature = "async")]
pub use stb_async::{read_stb_async, write_stb_async};
