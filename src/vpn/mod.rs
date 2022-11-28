mod common;
pub mod protocol;
pub mod provider;
pub mod vpn_client;
pub mod vpn_pool;

use self::protocol::Protocol;

#[derive(Debug, Clone)]
pub struct Vpn {
    pub provider: String,
    pub path: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub protocol: Protocol,
}
