// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, validate_engine_peer_liveness};
use anyhow::{Result, ensure};
use std::process::Child;

/// Reconcile child lifecycle before/after socket I/O; never hold the child mutex across that I/O.
pub(in crate::consensus) fn validate_engine_peer_child(
    peer: &AuthenticatedEnginePeer,
    child: &mut Child,
) -> Result<()> {
    ensure!(
        peer.pid == child.id() && child.try_wait()?.is_none(),
        "authenticated engine child is no longer active"
    );
    validate_engine_peer_liveness(peer)?;
    ensure!(
        child.try_wait()?.is_none(),
        "engine child exited during lifecycle validation"
    );
    Ok(())
}
