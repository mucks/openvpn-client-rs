use anyhow::{anyhow, Result};
use std::{
    process::Stdio,
    time::{Duration, Instant},
};

use tokio::{
    io::AsyncReadExt,
    process::{Child, Command},
};

use super::Vpn;

#[derive(Debug, Default)]
pub struct VpnClient {
    openvpn_process: Option<Child>,
}

impl VpnClient {
    pub async fn connect_to(&mut self, vpn: &Vpn) -> Result<()> {
        println!("connecting to {:#?}", vpn);

        let mut cmd = Command::new("openvpn")
            .arg(&vpn.path)
            .stdout(Stdio::piped())
            .spawn()?;

        let timeout = Duration::from_secs(20);
        let now = Instant::now();

        loop {
            if let Some(out) = &mut cmd.stdout.take() {
                let mut s = String::new();
                out.read_to_string(&mut s).await?;
                println!("{}", s);
                if s.contains("tun: Operation not permitted") {
                    return Err(anyhow!("require sudo to run openvpn"));
                }

                if s.contains("ERROR: ") {
                    return Err(anyhow!("error connecting to vpn: {:?}", vpn));
                }
                if s.contains("AUTH_FAILED") {
                    return Err(anyhow!("invalid user/password for {}", vpn.provider));
                }

                if s.contains("Initializing sequence complete") {
                    println!("successfully connected to {:?}", vpn);
                    break;
                }
            }
            if now.elapsed() > timeout {
                return Err(anyhow!("openvpn connect timeout"));
            }
        }

        self.openvpn_process = Some(cmd);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(child) = &mut self.openvpn_process {
            child.kill().await?;
            self.openvpn_process = None;
        }
        Ok(())
    }
}
