/// Aperçu hexadécimal pour les logs de debug (tronqué).
pub fn hex_preview(buf: &[u8], max: usize) -> String {
    let n = buf.len().min(max);
    let hex: String = buf[..n]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ");
    if buf.len() > max {
        format!("{hex} … (+{} octets)", buf.len() - max)
    } else {
        hex
    }
}

/// Interprétation du code commande ODETTE (1er octet OEB).
pub fn command_label(cmd: u8) -> &'static str {
    match cmd {
        b'I' => "SSRM",
        b'X' => "SSID",
        b'F' => "ESID",
        b'H' => "SFID",
        _ => "inconnu",
    }
}
