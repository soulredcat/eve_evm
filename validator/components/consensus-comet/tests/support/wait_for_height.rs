// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::rpc_json;
use anyhow::{Context, Result, ensure};
use std::{
    net::SocketAddr,
    process::Child,
    thread,
    time::{Duration, Instant},
};

/// Observe the engine height; the separate fixture-marker wait proves checkpoint publication.
pub fn wait_for_height(address: SocketAddr, height: i64, process: &mut Child) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(40);
    while Instant::now() < deadline {
        ensure!(
            process.try_wait()?.is_none(),
            "CometBFT exited before API smoke completed"
        );
        if let Ok(status) = rpc_json(address, "/status") {
            let current: i64 = status["sync_info"]["latest_block_height"]
                .as_str()
                .context("RPC latest height")?
                .parse()?;
            if current >= height {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    anyhow::bail!("actual CometBFT engine did not reach required height {height}")
}
