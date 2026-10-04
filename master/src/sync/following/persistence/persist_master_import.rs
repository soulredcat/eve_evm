// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::{
    MasterFollower,
    archive::{promote_staged_proof, write_staged_proof},
};
use anyhow::{Result, ensure};
use eve_state::StateCommit;
use eve_storage::state::commit_state;

/// Persist the already authenticated private candidate in proof-before-state order.
/// The caller holds raw/candidate/storage leases and fences every failed outcome.
pub(in crate::sync::following) fn persist_master_import(
    follower: &mut MasterFollower,
    bytes: &[u8],
    commit: &StateCommit,
) -> Result<()> {
    #[cfg(test)]
    let fault = follower.fault.take();
    #[cfg(test)]
    if fault == Some(crate::sync::types::MasterSyncFault::PartialStaging) {
        write_staged_proof(&follower.directory, &bytes[..bytes.len() / 2])?;
        anyhow::bail!("SIMULATED partial uncommitted staging; no physical power-loss claim");
    }
    write_staged_proof(&follower.directory, bytes)?;
    promote_staged_proof(&follower.directory, commit.target.height)?;
    #[cfg(test)]
    if fault == Some(crate::sync::types::MasterSyncFault::AfterProofSync) {
        anyhow::bail!("SIMULATED loss after actual proof sync before state commit");
    }
    let ack = commit_state(&mut follower.repository, commit)?;
    #[cfg(test)]
    if fault == Some(crate::sync::types::MasterSyncFault::AfterStateSync) {
        anyhow::bail!("SIMULATED lost acknowledgement after actual state sync");
    }
    ensure!(
        ack.committed == commit.target && ack.store_head == commit.target,
        "MASTER_DURABLE_ACK_MISMATCH"
    );
    Ok(())
}
