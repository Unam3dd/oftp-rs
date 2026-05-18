pub mod ssid;
pub mod ssrm;

#[derive(Debug)]
pub enum OftpExchangeBuffer {
    Ssrm,
    Ssid,
}
