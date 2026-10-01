// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{checksum_file::checksum_file, types::LocalSnapshotManifest};
use crate::state::StateStorageBudget;
use anyhow::{Context, Result, anyhow, ensure};
use eve_state::{StateCommit, compute_commit_identity, decode_state_commit, encode_state_commit};
use std::path::Path;

pub(crate) fn validate_snapshot_manifest(
    source: &Path,
    genesis: &StateCommit,
    budget: &StateStorageBudget,
) -> Result<(LocalSnapshotManifest, Vec<StateCommit>)> {
    let metadata = std::fs::symlink_metadata(source.join("manifest.json"))?;
    ensure!(
        metadata.is_file()
            && !metadata.file_type().is_symlink()
            && metadata.len() <= 16 * 1024 * 1024,
        "snapshot manifest is missing, linked or oversized"
    );
    let manifest: LocalSnapshotManifest =
        serde_json::from_slice(&std::fs::read(source.join("manifest.json"))?)?;
    ensure!(
        manifest.format_version == 1 && manifest.complete,
        "incomplete or unsupported snapshot format"
    );
    ensure!(
        manifest.references.len() <= budget.maximum_snapshot_files
            && u64::try_from(manifest.references.len())?
                == manifest
                    .height
                    .checked_add(1)
                    .context("snapshot height overflow")?,
        "snapshot reference count/height mismatch"
    );
    let expected_genesis = encode_state_commit(genesis, &budget.logical)
        .map_err(|error| anyhow!("invalid expected snapshot genesis: {error:?}"))?;
    let mut commits = Vec::new();
    let mut total = 0u64;
    let mut parent = None;
    for (index, reference) in manifest.references.iter().enumerate() {
        let height = u64::try_from(index)?;
        ensure!(
            reference.height == height && reference.file_name == format!("commit-{height:020}.rlp"),
            "reordered/duplicate/unsafe snapshot path"
        );
        let path = source.join(&reference.file_name);
        let metadata = std::fs::symlink_metadata(&path)?;
        ensure!(
            metadata.is_file()
                && !metadata.file_type().is_symlink()
                && metadata.len() == reference.length,
            "snapshot reference is missing, linked or length mismatched"
        );
        total = total
            .checked_add(reference.length)
            .context("snapshot declared byte overflow")?;
        ensure!(
            total <= u64::try_from(budget.maximum_snapshot_bytes)?
                && reference.length <= u64::try_from(budget.logical.maximum_commit_bytes)?,
            "snapshot declared bytes exceed budget"
        );
        ensure!(
            checksum_file(&path)? == reference.sha256,
            "snapshot reference checksum mismatch"
        );
        let bytes = std::fs::read(path)?;
        if height == 0 {
            ensure!(
                bytes == expected_genesis,
                "snapshot wrong genesis/config/schema/profile"
            );
        }
        let commit = decode_state_commit(&bytes, &budget.logical)
            .map_err(|error| anyhow!("invalid canonical snapshot commit: {error:?}"))?;
        ensure!(
            commit.target.height == height && commit.parent == parent,
            "snapshot parent/height recovery gap"
        );
        let identity = compute_commit_identity(&commit, &budget.logical)
            .map_err(|error| anyhow!("invalid snapshot commit identity: {error:?}"))?;
        ensure!(
            identity.0 == reference.commit_identity,
            "snapshot whole-commit identity mismatch"
        );
        parent = Some(commit.target.clone());
        commits.push(commit);
    }
    ensure!(
        total == manifest.total_bytes
            && manifest
                .references
                .first()
                .context("empty snapshot")?
                .commit_identity
                == manifest.genesis_commit_identity
            && manifest
                .references
                .last()
                .context("empty snapshot")?
                .commit_identity
                == manifest.head_commit_identity,
        "snapshot complete-reference summary mismatch"
    );
    Ok((manifest, commits))
}
