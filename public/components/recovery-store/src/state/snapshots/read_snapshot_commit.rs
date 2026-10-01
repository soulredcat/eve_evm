// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    StateSnapshot, encoding::height_key, repository::validation::validate_snapshot_rows,
    types::COMMIT_PREFIX,
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{StateCommit, compute_commit_identity, decode_state_commit};

pub fn read_snapshot_commit(
    snapshot: &StateSnapshot<'_>,
    height: u64,
) -> Result<Option<StateCommit>> {
    if height > snapshot.version.height {
        return Ok(None);
    }
    let bytes = snapshot
        .snapshot
        .get(height_key(COMMIT_PREFIX, height))?
        .context("missing retained snapshot recovery commit")?;
    let commit = decode_state_commit(&bytes, &snapshot.budget.logical)
        .map_err(|error| anyhow!("corrupt snapshot recovery commit: {error:?}"))?;
    ensure!(
        commit.target.height == height
            && commit.target.identity == snapshot.genesis.target.identity,
        "snapshot retained commit namespace/height mismatch"
    );
    let identity = compute_commit_identity(&commit, &snapshot.budget.logical)
        .map_err(|error| anyhow!("invalid snapshot commit identity: {error:?}"))?;
    validate_snapshot_rows(
        &snapshot.snapshot,
        &commit,
        bytes,
        identity,
        height == snapshot.version.height,
    )?;
    Ok(Some(commit))
}
