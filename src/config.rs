use std::env::var;

use crate::vpn::protocol::Protocol;

pub struct Config {
    pub surfshark_enabled: bool,
    pub purevpn_enabled: bool,
    pub nordvpn_enabled: bool,
}

fn env_expect(s: &str) -> String {
    var(s).unwrap_or_else(|_| panic!("{} ENV not set", s))
}

impl Config {
    pub fn new() -> Self {
        Self {
            surfshark_enabled: var("SURFSHARK_USER").is_ok(),
            purevpn_enabled: var("PUREVPN_USER").is_ok(),
            nordvpn_enabled: var("NORDVPN_USER").is_ok(),
        }
    }

    pub fn get_protocol() -> Option<Protocol> {
        if let Ok(p) = var("PROTOCOL") {
            Some(Protocol::from(p))
        } else {
            None
        }
    }

    pub fn get_provider() -> Option<String> {
        var("PROVIDER").ok()
    }
    pub fn get_surfshark() -> (String, String) {
        (
            env_expect("SURFSHARK_USER"),
            env_expect("SURFSHARK_PASSWORD"),
        )
    }
    pub fn get_purevpn() -> (String, String) {
        (env_expect("PUREVPN_USER"), env_expect("PUREVPN_PASSWORD"))
    }
    pub fn get_nordvpn() -> (String, String) {
        (env_expect("NORDVPN_USER"), env_expect("NORDVPN_PASSWORD"))
    }
}
