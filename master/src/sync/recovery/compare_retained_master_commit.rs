// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use eve_storage::state::{StateSnapshot, read_snapshot_commit, snapshot_commit_encoded_length};

/// The caller retains the conservative storage-decode envelope before snapshot capture/read.
pub(super) fn compare_retained_master_commit(
    snapshot: &StateSnapshot<'_>,
    expected: &StateCommit,
    maximum_commit_bytes: usize,
) -> Result<()> {
    let length = snapshot_commit_encoded_length(snapshot, expected.target.height)?
        .context("MASTER_RETAINED_COMMIT_MISSING")?;
    ensure!(
        length <= maximum_commit_bytes,
        "MASTER_RETAINED_COMMIT_BYTE_CAPACITY"
    );
    let actual = read_snapshot_commit(snapshot, expected.target.height)?
        .context("MASTER_RETAINED_COMMIT_MISSING")?;
    ensure!(&actual == expected, "MASTER_RETAINED_WHOLE_COMMIT_MISMATCH");
    Ok(())
}
