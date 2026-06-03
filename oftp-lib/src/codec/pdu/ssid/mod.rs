pub mod constant;
pub mod error;
pub mod fields;
pub mod mode;
pub mod protocol_level;
mod rfc_display;

#[cfg(test)]
mod tests;

pub use constant::*;
use super::super::fields::parse_yn;
pub use error::{SsidError, SsidFieldError};
use fields::*;
pub use mode::SsidMode;
pub use protocol_level::ProtocolLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ssid {
    pub level: ProtocolLevel,
    pub code: [u8; 25],
    pub password: [u8; 8],
    pub buffer_size: u32,
    pub mode: SsidMode,
    pub compression: bool,
    pub restart: bool,
    pub special_logic: bool,
    pub credit: u16,
    pub auth: bool,
    pub reserved: [u8; 4],
    pub user_data: [u8; 8],
    pub cr: u8,
}

impl Default for Ssid {
    fn default() -> Self {
        Self {
            level: ProtocolLevel::Rev20,
            code: [b' '; 25],
            password: [b' '; 8],
            buffer_size: 2048,
            mode: SsidMode::Both,
            compression: false,
            restart: false,
            special_logic: false,
            credit: 50,
            auth: false,
            reserved: [b' '; 4],
            user_data: [b' '; 8],
            cr: 0x0D,
        }
    }
}

impl Ssid {
    #[inline]
    fn is_valid_cr(cr: u8) -> bool {
        matches!(cr, 0x0D | 0x8D)
    }

    /// Définit le code identifiant (SSIDCODE), paddé à droite avec des espaces.
    pub fn set_code(&mut self, code: &str) -> Result<(), SsidFieldError> {
        if code.len() > 25 {
            return Err(SsidFieldError::CodeTooLong);
        }

        self.code = pad_field::<25>(code);

        Ok(())
    }

    pub fn set_level(&mut self, level: ProtocolLevel) -> &mut Self {
        self.level = level;
        self
    }

    pub fn set_compression(&mut self, compression: bool) -> &mut Self {
        self.compression = compression;
        self
    }

    pub fn set_restart(&mut self, restart: bool) -> &mut Self {
        self.restart = restart;
        self
    }

    pub fn set_credit(&mut self, credit: u16) -> Result<(), SsidFieldError> {

        if credit > CREDIT_MAX {
            return Err(SsidFieldError::CreditOutOfRange);
        }

        self.credit = credit;
        Ok(())
    }

    pub fn set_authentication(&mut self, authentication: bool) -> &mut Self {
        self.auth = authentication;
        self
    }

    pub fn set_mode(&mut self, mode: SsidMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_special_logic(&mut self, special_logic: bool) -> &mut Self {
        self.special_logic = special_logic;
        self
    }

    /// Définit le mot de passe (SSIDPSWD), paddé à droite avec des espaces.
    pub fn set_password(&mut self, password: &str) -> Result<(), SsidFieldError> {
        if password.len() > 8 {
            return Err(SsidFieldError::PasswordTooLong);
        }

        self.password = pad_field::<8>(password);

        Ok(())
    }

    pub fn set_buffer_size(&mut self, size: u32) -> Result<(), SsidFieldError> {
        if !(BUF_SIZE_MIN..=BUF_SIZE_MAX).contains(&size) {
            return Err(SsidFieldError::BufferSizeOutOfRange);
        }

        self.buffer_size = size;

        Ok(())
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<(), SsidError> {
        if buf.len() != SSID_LEN {
            return Err(SsidError::InvalidSsidSizeError);
        }

        if buf[0] != SSIDCMD {
            return Err(SsidError::BadCommandError);
        }

        self.level = ProtocolLevel::try_from(buf[1])?;

        self.code.copy_from_slice(&buf[2..27]);
        self.password.copy_from_slice(&buf[27..35]);
        self.buffer_size = parse_buffer_size_field(&buf[35..40])?;
        self.mode = SsidMode::try_from(buf[40])?;
        self.compression = parse_yn(buf[41]).ok_or(SsidError::BadYnError)?;
        self.restart = parse_yn(buf[42]).ok_or(SsidError::BadYnError)?;
        self.special_logic = parse_yn(buf[43]).ok_or(SsidError::BadYnError)?;
        self.credit = parse_credit_field(&buf[44..47])?;
        self.auth = parse_yn(buf[47]).ok_or(SsidError::BadYnError)?;
        self.reserved.copy_from_slice(&buf[48..52]);
        self.user_data.copy_from_slice(&buf[52..60]);

        if !Self::is_valid_cr(buf[60]) {
            return Err(SsidError::BadControlReturnError);
        }

        self.cr = buf[60];

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, SsidError> {
        if !Self::is_valid_cr(self.cr) {
            return Err(SsidError::BadControlReturnError);
        }

        let mut v = Vec::with_capacity(SSID_LEN);

        v.push(SSIDCMD);
        v.push(self.level.to_byte());
        v.extend_from_slice(&self.code);
        v.extend_from_slice(&self.password);
        v.extend_from_slice(&encode_buffer_size_field(self.buffer_size)?);
        v.push(self.mode.to_byte());
        v.push(yn_to_byte(self.compression));
        v.push(yn_to_byte(self.restart));
        v.push(yn_to_byte(self.special_logic));
        v.extend_from_slice(&encode_credit_field(self.credit)?);
        v.push(yn_to_byte(self.auth));
        v.extend_from_slice(&self.reserved);
        v.extend_from_slice(&self.user_data);
        v.push(self.cr);

        Ok(v)
    }
}
