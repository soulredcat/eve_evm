// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use crate::support::process::{process_start, write_private_file};
use anyhow::{Result, ensure};
use std::{
    fs::OpenOptions,
    process::{Command, Stdio},
};

pub(super) fn spawn_master_follower(follower: &mut MasterFollower, height: u64) -> Result<()> {
    ensure!(follower.child.is_none(), "MASTER_FOLLOWER_ALREADY_RUNNING");
    ensure!((1..=128).contains(&height), "MASTER_FOLLOWER_HEIGHT_BOUND");
    let launch = follower
        .launch_count
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("MASTER_FOLLOWER_LAUNCH_LIMIT"))?;
    ensure!(launch <= 2, "MASTER_FOLLOWER_LAUNCH_LIMIT");
    follower.launch_count = launch;
    follower.stdout = follower
        .artifact
        .join(format!("master-follower-{launch}.stdout"));
    follower.stderr = follower
        .artifact
        .join(format!("master-follower-{launch}.stderr"));
    write_private_file(&follower.stdout, &[])?;
    write_private_file(&follower.stderr, &[])?;
    let child = Command::new(&follower.binary)
        .arg("follow-dev")
        .arg("--root")
        .arg(&follower.repository_root)
        .arg("--data")
        .arg(&follower.data)
        .arg("--genesis")
        .arg(&follower.genesis)
        .arg("--mode")
        .arg("MASTER_SYNC_ONLY")
        .arg("--acknowledge-unsafe-development")
        .arg("--native-address")
        .arg(follower.validator.to_string())
        .arg("--through-height")
        .arg(height.to_string())
        .stdin(Stdio::null())
        .stdout(OpenOptions::new().write(true).open(&follower.stdout)?)
        .stderr(OpenOptions::new().write(true).open(&follower.stderr)?)
        .spawn()?;
    let pid = child.id();
    follower.child = Some(child);
    follower.process_start = Some(process_start(pid)?);
    Ok(())
}
