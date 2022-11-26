mod common;
pub mod purevpn;
pub mod surfshark;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum Protocol {
    TCP,
    UDP,
    Undetermined,
}

#[derive(Debug, Clone)]
pub struct Vpn {
    pub path: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub protocol: Protocol,
}

#[async_trait(?Send)]
pub trait VpnService {
    // async fn init() -> Result<()>;
    fn name() -> String;
    fn get_vpns(&self) -> Vec<Vpn>;
}
