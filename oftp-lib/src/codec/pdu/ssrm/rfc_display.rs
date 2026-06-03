//! Affichage style RFC §5.3.1 — tableau Pos / Field / Value / Format.

use core::fmt;

use super::{SSRM_LEN, SSRMMSG, SSRMCMD};

fn fmt_cr(cr: u8) -> String {
    match cr {
        0x0D => "0x0D".to_string(),
        0x8D => "0x8D (alt)".to_string(),
        other => format!("0x{other:02X}"),
    }
}

fn ssrm_message_display() -> String {
    std::str::from_utf8(SSRMMSG)
        .map(|s| format!("'{s}'"))
        .unwrap_or_else(|_| format!("{:?}", SSRMMSG))
}

impl super::Ssrm {
    /// Diagramme RFC §5.3.1 avec valeurs décodées.
    pub fn to_rfc_table(&self) -> String {
        let cmd = SSRMCMD as char;
        let msg = ssrm_message_display();
        let cr = fmt_cr(self.cr);

        format!(
            "\
o-------------------------------------------------------------------o
|       SSRM        Start Session Ready Message                     |
|                                                                   |
|       Start Session Phase     Responder ----> Initiator           |
|-------------------------------------------------------------------|
| Pos | Field     | Value                                 | Format  |
|-----+-----------+---------------------------------------+---------|
|   0 | SSRMCMD   | {cmd:<37} | F X(1)  |
|   1 | SSRMMSG   | {msg:<37} | F X(17) |
|  18 | SSRMCR    | {cr:<37} | F X(1)  |
o-------------------------------------------------------------------o
  Wire length: {SSRM_LEN} octets"
        )
    }
}

impl fmt::Display for super::Ssrm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Ssrm;

    #[test]
    fn rfc_table_contains_fields_and_values() {
        let table = Ssrm { cr: 0x0D }.to_rfc_table();
        assert!(table.contains("SSRM"));
        assert!(table.contains("SSRMCMD"));
        assert!(table.contains("SSRMMSG"));
        assert!(table.contains("ODETTE FTP READY"));
        assert!(table.contains("SSRMCR"));
        assert!(table.contains("0x0D"));
        assert!(table.contains("Wire length: 19"));
    }

    #[test]
    fn display_matches_to_rfc_table() {
        let ssrm = Ssrm { cr: 0x8D };
        assert_eq!(ssrm.to_string(), ssrm.to_rfc_table());
    }
}
