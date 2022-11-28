use anyhow::{anyhow, Result};
use std::{
    pin::Pin,
    process::Stdio,
    time::{Duration, Instant},
};

use tokio::{
    io::AsyncReadExt,
    process::{Child, Command},
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};

use super::Vpn;

#[derive(Debug, Default)]
pub struct VpnClient {
    openvpn_process: Option<JoinHandle<Result<()>>>,
}

use process_stream::{Process, ProcessExt, ProcessItem, Stream, StreamExt};

type MyStream = Pin<Box<dyn Stream<Item = ProcessItem> + Send>>;

async fn openvpn_wait_for_init(stream: &mut MyStream, vpn: &Vpn) -> Result<()> {
    while let Some(out) = stream.next().await {
        println!("{}", out);
        if out.contains("tun: Operation not permitted") {
            return Err(anyhow!("require sudo to run openvpn"));
        }

        if out.contains("ERROR: ") {
            return Err(anyhow!("error connecting to vpn: {:?}", vpn));
        }
        if out.contains("AUTH_FAILED") {
            return Err(anyhow!("invalid user/password for {}", vpn.provider));
        }

        if out.contains("Initialization Sequence Completed") {
            return Ok(());
        }
    }
    Err(anyhow!("openvpn process ended"))
}

async fn connect_to_openvpn(vpn: &Vpn, tx: Sender<Result<()>>) -> Result<()> {
    let mut cmd: Process = vec!["openvpn", vpn.path.as_str()].into();

    let mut stream = cmd.spawn_and_stream()?;

    let result = openvpn_wait_for_init(&mut stream, vpn).await;
    tx.send(result)
        .expect("tx in fn connect_to_openvpn in vpn_client failed to send");

    while let Some(out) = stream.next().await {
        println!("{}", out);
    }

    Ok(())
}

impl VpnClient {
    pub async fn connect_to(&mut self, vpn: &Vpn) -> Result<()> {
        println!("connecting to {:#?}", vpn);

        let vpn_clone = vpn.clone();
        let (tx, rx) = oneshot::channel::<Result<()>>();
        let handle = tokio::task::spawn(async move { connect_to_openvpn(&vpn_clone, tx).await });

        // wait for openvpn to initialize
        rx.await??;

        println!("successfully connected!");

        self.openvpn_process = Some(handle);
        Ok(())
    }

    pub async fn wait(self) -> Result<()> {
        if let Some(process) = self.openvpn_process {
            process.await??;
        }
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(handle) = &mut self.openvpn_process {
            handle.abort();
            self.openvpn_process = None;
        }
        Ok(())
    }
}
