use super::common::{add_auth_txt_to_openvpn_files_in_dir, create_auth_txt, get_openvpn_files};
use crate::util::{dir_exists_with_content, download_and_extract_zip};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::fs::File;
use std::io::{Read, Write};

use super::{Protocol, Vpn, VpnService};
use crate::config::Config;

#[derive(Debug)]
pub struct Surfshark {
    path: String,
    vpns: Vec<Vpn>,
}

impl Surfshark {
    pub async fn new() -> Result<Self> {
        let mut surfshark = Self {
            path: "./openvpn/surfshark".into(),
            vpns: Vec::new(),
        };
        surfshark.init().await?;

        Ok(surfshark)
    }

    async fn init(&mut self) -> Result<()> {
        if dir_exists_with_content(&self.path) {
            self.download_and_adjust_configs().await?;
        }
        let (user, pass) = Config::get_surfshark();
        create_auth_txt(&self.path, &user, &pass)?;
        self.get_vpns_from_file()?;

        Ok(())
    }

    fn get_vpns_from_file(&mut self) -> Result<()> {
        let mut vpns = vec![];

        for vpn in get_openvpn_files(&self.path)? {
            let mut city = None;
            let mut country = None;

            // if s.contains("-") {
            //     let split: Vec<&str> = s.split("-").collect();
            //     city = split.get(0).map(|f| f.to_owned().to_owned());
            //     country = split.get(1).map(|f| f.to_owned().to_owned());
            // }
            let protocol = if vpn.contains("tcp.ovpn") {
                Protocol::TCP
            } else if vpn.contains("udp.ovpn") {
                Protocol::UDP
            } else {
                Protocol::Undetermined
            };
            vpns.push(Vpn {
                path: vpn,
                city,
                country,
                protocol,
            });
        }

        self.vpns = vpns;

        Ok(())
    }

    async fn download_and_adjust_configs(&self) -> Result<()> {
        let url = "https://my.surfshark.com/vpn/api/v1/server/configurations";

        download_and_extract_zip(url, &self.path).await?;
        add_auth_txt_to_openvpn_files_in_dir(&self.path)?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl VpnService for Surfshark {
    fn name() -> String {
        "Surfshark".into()
    }

    fn get_vpns(&self) -> Vec<Vpn> {
        self.vpns.clone()
    }
}
