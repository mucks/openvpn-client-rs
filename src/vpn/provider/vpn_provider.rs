use crate::{config::Config, vpn::Vpn};
use anyhow::Result;

use super::{nordvpn::NordVPN, purevpn::PureVPN, surfshark::Surfshark};

pub trait VpnProvider {
    fn name(&self) -> String;
    fn get_vpns(&self) -> Vec<Vpn>;
}

pub async fn get_providers() -> Result<Vec<Box<dyn VpnProvider>>> {
    let config = Config::new();

    let mut providers: Vec<Box<dyn VpnProvider>> = vec![];
    if config.surfshark_enabled {
        providers.push(Box::new(Surfshark::new().await?));
    }
    if config.purevpn_enabled {
        providers.push(Box::new(PureVPN::new().await?));
    }
    if config.nordvpn_enabled {
        providers.push(Box::new(NordVPN::new().await?));
    }
    Ok(providers)
}
