//! Affichage style RFC §5.3.17.

use core::fmt;

use super::{AUCH_MIN_WIRE_LEN, AUCHCMD};

impl super::Auch {
    pub fn to_rfc_table(&self) -> String {
        let cmd = AUCHCMD as char;
        let len = self.challenge.len();

        format!(
            "\
o-------------------------------------------------------------------o
|       AUCH        Authentication Challenge                        |
|-------------------------------------------------------------------|
|   0 | AUCHCMD   | {cmd:<37} | F X(1)  |
|   1 | AUCHCHLL  | {len:<37} | V U(2)  |
|   3 | AUCHCHAL  | ({len} octets)                          | V U(n)  |
o-------------------------------------------------------------------o
  Wire length: {} octets",
            AUCH_MIN_WIRE_LEN + len
        )
    }
}

impl fmt::Display for super::Auch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc_table())
    }
}
