//! Affichage style RFC §5.3.18.

use core::fmt;

use super::{AURP_LEN, AURPCMD};

impl super::Aurp {
    pub fn to_rfc_table(&self) -> String {
        let cmd = AURPCMD as char;

        format!(
            "\
o-------------------------------------------------------------------o
|       AURP        Authentication Response                         |
|-------------------------------------------------------------------|
|   0 | AURPCMD   | {cmd:<37} | F X(1)  |
|   1 | AURPRSP   | (20 octets)                           | V U(20) |
o-------------------------------------------------------------------o
  Wire length: {AURP_LEN} octets"
        )
    }
}

impl fmt::Display for super::Aurp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}
