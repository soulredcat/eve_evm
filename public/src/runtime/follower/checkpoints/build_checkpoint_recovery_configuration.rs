// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{CHECKPOINT_CHUNK_BYTES, CHECKPOINT_MAX_HEIGHT};
use crate::sync::applied::checkpoints::{AppliedCheckpointLimits, CheckpointRecoveryConfig};
use eve_storage::checkpoints::messages::{
    MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES, MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES,
};
use std::path::Path;

pub(in crate::runtime::follower) fn build_checkpoint_recovery_configuration(
    data: &Path,
) -> CheckpointRecoveryConfig {
    let logical = eve_state::development_state_budget();
    CheckpointRecoveryConfig {
        content_root: data.join(".checkpoints").join("content"),
        proof_root: data.join(".checkpoints").join("proofs"),
        maximum_scan_records: 100_000,
        limits: AppliedCheckpointLimits {
            content: eve_storage::checkpoints::CheckpointLimits {
                logical,
                maximum_body_bytes: logical
                    .maximum_commit_bytes
                    .min(MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES),
                maximum_chunk_bytes: CHECKPOINT_CHUNK_BYTES,
                maximum_chunks: 4_096,
                maximum_manifest_bytes: MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES,
            },
            proofs: eve_storage::checkpoints::proofs::CheckpointProofLimits {
                maximum_witness_bytes: eve_finality_verifier::MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
                maximum_total_bytes: 128 * 1_048_576,
                maximum_files: CHECKPOINT_MAX_HEIGHT as usize + 1,
                maximum_manifest_bytes: 1_048_576,
                maximum_disk_bytes: 132 * 1_048_576,
            },
            verification: eve_finality_verifier::CheckpointLimits {
                maximum_height_gap: CHECKPOINT_MAX_HEIGHT,
                maximum_witness_bytes: eve_finality_verifier::MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
            },
        },
    }
}
