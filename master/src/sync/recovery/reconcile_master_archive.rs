// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    compare_retained_master_commit::compare_retained_master_commit,
    prepare_master_import::prepare_master_import,
    reconcile_master_staging::reconcile_master_staging,
};
use crate::sync::{
    MasterFollower,
    archive::{proof_file_name, read_charged_proof, sync_completed_proof},
    current_master_commit,
    types::{ArchiveInventory, PreparationFailure},
};
use anyhow::{Result, ensure};
use eve_finality_verifier::imported_state_commit;
use eve_state::encode_state_commit;
use eve_storage::state::{capture_state_snapshot, commit_state, snapshot_commit_encoded_length};

/// Replay authority from local genesis, not from the durable head/root. Every decoded
/// DB commit and proof stays within the owner's preleased storage/working envelopes.
pub(super) fn reconcile_master_archive(
    follower: &mut MasterFollower,
    inventory: ArchiveInventory,
) -> Result<()> {
    let snapshot = capture_state_snapshot(&follower.reader)?;
    let head = snapshot.version().height;
    ensure!(
        head <= follower.config.maximum_proof_files,
        "MASTER_DB_HEIGHT_CAPACITY"
    );
    ensure!(
        inventory.completed == head
            || inventory.completed
                == head
                    .checked_add(1)
                    .ok_or_else(|| anyhow::anyhow!("MASTER_HEIGHT_ARITHMETIC"))?,
        "MASTER_PROOF_DB_PREFIX_MISMATCH"
    );
    ensure!(
        inventory.completed == head || !inventory.staging,
        "MASTER_UNEXPLAINED_PROOF_AND_STAGING_SUFFIX"
    );
    compare_retained_master_commit(
        &snapshot,
        current_master_commit(follower),
        follower.config.storage.maximum_commit_bytes,
    )?;
    for height in 1..=inventory.completed {
        let proof = read_charged_proof(
            &follower.directory,
            &proof_file_name(height)?,
            follower.config.maximum_proof_bytes,
            &follower.pool,
        )?;
        let next = prepare_master_import(
            &follower.current.state,
            &proof.bytes,
            &follower.config.storage.logical,
            &follower.pool,
        )
        .map_err(|error| match error {
            PreparationFailure::Resource(error) | PreparationFailure::Invalid(error) => error,
        })?;
        let commit = imported_state_commit(&next.state);
        ensure!(commit.target.height == height, "MASTER_PROOF_HEIGHT");
        let length = if height <= head {
            snapshot_commit_encoded_length(&snapshot, height)?
                .ok_or_else(|| anyhow::anyhow!("MASTER_RETAINED_COMMIT_MISSING"))?
        } else {
            encode_state_commit(commit, &follower.config.storage.logical)
                .map_err(|error| anyhow::anyhow!("MASTER_COMMIT_MEASUREMENT: {error:?}"))?
                .len()
        };
        let retained_commit_bytes = follower
            .retained_commit_bytes
            .checked_add(u64::try_from(length)?)
            .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
        ensure!(
            retained_commit_bytes <= follower.config.maximum_retained_commit_bytes,
            "MASTER_COMMIT_ARCHIVE_BYTE_CAPACITY"
        );
        if height <= head {
            compare_retained_master_commit(
                &snapshot,
                commit,
                follower.config.storage.maximum_commit_bytes,
            )?;
        } else {
            sync_completed_proof(&follower.directory, height)?;
            let ack = commit_state(&mut follower.repository, commit)?;
            ensure!(
                ack.committed == commit.target && ack.store_head == commit.target,
                "MASTER_RECONCILIATION_ACK_MISMATCH"
            );
        }
        follower.current = next;
        follower.retained_commit_bytes = retained_commit_bytes;
        drop(proof);
    }
    drop(snapshot);
    if inventory.staging {
        reconcile_master_staging(follower)?;
    }
    Ok(())
}
