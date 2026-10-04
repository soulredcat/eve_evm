// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PublicFollower;
use crate::support::process::{signal_owned_process, wait_child};
use anyhow::{Result, ensure};

pub(super) fn stop_public_follower(follower: &mut PublicFollower) -> Result<()> {
    if let Some(child) = follower.child.as_mut() {
        if child.try_wait()?.is_none() {
            signal_owned_process(
                child.id(),
                "TERM",
                follower
                    .process_start
                    .map(|start| (follower.binary.as_path(), start)),
            )?;
        }
        wait_child(child)?;
        ensure!(
            child.try_wait()?.is_some_and(|status| status.success()),
            "PUBLIC_FOLLOWER_GRACEFUL_STOP_FAILED"
        );
    }
    follower.child = None;
    follower.process_start = None;
    Ok(())
}
