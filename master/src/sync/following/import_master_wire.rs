// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::persistence::persist_master_import;
use crate::sync::{
    MasterFollower, MasterSyncStatus, master_sync_status, recovery::prepare_master_import,
    resources::reserve_master_bytes, types::PreparationFailure,
};
use anyhow::{Result, ensure};
use eve_finality_verifier::imported_state_commit;
use eve_state::{StateVersion, encode_state_commit};

/// Import one borrowed untrusted V2 body. Authentication and exact advertised-target
/// matching precede all disk writes; ACK follows both proof and state actual syncs.
pub fn import_master_wire(
    follower: &mut MasterFollower,
    bytes: &[u8],
    advertised_target: &StateVersion,
) -> Result<MasterSyncStatus> {
    ensure!(!follower.fenced, "MASTER_FENCED_REOPEN_REQUIRED");
    ensure!(
        bytes.len() <= follower.config.maximum_proof_bytes,
        "MASTER_PROOF_BYTE_CAPACITY"
    );
    ensure!(
        follower.proof_count < follower.config.maximum_proof_files,
        "MASTER_PROOF_COUNT_CAPACITY"
    );
    let archive_bytes = follower
        .archive_bytes
        .checked_add(u64::try_from(bytes.len())?)
        .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
    ensure!(
        archive_bytes <= follower.config.maximum_archive_bytes,
        "MASTER_ARCHIVE_BYTE_CAPACITY"
    );
    let raw_lease = reserve_master_bytes(&follower.pool, bytes.len().max(1))?;
    let next = prepare_master_import(
        &follower.current.state,
        bytes,
        &follower.config.storage.logical,
        &follower.pool,
    )
    .map_err(|error| match error {
        PreparationFailure::Resource(error) | PreparationFailure::Invalid(error) => error,
    })?;
    let commit = imported_state_commit(&next.state);
    ensure!(
        &commit.target == advertised_target,
        "MASTER_ADVERTISED_TARGET_MISMATCH"
    );
    let commit_length = encode_state_commit(commit, &follower.config.storage.logical)
        .map_err(|error| anyhow::anyhow!("MASTER_COMMIT_MEASUREMENT: {error:?}"))?
        .len();
    let retained_commit_bytes = follower
        .retained_commit_bytes
        .checked_add(u64::try_from(commit_length)?)
        .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
    ensure!(
        retained_commit_bytes <= follower.config.maximum_retained_commit_bytes,
        "MASTER_COMMIT_ARCHIVE_BYTE_CAPACITY"
    );
    if let Err(error) = persist_master_import(follower, bytes, commit) {
        follower.fenced = true;
        return Err(error);
    }
    follower.current = next;
    follower.proof_count += 1;
    follower.archive_bytes = archive_bytes;
    follower.retained_commit_bytes = retained_commit_bytes;
    drop(raw_lease);
    Ok(master_sync_status(follower))
}
