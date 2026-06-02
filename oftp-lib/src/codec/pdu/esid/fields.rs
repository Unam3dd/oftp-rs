use super::error::{EsidError, NumericError};

pub fn parse_numeric_field(field: &[u8]) -> Result<u64, EsidError> {
    if field.is_empty() || !field.iter().all(|b| b.is_ascii_digit()) {
        return Err(NumericError::Invalid.into());
    }
    std::str::from_utf8(field)
        .map_err(|_| NumericError::Invalid)?
        .parse()
        .map_err(|_| NumericError::Invalid.into())
}

pub fn encode_numeric_field(value: u64, width: usize) -> Result<Vec<u8>, EsidError> {
    let s = format!("{value:0width$}");
    if s.len() != width {
        return Err(NumericError::Invalid.into());
    }
    Ok(s.into_bytes())
}
