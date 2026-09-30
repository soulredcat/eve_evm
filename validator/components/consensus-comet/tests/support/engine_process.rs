use std::{
    fs::File,
    net::SocketAddr,
    path::Path,
    process::{Child, Command, Stdio},
};

use anyhow::{Context, Result};

pub struct EngineProcess(pub Child);

impl Drop for EngineProcess {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

pub fn start_engine(
    binary: &Path,
    home: &Path,
    application: SocketAddr,
    rpc: SocketAddr,
    p2p: SocketAddr,
    log_path: &Path,
) -> Result<EngineProcess> {
    let log = File::create(log_path)?;
    let error_log = log.try_clone()?;
    let process = Command::new(binary)
        .args(["start", "--home"])
        .arg(home)
        .args(["--proxy_app", &format!("tcp://{application}")])
        .args(["--abci", "socket", "--rpc.laddr", &format!("tcp://{rpc}")])
        .args([
            "--p2p.laddr",
            &format!("tcp://{p2p}"),
            "--log_level",
            "error",
        ])
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(error_log))
        .spawn()
        .context("start pinned CometBFT API-test process")?;
    Ok(EngineProcess(process))
}
