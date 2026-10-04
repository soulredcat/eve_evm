// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    reserve_checkpoint_read::reserve_checkpoint_read,
    types::{CHECKPOINT_FIXTURE_BODY_LIMIT, CheckpointReadBudget},
};
use crate::cases::public_follower::PublicFollower;
use anyhow::{Result, ensure};
use eve_state::{
    StateCommit, decode_preflight_state_commit, development_state_budget, encode_state_commit,
    encode_state_version, measure_complete_state_bytes, preflight_state_commit,
    required_state_commit_decode_reservation,
};
use eve_storage::checkpoints::{
    CheckpointLimits, checkpoint_body_bytes, checkpoint_manifest_id, create_checkpoint_manifest,
    open_completed_checkpoint_store, preflight_checkpoint_body, preflight_checkpoint_manifest,
    read_checkpoint_body, required_checkpoint_body_reservation, required_checkpoint_io_reservation,
    required_checkpoint_metadata_reservation,
};
use std::{cell::Cell, fs::File, os::unix::fs::MetadataExt};

/// Inspect only a reaped child's immutable completed content. Canonical native
/// replay supplies the independent oracle; file checksums grant no finality.
pub(in crate::cases) fn verify_stored_checkpoint(
    follower: &PublicFollower,
    expected: &StateCommit,
) -> Result<[u8; 32]> {
    ensure!(
        follower.child.is_none(),
        "PUBLIC_CHECKPOINT_READ_REQUIRES_REAPED_CHILD"
    );
    let budget = CheckpointReadBudget { used: Cell::new(0) };
    let logical = development_state_budget();
    ensure!(
        measure_complete_state_bytes(&expected.state, &logical)
            .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_ORACLE_MEASURE: {error:?}"))?
            <= CHECKPOINT_FIXTURE_BODY_LIMIT / 2,
        "PUBLIC_CHECKPOINT_ORACLE_STATE_BOUND"
    );
    ensure!(
        expected.block.transactions.len() <= 256
            && expected.block.receipts.len() <= 256
            && expected
                .block
                .transactions
                .iter()
                .all(|bytes| bytes.len() <= 262_144)
            && expected
                .block
                .receipts
                .iter()
                .all(|bytes| bytes.len() <= 262_144),
        "PUBLIC_CHECKPOINT_ORACLE_BLOCK_BOUND"
    );
    let raw_bytes = expected
        .block
        .transactions
        .iter()
        .chain(&expected.block.receipts)
        .try_fold(0_usize, |sum, bytes| sum.checked_add(bytes.len()));
    ensure!(
        raw_bytes.is_some_and(|bytes| bytes <= CHECKPOINT_FIXTURE_BODY_LIMIT / 4),
        "PUBLIC_CHECKPOINT_ORACLE_RAW_BYTES_BOUND"
    );
    // The known small fixture is charged before canonical oracle encoding;
    // observed body size and actual decode counts are checked separately.
    let _oracle_lease = reserve_checkpoint_read(&budget, 8 * CHECKPOINT_FIXTURE_BODY_LIMIT)?;
    let expected_bytes = encode_state_commit(expected, &logical)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_ORACLE_ENCODING: {error:?}"))?;
    ensure!(
        expected_bytes.len() <= CHECKPOINT_FIXTURE_BODY_LIMIT,
        "PUBLIC_CHECKPOINT_FIXTURE_BODY_BOUND"
    );
    let limits = CheckpointLimits {
        logical,
        maximum_body_bytes: CHECKPOINT_FIXTURE_BODY_LIMIT,
        maximum_chunk_bytes: 32_768,
        maximum_chunks: 4_096,
        maximum_manifest_bytes: 262_144,
    };
    let metadata = required_checkpoint_metadata_reservation(&limits)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_METADATA_REQUIREMENT: {error:?}"))?;
    let _metadata_lease = reserve_checkpoint_read(&budget, metadata)?;
    let preflight = preflight_state_commit(&expected_bytes, &logical)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_ORACLE_PREFLIGHT: {error:?}"))?;
    let manifest = create_checkpoint_manifest(&preflight, &expected.target, &limits, metadata)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_MANIFEST: {error:?}"))?;
    let target = encode_state_version(&expected.target)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_VERSION: {error:?}"))?;
    let manifest_preflight = preflight_checkpoint_manifest(&manifest, &target, &limits)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_MANIFEST_PREFLIGHT: {error:?}"))?;
    let id = checkpoint_manifest_id(&manifest_preflight);
    let content_path = follower.data.join(".checkpoints/content");
    let proof_path = follower.data.join(".checkpoints/proofs");
    for path in [&content_path, &proof_path] {
        let metadata = std::fs::symlink_metadata(path)?;
        ensure!(
            metadata.is_dir() && !metadata.file_type().is_symlink() && metadata.mode() & 0o077 == 0,
            "PUBLIC_CHECKPOINT_PRIVATE_ARTIFACT_DIRECTORY"
        );
    }
    let content = File::open(content_path)?;
    let io = required_checkpoint_io_reservation(&limits)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_IO_REQUIREMENT: {error:?}"))?;
    let _io_lease = reserve_checkpoint_read(&budget, io)?;
    let store =
        open_completed_checkpoint_store(&content, &id, &expected.target, &limits, metadata, io)
            .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_COMPLETED_STORE: {error:?}"))?;
    let body_bytes = required_checkpoint_body_reservation(&store)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_BODY_REQUIREMENT: {error:?}"))?;
    ensure!(
        body_bytes <= CHECKPOINT_FIXTURE_BODY_LIMIT + io,
        "PUBLIC_CHECKPOINT_STORED_BODY_BOUND"
    );
    let _body_lease = reserve_checkpoint_read(&budget, body_bytes)?;
    let body = read_checkpoint_body(&store, body_bytes)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_BODY: {error:?}"))?;
    ensure!(
        checkpoint_body_bytes(&body) == expected_bytes.as_slice(),
        "PUBLIC_CHECKPOINT_COMPLETE_BYTES_MISMATCH"
    );
    let stored_preflight = preflight_checkpoint_body(&body)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_STORED_PREFLIGHT: {error:?}"))?;
    let decode = required_state_commit_decode_reservation(&stored_preflight)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_DECODE_REQUIREMENT: {error:?}"))?;
    let _decode_lease = reserve_checkpoint_read(&budget, decode)?;
    let stored = decode_preflight_state_commit(&stored_preflight)
        .map_err(|error| anyhow::anyhow!("PUBLIC_CHECKPOINT_STORED_DECODE: {error:?}"))?;
    ensure!(
        stored == *expected,
        "PUBLIC_CHECKPOINT_COMPLETE_COMMIT_MISMATCH"
    );
    Ok(id)
}
