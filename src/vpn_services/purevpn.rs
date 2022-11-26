use std::{
    fs::{self, create_dir_all, read_dir, remove_dir, remove_dir_all, remove_file, File},
    io::Write,
};

use anyhow::Result;

use super::common::{add_auth_txt_to_openvpn_files_in_dir, create_auth_txt};
use crate::{
    config::Config,
    util::{download_and_extract_zip, move_all_files_from_dir_to},
};

#[derive(Debug)]
pub struct PureVPN {
    path: String,
}

impl PureVPN {
    pub async fn new() -> Result<Self> {
        let purevpn = Self {
            path: "./openvpn/purevpn".into(),
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

    async fn init(&self) -> Result<()> {
        let url = "https://d32d3g1fvkpl8y.cloudfront.net/heartbleed/windows/New+OVPN+Files.zip";

        download_and_extract_zip(url, &self.path).await?;
        self.move_files_to_path()?;
        add_auth_txt_to_openvpn_files_in_dir(&self.path)?;
        let (user, pass) = Config::get_purevpn();
        create_auth_txt(&self.path, &user, &pass)?;

        Ok(())
    }
}
