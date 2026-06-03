//! Affichage style RFC §5.3.10 — tableau Pos / Field / Value / Format.

use core::fmt;

use super::mode::SsidMode;
use super::protocol_level::ProtocolLevel;
use super::{SSIDCMD, SSID_LEN};

fn fmt_cr(cr: u8) -> String {
    match cr {
        0x0D => "0x0D".to_string(),
        0x8D => "0x8D (alt)".to_string(),
        other => format!("0x{other:02X}"),
    }
}

fn fmt_yn(value: bool) -> &'static str {
    if value { "Y" } else { "N" }
}

fn fmt_padded_ascii(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => {
            let trimmed = s.trim_end();
            if trimmed.is_empty() {
                "(empty)".to_string()
            } else {
                format!("\"{trimmed}\"")
            }
        }
        Err(_) => format!("{bytes:?}"),
    }
}

fn fmt_password(bytes: &[u8; 8]) -> String {
    if bytes.iter().all(|&b| b == b' ') {
        return "(empty)".to_string();
    }
    "(redacted)".to_string()
}

fn fmt_level(level: ProtocolLevel) -> String {
    match level {
        ProtocolLevel::Rev12 => "1 (Rev1.2)".to_string(),
        ProtocolLevel::Rev13 => "2 (Rev1.3)".to_string(),
        ProtocolLevel::Rev14 => "4 (Rev1.4)".to_string(),
        ProtocolLevel::Rev20 => "5 (Rev2.0)".to_string(),
    }
}

fn fmt_mode(mode: SsidMode) -> String {
    match mode {
        SsidMode::SendOnly => "S (SendOnly)".to_string(),
        SsidMode::ReceiveOnly => "R (ReceiveOnly)".to_string(),
        SsidMode::Both => "B (Both)".to_string(),
    }
}

impl super::Ssid {
    /// Diagramme RFC §5.3.10 avec valeurs décodées.
    pub fn to_rfc_table(&self) -> String {
        let cmd = SSIDCMD as char;
        let level = fmt_level(self.level);
        let code = fmt_padded_ascii(&self.code);
        let password = fmt_password(&self.password);
        let buffer_size = self.buffer_size.to_string();
        let mode = fmt_mode(self.mode);
        let compression = fmt_yn(self.compression);
        let restart = fmt_yn(self.restart);
        let special = fmt_yn(self.special_logic);
        let credit = self.credit.to_string();
        let auth = fmt_yn(self.auth);
        let reserved = fmt_padded_ascii(&self.reserved);
        let user_data = fmt_padded_ascii(&self.user_data);
        let cr = fmt_cr(self.cr);

        format!(
            "\
o-------------------------------------------------------------------o
|       SSID        Start Session                                   |
|                                                                   |
|       Start Session Phase     Initiator <---> Responder           |
|-------------------------------------------------------------------|
| Pos | Field     | Value                                 | Format  |
|-----+-----------+---------------------------------------+---------|
|   0 | SSIDCMD   | {cmd:<37} | F X(1)  |
|   1 | SSIDLEV   | {level:<37} | F 9(1)  |
|   2 | SSIDCODE  | {code:<37} | V X(25) |
|  27 | SSIDPSWD  | {password:<37} | V X(8)  |
|  35 | SSIDSDEB  | {buffer_size:<37} | V 9(5)  |
|  40 | SSIDSR    | {mode:<37} | F X(1)  |
|  41 | SSIDCMPR  | {compression:<37} | F X(1)  |
|  42 | SSIDREST  | {restart:<37} | F X(1)  |
|  43 | SSIDSPEC  | {special:<37} | F X(1)  |
|  44 | SSIDCRED  | {credit:<37} | V 9(3)  |
|  47 | SSIDAUTH  | {auth:<37} | F X(1)  |
|  48 | SSIDRSV1  | {reserved:<37} | F X(4)  |
|  52 | SSIDUSER  | {user_data:<37} | V X(8)  |
|  60 | SSIDCR    | {cr:<37} | F X(1)  |
o-------------------------------------------------------------------o
  Wire length: {SSID_LEN} octets"
        )
    }
}

impl fmt::Display for super::Ssid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Ssid;

    #[test]
    fn rfc_table_contains_all_fields() {
        let mut ssid = Ssid::default();
        ssid.set_code("CLIENT01").unwrap();
        let table = ssid.to_rfc_table();

        for field in [
            "SSID",
            "SSIDCMD",
            "SSIDLEV",
            "SSIDCODE",
            "SSIDPSWD",
            "SSIDSDEB",
            "SSIDSR",
            "SSIDCMPR",
            "SSIDREST",
            "SSIDSPEC",
            "SSIDCRED",
            "SSIDAUTH",
            "SSIDRSV1",
            "SSIDUSER",
            "SSIDCR",
        ] {
            assert!(table.contains(field), "missing {field}");
        }
    }

    #[test]
    fn rfc_table_shows_decoded_values() {
        let mut ssid = Ssid::default();
        ssid.set_code("O1999MENDESTALES").unwrap();
        ssid.buffer_size = 2048;
        ssid.credit = 50;

        let table = ssid.to_rfc_table();
        assert!(table.contains("\"O1999MENDESTALES\""));
        assert!(table.contains("2048"));
        assert!(table.contains("B (Both)"));
        assert!(table.contains("50"));
        assert!(table.contains("5 (Rev2.0)"));
        assert!(table.contains("Wire length: 61"));
    }

    #[test]
    fn password_redacted_when_set() {
        let mut ssid = Ssid::default();
        ssid.set_password("secret").unwrap();
        assert!(ssid.to_rfc_table().contains("(redacted)"));
    }

    #[test]
    fn display_matches_to_rfc_table() {
        let ssid = Ssid::default();
        assert_eq!(ssid.to_string(), ssid.to_rfc_table());
    }
}
