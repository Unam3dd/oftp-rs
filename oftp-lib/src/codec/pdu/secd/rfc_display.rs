//! Affichage style RFC §5.3.16 — tableau Pos / Field / Value / Format.

use core::fmt;

use super::{SECD_LEN, SECDCMD};

impl super::Secd {
    /// Diagramme RFC §5.3.16 avec valeurs encodées.
    pub fn to_rfc_table(&self) -> String {
        let cmd = SECDCMD as char;

        format!(
            "\
o-------------------------------------------------------------------o
|       SECD        Security Change Direction                       |
|                                                                   |
|       Start Session Phase     Initiator <---> Responder           |
|-------------------------------------------------------------------|
| Pos | Field     | Value                                 | Format  |
|-----+-----------+---------------------------------------+---------|
|   0 | SECDCMD   | {cmd:<37} | F X(1)  |
o-------------------------------------------------------------------o
  Wire length: {SECD_LEN} octet"
        )
    }
}

impl fmt::Display for super::Secd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}

#[cfg(test)]
mod tests {
    use super::super::Secd;

    #[test]
    fn rfc_table_contains_fields_and_values() {
        let table = Secd.to_rfc_table();
        assert!(table.contains("SECD"));
        assert!(table.contains("SECDCMD"));
        assert!(table.contains('J'));
        assert!(table.contains("Wire length: 1"));
    }

    #[test]
    fn display_matches_to_rfc_table() {
        let secd = Secd;
        assert_eq!(secd.to_string(), secd.to_rfc_table());
    }
}
