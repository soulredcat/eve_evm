// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    checksum_file::checksum_file,
    sync_snapshot_directory::sync_snapshot_directory,
    types::{LocalSnapshotManifest, SnapshotCommitReference},
    write_synced_file::write_synced_file,
};
use crate::state::{
    StateSnapshot, encoding::height_key, read_snapshot_commit, types::COMMIT_PREFIX,
};
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::compute_commit_identity;
use std::path::Path;

/// Preserve source data; publish a complete manifest only after every referenced file is synced.
pub fn export_state_snapshot(
    snapshot: &StateSnapshot<'_>,
    target: &Path,
) -> Result<LocalSnapshotManifest> {
    let count = snapshot
        .version
        .height
        .checked_add(1)
        .context("snapshot height overflow")?;
    ensure!(
        count <= u64::try_from(snapshot.budget.maximum_snapshot_files)?,
        "snapshot reference count exceeds budget"
    );
    std::fs::create_dir(target).context("snapshot destination must be a NEW directory")?;
    let mut references = Vec::new();
    let mut total = 0u64;
    let mut head = None;
    for height in 0..=snapshot.version.height {
        let commit =
            read_snapshot_commit(snapshot, height)?.context("snapshot recovery chain gap")?;
        let bytes = snapshot
            .snapshot
            .get(height_key(COMMIT_PREFIX, height))?
            .context("snapshot payload disappeared")?;
        total = total
            .checked_add(u64::try_from(bytes.len())?)
            .context("snapshot byte count overflow")?;
        ensure!(
            total <= u64::try_from(snapshot.budget.maximum_snapshot_bytes)?,
            "snapshot byte budget exceeded; partial directory retained"
        );
        let file_name = format!("commit-{height:020}.rlp");
        let path = target.join(&file_name);
        write_synced_file(&path, &bytes)?;
        let identity = compute_commit_identity(&commit, &snapshot.budget.logical)
            .map_err(|error| anyhow!("invalid snapshot identity: {error:?}"))?;
        references.push(SnapshotCommitReference {
            height,
            file_name,
            length: u64::try_from(bytes.len())?,
            sha256: checksum_file(&path)?,
            commit_identity: identity.0,
        });
        head = Some(identity.0);
    }
    let genesis = compute_commit_identity(&snapshot.genesis, &snapshot.budget.logical)
        .map_err(|error| anyhow!("invalid snapshot genesis: {error:?}"))?;
    let manifest = LocalSnapshotManifest {
        format_version: 1,
        complete: true,
        captured_sequence: snapshot.database_sequence,
        height: snapshot.version.height,
        head_commit_identity: head.context("empty snapshot")?,
        genesis_commit_identity: genesis.0,
        total_bytes: total,
        references,
    };
    write_synced_file(
        &target.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest)?,
    )?;
    sync_snapshot_directory(target)?;
    Ok(manifest)
}
