/// Octet 0 — AURPCMD (RFC 5024 §5.3.18).
pub const AURPCMD: u8 = b'S';

/// Longueur sur le fil : commande + AURPRSP (20 octets).
pub const AURP_LEN: usize = 21;

/// Taille AURPRSP.
pub const AURP_RESPONSE_LEN: usize = 20;
