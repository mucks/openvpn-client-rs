use crate::config::Config;

use super::{provider::vpn_provider::get_providers, Vpn};
use anyhow::Result;

pub struct VpnPool {
    vpns: Vec<Vpn>,
}

impl VpnPool {
    pub fn size(&self) -> usize {
        self.vpns.len()
    }
    pub async fn new() -> Result<Self> {
        let providers = get_providers().await?;

        let vpn_pool: Vec<Vpn> = providers
            .iter()
            .flat_map(|p| p.get_vpns())
            //filter by provider
            .filter(|v| {
                if let Some(provider) = Config::get_provider() {
                    v.provider == provider
                } else {
                    true
                }
            })
            //filter by protocol
            .filter(|v| {
                if let Some(protocol) = Config::get_protocol() {
                    v.protocol == protocol
                } else {
                    true
                }
            })
            //filter by country env
            //filter by city env
            .collect();

        Ok(Self { vpns: vpn_pool })
    }
    pub fn get_random(&self) -> Option<Vpn> {
        self.vpns.get(0).cloned()
    }
}
