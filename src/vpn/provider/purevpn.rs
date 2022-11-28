use std::fs::remove_dir_all;

use anyhow::Result;

use super::{
    super::common::{add_auth_txt_to_openvpn_files_in_dir, create_auth_txt},
    vpn_provider::VpnProvider,
};
use crate::{
    config::Config,
    util::{dir_exists_with_content, download_and_extract_zip, move_all_files_from_dir_to},
    vpn::{common::get_vpns_from_dir, Vpn},
};

#[derive(Debug)]
pub struct PureVPN {
    name: String,
    path: String,
    vpns: Vec<Vpn>,
}

impl PureVPN {
    pub async fn new() -> Result<Self> {
        let mut purevpn = Self {
            name: "purevpn".into(),
            path: "./openvpn/purevpn".into(),
            vpns: Vec::new(),
        };
        purevpn.init().await?;

        Ok(purevpn)
    }
    // move all files in the 2 subdirs to self.path
    fn move_files_to_path(&self) -> Result<()> {
        let zip_name = "New+OVPN+Files";
        let unzipped_path = format!("{}/{}", self.path, zip_name);
        let tcp_path = format!("{}/TCP", unzipped_path);
        let udp_path = format!("{}/UDP", unzipped_path);

        move_all_files_from_dir_to(&tcp_path, &self.path)?;
        move_all_files_from_dir_to(&udp_path, &self.path)?;

        remove_dir_all(unzipped_path)?;
        Ok(())
    }

    async fn init(&mut self) -> Result<()> {
        if !dir_exists_with_content(&self.path) {
            let url = "https://d32d3g1fvkpl8y.cloudfront.net/heartbleed/windows/New+OVPN+Files.zip";
            download_and_extract_zip(url, &self.path).await?;
            self.move_files_to_path()?;
            add_auth_txt_to_openvpn_files_in_dir(&self.path)?;
        }
        let (user, pass) = Config::get_purevpn();
        create_auth_txt(&self.path, &user, &pass)?;
        self.vpns = get_vpns_from_dir(&self.name, &self.path)?;

        Ok(())
    }
}

impl VpnProvider for PureVPN {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn get_vpns(&self) -> Vec<crate::vpn::Vpn> {
        self.vpns.clone()
    }
}
