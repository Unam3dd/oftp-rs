/// Octet 0 — AUCHCMD (RFC 5024 §5.3.17).
pub const AUCHCMD: u8 = b'A';

/// Longueur minimale : commande + AUCHCHLL (2 octets).
pub const AUCH_MIN_WIRE_LEN: usize = 3;

/// Défi aléatoire RFC (avant enveloppe CMS) — 20 octets.
pub const AUCH_CHALLENGE_LEN: usize = 20;

/// Limite défensive sur AUCHCHAL (hors CMS).
pub const AUCH_CHALLENGE_MAX: usize = 4096;
