#[macro_export]
macro_rules! dbg_hex {
    ($val:expr $(,)?) => {{
        let slice: &[u8] = $val;
        let hex = slice
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[dbg_hex] {} = {}", stringify!($val), hex);
    }};
}