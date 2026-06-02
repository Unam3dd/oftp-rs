use super::constant::{BUF_SIZE_MAX, BUF_SIZE_MIN, CREDIT_MAX};
use super::error::SsidError;

pub use super::super::super::fields::{pad_field, yn_to_byte};

pub fn parse_buffer_size_field(field: &[u8]) -> Result<u32, SsidError> {
    
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

pub fn parse_credit_field(field: &[u8]) -> Result<u16, SsidError> {
    
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

pub fn encode_buffer_size_field(value: u32) -> Result<[u8; 5], SsidError> {
    
    if !(BUF_SIZE_MIN..=BUF_SIZE_MAX).contains(&value) {
        return Err(SsidError::EncodeBufferSizeError);
    }

    let mut buf = [b'0'; 5];
    let s = format!("{value:05}");
    buf.copy_from_slice(s.as_bytes());
    Ok(buf)
}

pub fn encode_credit_field(value: u16) -> Result<[u8; 3], SsidError> {
    if value > CREDIT_MAX {
        return Err(SsidError::EncodeCreditError);
    }

    let mut buf = [b'0'; 3];
    let s = format!("{value:03}");
    buf.copy_from_slice(s.as_bytes());
    Ok(buf)
}

