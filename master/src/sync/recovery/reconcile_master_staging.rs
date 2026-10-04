// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::prepare_master_import::prepare_master_import;
use crate::sync::{
    MasterFollower,
    archive::{STAGING, promote_staged_proof, read_charged_proof, reject_staged_proof},
    current_master_commit,
    types::PreparationFailure,
};
use anyhow::{Result, ensure};
use eve_finality_verifier::imported_state_commit;
use eve_state::encode_state_commit;
use eve_storage::state::commit_state;

/// One uncommitted staging body can be authenticated/promoted, or preserved in one
/// rejected slot. Capacity pressure alone never reclassifies a valid proof as malformed.
pub(super) fn reconcile_master_staging(follower: &mut MasterFollower) -> Result<()> {
    let proof = read_charged_proof(
        &follower.directory,
        STAGING,
        follower.config.maximum_proof_bytes,
        &follower.pool,
    )?;
    let next = match prepare_master_import(
        &follower.current.state,
        &proof.bytes,
        &follower.config.storage.logical,
        &follower.pool,
    ) {
        Ok(next) => next,
        Err(PreparationFailure::Resource(error)) => return Err(error),
        Err(PreparationFailure::Invalid(_)) => {
            ensure!(
                !follower.rejected_staging,
                "MASTER_REJECTED_STAGING_SLOT_OCCUPIED"
            );
            reject_staged_proof(&follower.directory)?;
            follower.rejected_staging = true;
            return Ok(());
        }
    };
    ensure!(
        follower.proof_count < follower.config.maximum_proof_files,
        "MASTER_PROOF_COUNT_CAPACITY"
    );
    let height = current_master_commit(follower)
        .target
        .height
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("MASTER_HEIGHT_ARITHMETIC"))?;
    let commit = imported_state_commit(&next.state);
    ensure!(commit.target.height == height, "MASTER_STAGING_HEIGHT");
    let length = encode_state_commit(commit, &follower.config.storage.logical)
        .map_err(|error| anyhow::anyhow!("MASTER_COMMIT_MEASUREMENT: {error:?}"))?
        .len();
    let retained_commit_bytes = follower
        .retained_commit_bytes
        .checked_add(u64::try_from(length)?)
        .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
    ensure!(
        retained_commit_bytes <= follower.config.maximum_retained_commit_bytes,
        "MASTER_COMMIT_ARCHIVE_BYTE_CAPACITY"
    );
    promote_staged_proof(&follower.directory, height)?;
    let ack = commit_state(&mut follower.repository, commit)?;
    ensure!(
        ack.committed == commit.target && ack.store_head == commit.target,
        "MASTER_STAGING_ACK_MISMATCH"
    );
    follower.proof_count += 1;
    follower.current = next;
    follower.retained_commit_bytes = retained_commit_bytes;
    Ok(())
}
