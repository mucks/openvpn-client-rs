use super::super::common::{
    add_auth_txt_to_openvpn_files_in_dir, create_auth_txt, get_openvpn_files,
};
use super::super::protocol::Protocol;
use super::vpn_provider::VpnProvider;
use crate::util::{dir_exists_with_content, download_and_extract_zip};
use crate::vpn::common::get_vpns_from_dir;
use crate::vpn::Vpn;
use anyhow::Result;

use crate::config::Config;

#[derive(Debug)]
pub struct Surfshark {
    name: String,
    path: String,
    vpns: Vec<Vpn>,
}

impl Surfshark {
    pub async fn new() -> Result<Self> {
        let mut surfshark = Self {
            name: "surfshark".into(),
            path: "./openvpn/surfshark".into(),
            vpns: Vec::new(),
        };
        surfshark.init().await?;

        Ok(surfshark)
    }

    async fn init(&mut self) -> Result<()> {
        if !dir_exists_with_content(&self.path) {
            let url = "https://my.surfshark.com/vpn/api/v1/server/configurations";
            download_and_extract_zip(url, &self.path).await?;
            add_auth_txt_to_openvpn_files_in_dir(&self.path)?;
        }
        let (user, pass) = Config::get_surfshark();
        create_auth_txt(&self.path, &user, &pass)?;
        self.vpns = get_vpns_from_dir(&self.name, &self.path)?;

        Ok(())
    }
}

impl VpnProvider for Surfshark {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_vpns(&self) -> Vec<Vpn> {
        self.vpns.clone()
    }
}
