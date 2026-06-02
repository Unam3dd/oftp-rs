pub fn pad_field<const N: usize>(s: &str) -> [u8; N] {
    let mut buf = [b' '; N];
    let n = s.len().min(N);
    buf[..n].copy_from_slice(&s.as_bytes()[..n]);
    buf
}

pub fn parse_yn(byte: u8) -> Option<bool> {
    match byte {
        b'Y' => Some(true),
        b'N' => Some(false),
        _ => None,
    }
}

pub fn yn_to_byte(value: bool) -> u8 {
    if value { b'Y' } else { b'N' }
}
