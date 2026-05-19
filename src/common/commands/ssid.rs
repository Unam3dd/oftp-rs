use thiserror::Error;

pub const SSIDCMD: u8 = b'X';
pub const SSID_LEN: usize = 61;
pub const LEV_OFTP2: u8 = b'5';

const BUF_SIZE_MIN: u32 = 128;
const BUF_SIZE_MAX: u32 = 99_999;
const CREDIT_MAX: u16 = 999;

#[derive(Debug, Error)]
pub enum SsidError {
    #[error("failed to decode SSID: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SSID: Bad Protocol Level !")]
    BadLevelError,

    #[error("failed to decode SSID: Bad Buffer Size !")]
    BadBufferSizeError,

    #[error("failed to decode SSID: Bad Send/Receive Capability !")]
    BadModeError,

    #[error("failed to decode SSID: Bad Y/N Indicator !")]
    BadYnError,

    #[error("failed to decode SSID: Bad Credit !")]
    BadCreditError,

    #[error("failed to decode SSID: Bad Control return !")]
    BadControlReturnError,

    #[error("failed to decode SSID: Invalid SSID packet size !")]
    InvalidSsidSizeError,

    #[error("failed to encode SSID: Bad Buffer Size !")]
    EncodeBufferSizeError,

    #[error("failed to encode SSID: Bad Credit !")]
    EncodeCreditError,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SsidFieldError {
    #[error("identification code too long (max 25 characters)")]
    CodeTooLong,

    #[error("password too long (max 8 characters)")]
    PasswordTooLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsidMode {
    SendOnly,
    ReceiveOnly,
    Both,
}

#[derive(Debug, Clone)]
pub struct Ssid {
    pub level: u8,
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
            level: LEV_OFTP2,
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
    /// Définit le code identifiant (SSIDCODE), paddé à droite avec des espaces.
    pub fn set_code(&mut self, code: &str) -> Result<&mut Self, SsidFieldError> {
        if code.len() > 25 {
            return Err(SsidFieldError::CodeTooLong);
        }
        self.code = pad_field::<25>(code);
        Ok(self)
    }

    /// Définit le mot de passe (SSIDPSWD), paddé à droite avec des espaces.
    pub fn set_password(&mut self, password: &str) -> Result<&mut Self, SsidFieldError> {
        if password.len() > 8 {
            return Err(SsidFieldError::PasswordTooLong);
        }
        self.password = pad_field::<8>(password);
        Ok(self)
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SsidError> {
        if buf.len() != SSID_LEN {
            return Err(SsidError::InvalidSsidSizeError);
        }

        if buf[0] != SSIDCMD {
            return Err(SsidError::BadCommandError);
        }

        self.level = buf[1];
        
        if !matches!(self.level, b'1' | b'2' | b'4' | b'5') {
            return Err(SsidError::BadLevelError);
        }

        self.code.copy_from_slice(&buf[2..27]);
        self.password.copy_from_slice(&buf[27..35]);
        self.buffer_size = parse_buffer_size_field(&buf[35..40])?;
        self.mode = parse_mode(buf[40])?;
        self.compression = parse_yn(buf[41])?;
        self.restart = parse_yn(buf[42])?;
        self.special_logic = parse_yn(buf[43])?;
        self.credit = parse_credit_field(&buf[44..47])?;
        self.auth = parse_yn(buf[47])?;
        self.reserved.copy_from_slice(&buf[48..52]);
        self.user_data.copy_from_slice(&buf[52..60]);

        self.cr = buf[60];

        if self.cr != 0x0D && self.cr != 0x8D {
            return Err(SsidError::BadControlReturnError);
        }

        Ok(self)
    }

    pub fn encode(&mut self) -> Result<Vec<u8>, SsidError> {
        if self.cr != 0x0D && self.cr != 0x8D {
            return Err(SsidError::BadControlReturnError);
        }

        let mut v = Vec::with_capacity(SSID_LEN);

        v.push(SSIDCMD);
        v.push(self.level);
        v.extend_from_slice(&self.code);
        v.extend_from_slice(&self.password);
        v.extend_from_slice(&encode_buffer_size_field(self.buffer_size)?);
        v.push(mode_to_byte(self.mode));
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

/// Décode le champ SSIDSDEB (5 chiffres ASCII, 128–99999).
fn pad_field<const N: usize>(s: &str) -> [u8; N] {
    let mut buf = [b' '; N];
    let n = s.len().min(N);
    buf[..n].copy_from_slice(&s.as_bytes()[..n]);
    buf
}

fn parse_buffer_size_field(field: &[u8]) -> Result<u32, SsidError> {
    if field.len() != 5 || !field.iter().all(|b| b.is_ascii_digit()) {
        return Err(SsidError::BadBufferSizeError);
    }
    let value: u32 = std::str::from_utf8(field)
        .map_err(|_| SsidError::BadBufferSizeError)?
        .parse()
        .map_err(|_| SsidError::BadBufferSizeError)?;
    if !(BUF_SIZE_MIN..=BUF_SIZE_MAX).contains(&value) {
        return Err(SsidError::BadBufferSizeError);
    }
    Ok(value)
}

/// Décode le champ SSIDCRED (3 chiffres ASCII, 0–999).
fn parse_credit_field(field: &[u8]) -> Result<u16, SsidError> {
    if field.len() != 3 || !field.iter().all(|b| b.is_ascii_digit()) {
        return Err(SsidError::BadCreditError);
    }
    let value: u16 = std::str::from_utf8(field)
        .map_err(|_| SsidError::BadCreditError)?
        .parse()
        .map_err(|_| SsidError::BadCreditError)?;
    if value > CREDIT_MAX {
        return Err(SsidError::BadCreditError);
    }
    Ok(value)
}

fn encode_buffer_size_field(value: u32) -> Result<[u8; 5], SsidError> {
    if !(BUF_SIZE_MIN..=BUF_SIZE_MAX).contains(&value) {
        return Err(SsidError::EncodeBufferSizeError);
    }
    let mut buf = [b'0'; 5];
    let s = format!("{value:05}");
    buf.copy_from_slice(s.as_bytes());
    Ok(buf)
}

fn encode_credit_field(value: u16) -> Result<[u8; 3], SsidError> {
    
    if value > CREDIT_MAX {
        return Err(SsidError::EncodeCreditError);
    }

    let mut buf = [b'0'; 3];
    
    let s = format!("{value:03}");
    
    buf.copy_from_slice(s.as_bytes());
    
    Ok(buf)
}

fn parse_yn(byte: u8) -> Result<bool, SsidError> {
    match byte {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        _ => Err(SsidError::BadYnError),
    }
}

fn yn_to_byte(value: bool) -> u8 {
    if value { b'Y' } else { b'N' }
}

fn parse_mode(byte: u8) -> Result<SsidMode, SsidError> {
    match byte {
        b'S' => Ok(SsidMode::SendOnly),
        b'R' => Ok(SsidMode::ReceiveOnly),
        b'B' => Ok(SsidMode::Both),
        _ => Err(SsidError::BadModeError),
    }
}

fn mode_to_byte(mode: SsidMode) -> u8 {
    match mode {
        SsidMode::SendOnly => b'S',
        SsidMode::ReceiveOnly => b'R',
        SsidMode::Both => b'B',
    }
}

#[cfg(test)]
fn sample_packet() -> [u8; SSID_LEN] {
    let mut ssid = Ssid::default();
    ssid.encode().expect("sample SSID encode")
        .try_into()
        .expect("SSID length")
}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut ssid = Ssid::default();
        let buf = ssid.encode().unwrap();
        assert_eq!(buf.len(), SSID_LEN);
        assert_eq!(buf[0], SSIDCMD);
    }

    #[test]
    fn test_encode_bad_cr() {
        let mut ssid = Ssid::default();
        ssid.cr = 0x0A;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::BadControlReturnError));
    }

    #[test]
    fn test_encode_bad_buffer_size() {
        let mut ssid = Ssid::default();
        ssid.buffer_size = 50;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::EncodeBufferSizeError));
    }

    #[test]
    fn test_encode_bad_credit() {
        let mut ssid = Ssid::default();
        ssid.credit = 1000;
        let res = ssid.encode().unwrap_err();
        assert!(matches!(res, SsidError::EncodeCreditError));
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn test_decode_ok() {
        let mut ssid = Ssid::default();
        let buf = sample_packet();
        assert!(ssid.decode(&buf).is_ok());
    }

    #[test]
    fn test_set_code() {
        let mut ssid = Ssid::default();
        ssid.set_code("O01779122072341").unwrap();
        assert_eq!(&ssid.code[..15], b"O01779122072341");
        assert_eq!(&ssid.code[15..], [b' '; 10]);
    }

    #[test]
    fn test_set_code_too_long() {
        let mut ssid = Ssid::default();
        assert!(matches!(
            ssid.set_code("O01234567890123456789012345"),
            Err(SsidFieldError::CodeTooLong)
        ));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let mut ssid = Ssid::default();
        ssid.code = *b"O00041234TESTORG000001234";
        ssid.password = *b"SECRET  ";
        ssid.buffer_size = 4096;
        ssid.mode = SsidMode::SendOnly;
        ssid.compression = true;
        ssid.restart = true;
        ssid.credit = 100;
        ssid.auth = true;
        ssid.user_data = *b"MYDATA  ";

        let buf = ssid.encode().unwrap();
        let mut decoded = Ssid::default();
        decoded.decode(&buf).unwrap();

        assert_eq!(decoded.level, ssid.level);
        assert_eq!(decoded.code, ssid.code);
        assert_eq!(decoded.password, ssid.password);
        assert_eq!(decoded.buffer_size, ssid.buffer_size);
        assert_eq!(decoded.mode, ssid.mode);
        assert_eq!(decoded.compression, ssid.compression);
        assert_eq!(decoded.restart, ssid.restart);
        assert_eq!(decoded.special_logic, ssid.special_logic);
        assert_eq!(decoded.credit, ssid.credit);
        assert_eq!(decoded.auth, ssid.auth);
        assert_eq!(decoded.user_data, ssid.user_data);
        assert_eq!(decoded.cr, ssid.cr);
    }

    #[test]
    fn test_decode_fields() {
        let mut ssid = Ssid::default();
        let buf = sample_packet();
        let res = ssid.decode(&buf).unwrap();

        assert_eq!(res.level, LEV_OFTP2);
        assert_eq!(res.buffer_size, 2048);
        assert_eq!(res.mode, SsidMode::Both);
        assert_eq!(res.credit, 50);
        assert!(!res.compression);
        assert_eq!(res.cr, 0x0D);
    }

    #[test]
    fn test_decode_bad_cr() {
        let mut buf = sample_packet();
        buf[60] = 0xAA;

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadControlReturnError));
    }

    #[test]
    fn test_decode_bad_len() {
        let buf = [0u8; 60];
        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::InvalidSsidSizeError));
    }

    #[test]
    fn test_decode_bad_command() {
        let mut buf = sample_packet();
        buf[0] = b'I';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadCommandError));
    }

    #[test]
    fn test_decode_bad_level() {
        let mut buf = sample_packet();
        buf[1] = b'9';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadLevelError));
    }

    #[test]
    fn test_decode_bad_mode() {
        let mut buf = sample_packet();
        buf[40] = b'X';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadModeError));
    }

    #[test]
    fn test_decode_bad_buffer_size() {
        let mut buf = sample_packet();
        buf[35..40].copy_from_slice(b"00099");

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadBufferSizeError));
    }

    #[test]
    fn test_decode_bad_yn() {
        let mut buf = sample_packet();
        buf[41] = b'X';

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadYnError));
    }

    #[test]
    fn test_decode_bad_credit() {
        let mut buf = sample_packet();
        buf[44..47].copy_from_slice(b"AAA");

        let mut ssid = Ssid::default();
        let res = ssid.decode(&buf).unwrap_err();
        assert!(matches!(res, SsidError::BadCreditError));
    }
}
