// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

pub(super) fn wait_master_completion(
    follower: &mut MasterFollower,
    deadline: Instant,
) -> Result<Value> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "MASTER_FOLLOWER_COMPLETION_DEADLINE"
        );
        let child = follower
            .child
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MASTER_FOLLOWER_CHILD_MISSING"))?;
        if let Some(status) = child.try_wait()? {
            ensure!(status.success(), "MASTER_FOLLOWER_CLI_FAILED");
            let mut bytes = Vec::new();
            File::open(&follower.stdout)?
                .take(16_385)
                .read_to_end(&mut bytes)?;
            ensure!(
                !bytes.is_empty() && bytes.len() <= 16_384,
                "MASTER_FOLLOWER_STATUS_BOUND"
            );
            let status: Value = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("MASTER_FOLLOWER_STATUS_JSON"))?;
            ensure!(status.is_object(), "MASTER_FOLLOWER_STATUS_JSON");
            follower.child = None;
            follower.process_start = None;
            return Ok(status);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
