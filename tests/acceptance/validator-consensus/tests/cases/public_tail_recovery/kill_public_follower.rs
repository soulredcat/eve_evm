// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::public_follower::PublicFollower;
use crate::support::process::{signal_owned_process, wait_child};
use anyhow::{Context, Result, ensure};
use std::os::unix::process::ExitStatusExt;

/// Kill only this fixture's actual child after checking its executable/start
/// identity. Reaping SIGKILL is process-crash evidence, not hardware power loss.
pub(in crate::cases) fn kill_public_follower(follower: &mut PublicFollower) -> Result<()> {
    let start = follower
        .process_start
        .context("PUBLIC_TAIL_PROCESS_START")?;
    let child = follower
        .child
        .as_mut()
        .context("PUBLIC_TAIL_CHILD_MISSING")?;
    ensure!(
        child.try_wait()?.is_none(),
        "PUBLIC_TAIL_CHILD_ALREADY_EXITED"
    );
    signal_owned_process(child.id(), "KILL", Some((follower.binary.as_path(), start)))?;
    wait_child(child)?;
    ensure!(
        child
            .try_wait()?
            .is_some_and(|status| status.signal() == Some(9)),
        "PUBLIC_TAIL_DID_NOT_EXIT_BY_SIGKILL"
    );
    follower.child = None;
    follower.process_start = None;
    Ok(())
}
