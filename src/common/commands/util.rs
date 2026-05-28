use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NumericError {
    #[error("invalid numeric field")]
    Invalid,
    #[error("value out of range for field width")]
    OutOfRange,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum YnError {
    #[error("expected Y or N")]
    Invalid,
}

pub fn parse_numeric_field(field: &[u8]) -> Result<u64, NumericError> {
    if field.is_empty() || !field.iter().all(|b| b.is_ascii_digit()) {
        return Err(NumericError::Invalid);
    }
    field
        .iter()
        .map(|&b| (b - b'0') as u64)
        .try_fold(0u64, |acc, d| acc.checked_mul(10).and_then(|a| a.checked_add(d)))
        .ok_or(NumericError::OutOfRange)
}

pub fn encode_numeric_field(value: u64, width: usize) -> Result<Vec<u8>, NumericError> {
    let max = 10u64
        .checked_pow(width as u32)
        .and_then(|p| p.checked_sub(1))
        .ok_or(NumericError::OutOfRange)?;
    if value > max {
        return Err(NumericError::OutOfRange);
    }
    Ok(format!("{value:0width$}").into_bytes())
}

pub fn parse_yn(byte: u8) -> Result<bool, YnError> {
    match byte {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        _ => Err(YnError::Invalid),
    }
}

pub fn yn_to_byte(value: bool) -> u8 {
    if value { b'Y' } else { b'N' }
}
