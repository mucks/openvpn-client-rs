use std::fs::remove_dir_all;

use crate::{
    config::Config,
    util::{dir_exists_with_content, download_and_extract_zip, move_all_files_from_dir_to},
    vpn::{
        common::{add_auth_txt_to_openvpn_files_in_dir, create_auth_txt, get_vpns_from_dir},
        Vpn,
    },
};
use anyhow::Result;

use super::vpn_provider::VpnProvider;

pub struct NordVPN {
    name: String,
    path: String,
    vpns: Vec<Vpn>,
}

impl NordVPN {
    pub async fn new() -> Result<Self> {
        let mut nordvpn = Self {
            name: "nordvpn".into(),
            path: "./openvpn/nordvpn".into(),
            vpns: Vec::new(),
        };
        nordvpn.init().await?;
        Ok(nordvpn)
    }
    fn move_files_to_path(&self) -> Result<()> {
        let tcp_path = format!("{}/ovpn_tcp", self.path);
        let udp_path = format!("{}/ovpn_udp", self.path);

        move_all_files_from_dir_to(&tcp_path, &self.path)?;
        move_all_files_from_dir_to(&udp_path, &self.path)?;

        remove_dir_all(tcp_path)?;
        remove_dir_all(udp_path)?;

        Ok(())
    }

    async fn init(&mut self) -> Result<()> {
        if !dir_exists_with_content(&self.path) {
            let url = "https://downloads.nordcdn.com/configs/archives/servers/ovpn.zip";
            download_and_extract_zip(url, &self.path).await?;
            self.move_files_to_path()?;
            add_auth_txt_to_openvpn_files_in_dir(&self.path)?;
        }
        let (user, pass) = Config::get_nordvpn();
        create_auth_txt(&self.path, &user, &pass)?;
        self.vpns = get_vpns_from_dir(&self.name, &self.path)?;

        Ok(())
    }
}

impl VpnProvider for NordVPN {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_vpns(&self) -> Vec<Vpn> {
        self.vpns.clone()
    }
}
