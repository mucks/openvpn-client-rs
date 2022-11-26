use vpn_services::{surfshark::Surfshark, VpnService};

use crate::vpn_services::purevpn::PureVPN;

mod config;
mod util;
mod vpn_services;

#[tokio::main]
async fn main() {
    // only run this in non production
    dotenvy::dotenv().ok();

    // let surfshark = Surfshark::new().await.unwrap();
    // println!("{:?}", surfshark);
    let purevpn = PureVPN::new().await.unwrap();
    println!("{:?}", purevpn);
}
