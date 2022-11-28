use std::io::Write;

use anyhow::{anyhow, Result};
use vpn::{vpn_client::VpnClient, vpn_pool::VpnPool};

mod config;
mod util;
mod vpn;

fn set_resolv_conf() -> Result<()> {
    let ns_one = "8.8.8.8";
    let ns_two = "8.8.4.4";
    let content = format!("nameserver {}\nnameserver {}\n", ns_one, ns_two);
    let mut f = std::fs::File::create("/etc/resolv.conf")?;
    let mut perms = f.metadata()?.permissions();
    perms.set_readonly(true);
    f.set_permissions(perms)?;
    f.write_all(content.as_bytes())?;
    Ok(())
}

async fn run() -> Result<()> {
    println!("starting openvpn-client-rs");
    set_resolv_conf().ok();

    let vpn_pool = VpnPool::new().await?;
    println!("VpnPool has {} available vpns", vpn_pool.size());
    let mut vpn_client = VpnClient::default();
    if let Some(vpn) = vpn_pool.get_random() {
        vpn_client.connect_to(&vpn).await?;
        vpn_client.wait().await?;
    } else {
        return Err(anyhow!("Could not find vpn with the configurations in env"));
    }
    Ok(())
}

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().ok();
    run().await.unwrap();
}
