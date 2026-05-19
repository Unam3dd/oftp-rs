use thiserror::Error;

pub const SFIDCMD: u8 = b'H';
/// Octets fixes jusqu'à SFIDDESCL inclus (positions 0–164).
pub const SFID_FIXED_LEN: usize = 165;
pub const DESC_MAX_LEN: usize = 999;

const LRECL_MAX: u32 = 99_999;
const FILE_SIZE_MAX: u64 = 9_999_999_999_999;
const RESTART_MAX: u64 = 99_999_999_999_999_999;

#[derive(Debug, Error)]
pub enum SfidError {
    #[error("failed to decode SFID: Bad Command !")]
    BadCommandError,

    #[error("failed to decode SFID: Bad File Format !")]
    BadFormatError,

    #[error("failed to decode SFID: Bad Y/N Indicator !")]
    BadYnError,

    #[error("failed to decode SFID: Bad numeric field !")]
    BadNumericError,

    #[error("failed to decode SFID: Bad Security Level !")]
    BadSecurityError,

    #[error("failed to decode SFID: Bad Cipher suite !")]
    BadCipherError,

    #[error("failed to decode SFID: Bad Compression !")]
    BadCompressionError,

    #[error("failed to decode SFID: Bad Envelope !")]
    BadEnvelopeError,

    #[error("failed to decode SFID: Invalid SFID packet size !")]
    InvalidSizeError,

    #[error("failed to decode SFID: description length mismatch !")]
    DescriptionLengthMismatch,

    #[error("failed to encode SFID: Bad numeric field !")]
    EncodeNumericError,

    #[error("failed to encode SFID: description too long !")]
    EncodeDescriptionTooLong,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SfidFieldError {
    #[error("dataset name too long (max 26 characters)")]
    DsnTooLong,

    #[error("user data too long (max 8 characters)")]
    UserDataTooLong,

    #[error("destination too long (max 25 characters)")]
    DestTooLong,

    #[error("originator too long (max 25 characters)")]
    OrigTooLong,

    #[error("virtual file description too long (max 999 bytes UTF-8)")]
    DescriptionTooLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfidFormat {
    Fixed,
    Variable,
    Unstructured,
    Text,
}

#[derive(Debug, Clone)]
pub struct Sfid {
    pub dsn: [u8; 26],
    pub date: [u8; 8],
    pub time: [u8; 10],
    pub user_data: [u8; 8],
    pub dest: [u8; 25],
    pub orig: [u8; 25],
    pub format: SfidFormat,
    pub lrecl: u32,
    pub file_size_k: u64,
    pub orig_file_size_k: u64,
    pub restart_pos: u64,
    pub security: u8,
    pub cipher: u8,
    pub compression: u8,
    pub envelope: u8,
    pub sign_eerp: bool,
    pub description: Vec<u8>,
}

impl Default for Sfid {
    fn default() -> Self {
        Self {
            dsn: [b' '; 26],
            date: *b"00000000",
            time: *b"0000000000",
            user_data: [b' '; 8],
            dest: [b' '; 25],
            orig: [b' '; 25],
            format: SfidFormat::Variable,
            lrecl: 0,
            file_size_k: 0,
            orig_file_size_k: 0,
            restart_pos: 0,
            security: 0,
            cipher: 0,
            compression: 0,
            envelope: 0,
            sign_eerp: false,
            description: Vec::new(),
        }
    }
}

impl Sfid {
    pub fn set_dsn(&mut self, name: &str) -> Result<&mut Self, SfidFieldError> {
        if name.len() > 26 {
            return Err(SfidFieldError::DsnTooLong);
        }
        self.dsn = pad_field::<26>(name);
        Ok(self)
    }

    pub fn set_dest(&mut self, dest: &str) -> Result<&mut Self, SfidFieldError> {
        if dest.len() > 25 {
            return Err(SfidFieldError::DestTooLong);
        }
        self.dest = pad_field::<25>(dest);
        Ok(self)
    }

    pub fn set_orig(&mut self, orig: &str) -> Result<&mut Self, SfidFieldError> {
        if orig.len() > 25 {
            return Err(SfidFieldError::OrigTooLong);
        }
        self.orig = pad_field::<25>(orig);
        Ok(self)
    }

    pub fn set_user_data(&mut self, data: &str) -> Result<&mut Self, SfidFieldError> {
        if data.len() > 8 {
            return Err(SfidFieldError::UserDataTooLong);
        }
        self.user_data = pad_field::<8>(data);
        Ok(self)
    }

    pub fn set_description(&mut self, desc: &str) -> Result<&mut Self, SfidFieldError> {
        let bytes = desc.as_bytes();
        if bytes.len() > DESC_MAX_LEN {
            return Err(SfidFieldError::DescriptionTooLong);
        }
        self.description = bytes.to_vec();
        Ok(self)
    }

    pub fn decode(&mut self, buf: &[u8]) -> Result<&Self, SfidError> {
        if buf.len() < SFID_FIXED_LEN {
            return Err(SfidError::InvalidSizeError);
        }

        if buf[0] != SFIDCMD {
            return Err(SfidError::BadCommandError);
        }

        self.dsn.copy_from_slice(&buf[1..27]);
        // SFIDRSV1 — réservé, ignoré à la lecture
        self.date.copy_from_slice(&buf[30..38]);
        self.time.copy_from_slice(&buf[38..48]);
        self.user_data.copy_from_slice(&buf[48..56]);
        self.dest.copy_from_slice(&buf[56..81]);
        self.orig.copy_from_slice(&buf[81..106]);
        self.format = parse_format(buf[106])?;
        self.lrecl = parse_numeric_field::<5>(&buf[107..112])? as u32;
        if self.lrecl > LRECL_MAX {
            return Err(SfidError::BadNumericError);
        }
        self.file_size_k = parse_numeric_field::<13>(&buf[112..125])?;
        if self.file_size_k > FILE_SIZE_MAX {
            return Err(SfidError::BadNumericError);
        }
        self.orig_file_size_k = parse_numeric_field::<13>(&buf[125..138])?;
        if self.orig_file_size_k > FILE_SIZE_MAX {
            return Err(SfidError::BadNumericError);
        }
        self.restart_pos = parse_numeric_field::<17>(&buf[138..155])?;
        if self.restart_pos > RESTART_MAX {
            return Err(SfidError::BadNumericError);
        }
        self.security = parse_fixed_security(&buf[155..157])?;
        self.cipher = parse_fixed_cipher(&buf[157..159])?;
        self.compression = parse_fixed_digit(&buf[159..160], 0, 1)?;
        self.envelope = parse_fixed_digit(&buf[160..161], 0, 1)?;
        self.sign_eerp = parse_yn(buf[161])?;

        let desc_len = parse_numeric_field::<3>(&buf[162..165])? as usize;
        if desc_len > DESC_MAX_LEN {
            return Err(SfidError::BadNumericError);
        }

        let expected_len = SFID_FIXED_LEN + desc_len;
        if buf.len() != expected_len {
            return Err(SfidError::InvalidSizeError);
        }

        self.description.clear();
        if desc_len > 0 {
            self.description.extend_from_slice(&buf[165..165 + desc_len]);
        }

        Ok(self)
    }

    pub fn encode(&self) -> Result<Vec<u8>, SfidError> {
        if self.description.len() > DESC_MAX_LEN {
            return Err(SfidError::EncodeDescriptionTooLong);
        }

        let desc_len = self.description.len();
        let mut v = Vec::with_capacity(SFID_FIXED_LEN + desc_len);

        v.push(SFIDCMD);
        v.extend_from_slice(&self.dsn);
        v.extend_from_slice(b"   ");
        v.extend_from_slice(&self.date);
        v.extend_from_slice(&self.time);
        v.extend_from_slice(&self.user_data);
        v.extend_from_slice(&self.dest);
        v.extend_from_slice(&self.orig);
        v.push(format_to_byte(self.format));
        v.extend_from_slice(&encode_numeric_field(self.lrecl as u64, 5)?);
        v.extend_from_slice(&encode_numeric_field(self.file_size_k, 13)?);
        v.extend_from_slice(&encode_numeric_field(self.orig_file_size_k, 13)?);
        v.extend_from_slice(&encode_numeric_field(self.restart_pos, 17)?);
        v.extend_from_slice(&encode_fixed_pair(self.security, 0, 3)?);
        v.extend_from_slice(&encode_fixed_pair(self.cipher, 0, 1)?);
        v.push(b'0' + self.compression);
        v.push(b'0' + self.envelope);
        v.push(yn_to_byte(self.sign_eerp));
        v.extend_from_slice(&encode_numeric_field(desc_len as u64, 3)?);
        v.extend_from_slice(&self.description);

        Ok(v)
    }
}

fn pad_field<const N: usize>(s: &str) -> [u8; N] {
    let mut buf = [b' '; N];
    let n = s.len().min(N);
    buf[..n].copy_from_slice(&s.as_bytes()[..n]);
    buf
}

fn parse_numeric_field<const N: usize>(field: &[u8]) -> Result<u64, SfidError> {
    if field.len() != N || !field.iter().all(|b| b.is_ascii_digit()) {
        return Err(SfidError::BadNumericError);
    }
    std::str::from_utf8(field)
        .map_err(|_| SfidError::BadNumericError)?
        .parse::<u64>()
        .map_err(|_| SfidError::BadNumericError)
}

fn encode_numeric_field(value: u64, width: usize) -> Result<Vec<u8>, SfidError> {
    let max = 10u64.pow(width as u32).saturating_sub(1);
    if value > max {
        return Err(SfidError::EncodeNumericError);
    }
    Ok(format!("{value:0width$}").into_bytes())
}

fn parse_fixed_security(field: &[u8]) -> Result<u8, SfidError> {
    let v = parse_numeric_field::<2>(field)? as u8;
    if v > 3 {
        return Err(SfidError::BadSecurityError);
    }
    Ok(v)
}

fn parse_fixed_cipher(field: &[u8]) -> Result<u8, SfidError> {
    let v = parse_numeric_field::<2>(field)? as u8;
    if v > 1 {
        return Err(SfidError::BadCipherError);
    }
    Ok(v)
}

fn parse_fixed_digit(field: &[u8], min: u8, max: u8) -> Result<u8, SfidError> {
    if field.len() != 1 || !field[0].is_ascii_digit() {
        return Err(SfidError::BadNumericError);
    }
    let v = field[0] - b'0';
    if !(min..=max).contains(&v) {
        return Err(SfidError::BadNumericError);
    }
    Ok(v)
}

fn encode_fixed_pair(value: u8, min: u8, max: u8) -> Result<[u8; 2], SfidError> {
    if !(min..=max).contains(&value) {
        return Err(SfidError::EncodeNumericError);
    }
    let s = format!("{value:02}");
    Ok([s.as_bytes()[0], s.as_bytes()[1]])
}

fn parse_format(byte: u8) -> Result<SfidFormat, SfidError> {
    match byte {
        b'F' => Ok(SfidFormat::Fixed),
        b'V' => Ok(SfidFormat::Variable),
        b'U' => Ok(SfidFormat::Unstructured),
        b'T' => Ok(SfidFormat::Text),
        _ => Err(SfidError::BadFormatError),
    }
}

fn format_to_byte(format: SfidFormat) -> u8 {
    match format {
        SfidFormat::Fixed => b'F',
        SfidFormat::Variable => b'V',
        SfidFormat::Unstructured => b'U',
        SfidFormat::Text => b'T',
    }
}

fn parse_yn(byte: u8) -> Result<bool, SfidError> {
    match byte {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        _ => Err(SfidError::BadYnError),
    }
}

fn yn_to_byte(value: bool) -> u8 {
    if value { b'Y' } else { b'N' }
}

#[cfg(test)]
fn sample_packet() -> Vec<u8> {
    let mut sfid = Sfid::default();
    sfid.set_dsn("MYDATASET").unwrap();
    sfid.set_dest("O01779122072341").unwrap();
    sfid.set_orig("O01779122072341").unwrap();
    sfid.file_size_k = 1;
    sfid.orig_file_size_k = 1;
    sfid.encode().expect("sample SFID encode")
}

#[cfg(test)]
mod encode_tests {
    use super::*;

    #[test]
    fn test_encode_minimal() {
        let sfid = Sfid::default();
        let buf = sfid.encode().unwrap();
        assert_eq!(buf.len(), SFID_FIXED_LEN);
        assert_eq!(buf[0], SFIDCMD);
        assert_eq!(&buf[27..30], b"   ");
    }

    #[test]
    fn test_encode_with_description() {
        let mut sfid = Sfid::default();
        sfid.set_description("fichier test").unwrap();
        let buf = sfid.encode().unwrap();
        assert_eq!(buf.len(), SFID_FIXED_LEN + "fichier test".len());
        assert_eq!(&buf[162..165], b"012");
    }

    #[test]
    fn test_encode_description_too_long() {
        let mut sfid = Sfid::default();
        sfid.description = vec![b'x'; 1000];
        assert!(matches!(
            sfid.encode().unwrap_err(),
            SfidError::EncodeDescriptionTooLong
        ));
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn test_decode_ok() {
        let mut sfid = Sfid::default();
        let buf = sample_packet();
        assert!(sfid.decode(&buf).is_ok());
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let mut sfid = Sfid::default();
        sfid.set_dsn("TESTFILE.DAT").unwrap();
        sfid.date = *b"20260519";
        sfid.time = *b"1200000001";
        sfid.set_user_data("USR1").unwrap();
        sfid.set_dest("O01779122072341").unwrap();
        sfid.set_orig("O01779122072341").unwrap();
        sfid.format = SfidFormat::Text;
        sfid.lrecl = 0;
        sfid.file_size_k = 42;
        sfid.orig_file_size_k = 42;
        sfid.restart_pos = 0;
        sfid.security = 1;
        sfid.cipher = 0;
        sfid.compression = 1;
        sfid.envelope = 0;
        sfid.sign_eerp = true;
        sfid.set_description("échange test").unwrap();

        let buf = sfid.encode().unwrap();
        let mut decoded = Sfid::default();
        decoded.decode(&buf).unwrap();

        assert_eq!(decoded.dsn, sfid.dsn);
        assert_eq!(decoded.date, sfid.date);
        assert_eq!(decoded.time, sfid.time);
        assert_eq!(decoded.dest, sfid.dest);
        assert_eq!(decoded.format, sfid.format);
        assert_eq!(decoded.file_size_k, sfid.file_size_k);
        assert_eq!(decoded.sign_eerp, sfid.sign_eerp);
        assert_eq!(decoded.description, sfid.description);
    }

    #[test]
    fn test_set_dsn_too_long() {
        let mut sfid = Sfid::default();
        assert!(matches!(
            sfid.set_dsn("0123456789012345678901234567"),
            Err(SfidFieldError::DsnTooLong)
        ));
    }

    #[test]
    fn test_decode_bad_command() {
        let mut buf = sample_packet();
        buf[0] = b'X';
        let mut sfid = Sfid::default();
        assert!(matches!(
            sfid.decode(&buf).unwrap_err(),
            SfidError::BadCommandError
        ));
    }

    #[test]
    fn test_decode_bad_len() {
        let buf = [0u8; 100];
        let mut sfid = Sfid::default();
        assert!(matches!(
            sfid.decode(&buf).unwrap_err(),
            SfidError::InvalidSizeError
        ));
    }

    #[test]
    fn test_decode_bad_format() {
        let mut buf = sample_packet();
        buf[106] = b'Z';
        let mut sfid = Sfid::default();
        assert!(matches!(
            sfid.decode(&buf).unwrap_err(),
            SfidError::BadFormatError
        ));
    }

    #[test]
    fn test_decode_desc_len_mismatch() {
        let mut buf = sample_packet();
        buf[162..165].copy_from_slice(b"005");
        let mut sfid = Sfid::default();
        assert!(matches!(
            sfid.decode(&buf).unwrap_err(),
            SfidError::InvalidSizeError
        ));
    }
}
