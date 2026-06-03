//! Affichage style RFC §5.3.11 — tableau Pos / Field / Value / Format.

use core::fmt;

use super::{esid_wire_len, ESIDCMD, ESID_MIN_WIRE_LEN, ESID_TEXT_OFFSET};

fn fmt_cr(cr: u8) -> String {
    match cr {
        0x0D => "0x0D".to_string(),
        0x8D => "0x8D (alt)".to_string(),
        other => format!("0x{other:02X}"),
    }
}

fn fmt_reason_text(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "(empty)".to_string();
    }
    match std::str::from_utf8(bytes) {
        Ok(s) => format!("\"{s}\""),
        Err(_) => format!("{bytes:?}"),
    }
}

impl super::Esid {
    /// Diagramme RFC §5.3.11 avec valeurs décodées.
    pub fn to_rfc_table(&self) -> String {
        let cmd = ESIDCMD as char;
        let reason = self.reason.to_string();
        let text_len = self.reason_text.len().to_string();
        let reason_text = fmt_reason_text(&self.reason_text);
        let cr = fmt_cr(self.cr);
        let wire_len = esid_wire_len(self.reason_text.len());

        format!(
            "\
o-------------------------------------------------------------------o
|       ESID        End Session                                     |
|                                                                   |
|       End Session Phase       Speaker ----> Listener              |
|-------------------------------------------------------------------|
| Pos | Field     | Value                                 | Format  |
|-----+-----------+---------------------------------------+---------|
|   0 | ESIDCMD   | {cmd:<37} | F X(1)  |
|   1 | ESIDREAS  | {reason:<37} | F 9(2)  |
|   3 | ESIDREASL | {text_len:<37} | V 9(3)  |
|   6 | ESIDREAST | {reason_text:<37} | V T(n)  |
| {cr_pos:>3} | ESIDCR    | {cr:<37} | F X(1)  |
o-------------------------------------------------------------------o
  Wire length: {wire_len} octets (min {ESID_MIN_WIRE_LEN}, ESIDREAST @ {ESID_TEXT_OFFSET})"
        ,
            cr_pos = ESID_TEXT_OFFSET + self.reason_text.len(),
        )
    }
}

impl fmt::Display for super::Esid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Esid, EsidReason};

    #[test]
    fn rfc_table_contains_all_fields() {
        let table = Esid::normal().to_rfc_table();
        for field in ["ESID", "ESIDCMD", "ESIDREAS", "ESIDREASL", "ESIDREAST", "ESIDCR"] {
            assert!(table.contains(field), "missing {field}");
        }
    }

    #[test]
    fn rfc_table_normal_session_end() {
        let table = Esid::normal().to_rfc_table();
        assert!(table.contains("normal session termination (00)"));
        assert!(table.contains("(empty)"));
        assert!(table.contains("0x0D"));
        assert!(table.contains("Wire length: 7"));
    }

    #[test]
    fn rfc_table_with_reason_text() {
        let esid = Esid {
            reason: EsidReason::ProtocolViolation,
            reason_text: b"unexpected PDU".to_vec(),
            cr: 0x8D,
        };
        let table = esid.to_rfc_table();
        assert!(table.contains("protocol violation (02)"));
        assert!(table.contains("\"unexpected PDU\""));
        assert!(table.contains("0x8D (alt)"));
        assert!(table.contains("Wire length: 21"));
    }

    #[test]
    fn display_matches_to_rfc_table() {
        let esid = Esid::with_reason(EsidReason::Unspecified);
        assert_eq!(esid.to_string(), esid.to_rfc_table());
    }
}
