pub const ESIDCMD: u8 = b'F';

pub const ESID_CR: u8 = 0x0D;
pub const ESID_CR_ALT: u8 = 0x8D;

/// ESIDREAS = 00 — fin de session normale.
pub const ESID_REASON_NORMAL: u8 = 0;

pub const ESID_REASON_LEN: usize = 2;
pub const ESID_TEXT_LEN_FIELD: usize = 3;
pub const ESID_TEXT_MAX: usize = 999;
pub const ESID_EMPTY_TEXT_LEN: usize = 0;

pub const ESID_CMD_LEN: usize = 1;
pub const ESID_REASON_OFFSET: usize = ESID_CMD_LEN;
pub const ESID_TEXT_LEN_OFFSET: usize = ESID_REASON_OFFSET + ESID_REASON_LEN;
pub const ESID_TEXT_OFFSET: usize = ESID_TEXT_LEN_OFFSET + ESID_TEXT_LEN_FIELD;
pub const ESID_HEADER_LEN: usize = ESID_TEXT_OFFSET;

/// En-tête + CR, sans ESIDREAST.
pub const ESID_MIN_WIRE_LEN: usize = ESID_HEADER_LEN + 1;

#[inline]
pub fn esid_wire_len(text_len: usize) -> usize {
    ESID_MIN_WIRE_LEN + text_len
}

#[inline]
pub fn is_valid_esid_cr(cr: u8) -> bool {
    matches!(cr, ESID_CR | ESID_CR_ALT)
}
