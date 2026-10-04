// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PublicFollower;
use crate::support::process::{process_start, write_private_file};
use anyhow::{Result, ensure};
use std::{
    fs::File,
    process::{Command, Stdio},
};

pub(super) fn spawn_public_follower(follower: &mut PublicFollower) -> Result<()> {
    ensure!(follower.child.is_none(), "PUBLIC_FOLLOWER_ALREADY_RUNNING");
    let launch = follower
        .launch_count
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_LAUNCH_LIMIT"))?;
    ensure!(launch <= 2, "PUBLIC_FOLLOWER_LAUNCH_LIMIT");
    follower.launch_count = launch;
    follower.stdout = follower
        .stdout
        .with_file_name(format!("public-follower-{launch}.stdout"));
    follower.stderr = follower
        .stderr
        .with_file_name(format!("public-follower-{launch}.stderr"));
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
        .arg("--acknowledge-unsafe-development")
        .arg("--validator-address")
        .arg(follower.validator.to_string())
        .arg("--http-address")
        .arg(follower.http.to_string())
        .arg("--ws-address")
        .arg(follower.ws.to_string())
        .arg("--node-name")
        .arg("acceptance-public")
        .arg("--zone-id")
        .arg("1")
        .arg("--poll-interval-ms")
        .arg("1000")
        .stdin(Stdio::null())
        .stdout(File::create(&follower.stdout)?)
        .stderr(File::create(&follower.stderr)?)
        .spawn()?;
    let pid = child.id();
    follower.child = Some(child);
    follower.process_start = Some(process_start(pid)?);
    Ok(())
}
