// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use eve_storage::state::{
    capture_state_snapshot, development_state_storage_budget, open_state_repository,
    read_snapshot_commit, state_reader,
};

/// Inspect only after the actual CLI releases its exclusive repository ownership.
/// Exact complete commits include balances, nonces, fees, receipts and system state.
pub(super) fn compare_master_store(
    follower: &MasterFollower,
    oracle: &[StateCommit],
    height: u64,
) -> Result<()> {
    ensure!(
        follower.child.is_none() && height <= 128,
        "MASTER_FOLLOWER_STORE_INSPECTION_OWNERSHIP"
    );
    let genesis = oracle.first().context("MASTER_FOLLOWER_ORACLE_GENESIS")?;
    let repository = open_state_repository(
        &follower.data.join("state"),
        genesis,
        development_state_storage_budget(),
    )?;
    let reader = state_reader(&repository);
    let snapshot = capture_state_snapshot(&reader)?;
    ensure!(
        snapshot.version().height == height,
        "MASTER_FOLLOWER_ACTUAL_DB_HEIGHT"
    );
    for expected_height in 0..=height {
        let expected = oracle
            .get(usize::try_from(expected_height)?)
            .context("MASTER_FOLLOWER_ORACLE_HEIGHT")?;
        let retained = read_snapshot_commit(&snapshot, expected_height)?
            .context("MASTER_FOLLOWER_RETAINED_COMMIT_MISSING")?;
        ensure!(
            retained == *expected,
            "MASTER_FOLLOWER_ACTUAL_COMPLETE_COMMIT_OR_FEE_MISMATCH"
        );
    }
    Ok(())
}
