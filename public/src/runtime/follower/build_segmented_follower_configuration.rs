// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::segmented::required_segmented_metadata_reservation;
use crate::sync::applied::{AppliedConfig, SegmentedAppliedConfig};
use anyhow::{Context, Result};
use eve_node_policy::{
    SegmentedRecoveryBounds, available_segmented_allowance, development_public_budget,
    development_segmented_recovery_policy,
};
use eve_state::development_state_budget;
use eve_storage::records::{
    OpaqueRecordIdentity, development_opaque_record_budget,
    segmented::{
        SEGMENTED_MAX_CHUNK_BYTES, SEGMENTED_MAX_MARKER_BYTES,
        SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES, SEGMENTED_MAX_REFERENCES, SegmentedCodecLimits,
    },
};
use std::path::PathBuf;

pub(super) fn build_segmented_follower_configuration(
    path: PathBuf,
    identity: OpaqueRecordIdentity,
) -> Result<SegmentedAppliedConfig> {
    let base = development_public_budget();
    let mut repository = development_opaque_record_budget();
    repository.maximum_open_files = base.open_files as i32;
    repository.maximum_batch_records = 1;
    let codec = SegmentedCodecLimits {
        maximum_logical_bytes: eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES,
        maximum_chunk_bytes: SEGMENTED_MAX_CHUNK_BYTES,
        maximum_segments: SEGMENTED_MAX_REFERENCES,
        maximum_payload_bytes: SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES,
    };
    let required_scratch = codec
        .maximum_payload_bytes
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(88 + repository.maximum_batch_bytes))
        .and_then(|bytes| bytes.checked_add(repository.maximum_read_bytes.checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(4096))
        .context("follower scratch accounting overflow")?;
    let bounds = SegmentedRecoveryBounds {
        maximum_logical_bytes: codec.maximum_logical_bytes as u64,
        maximum_segment_bytes: codec.maximum_payload_bytes as u64,
        segment_header_bytes: 177,
        segment_footer_bytes: 32,
        maximum_segments: codec.maximum_segments as u64,
        maximum_marker_bytes: SEGMENTED_MAX_MARKER_BYTES as u64,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: repository.maximum_record_bytes as u64,
        maximum_opaque_read_bytes: repository.maximum_read_bytes as u64,
        maximum_opaque_batch_bytes: repository.maximum_batch_bytes as u64,
        maximum_opaque_batch_records: 1,
        required_metadata_bytes: required_segmented_metadata_reservation()
            .map_err(|error| anyhow::anyhow!("follower metadata: {error:?}"))?
            as u64,
        required_scratch_bytes: required_scratch as u64,
        metadata_limit_bytes: 1_048_576,
        scratch_limit_bytes: 40 * 1_048_576,
        available_auxiliary_bytes: available_segmented_allowance(base)
            .map_err(|error| anyhow::anyhow!("follower budget: {error:?}"))?,
    };
    let policy = development_segmented_recovery_policy(base, bounds)
        .map_err(|error| anyhow::anyhow!("follower policy: {error:?}"))?;
    Ok(SegmentedAppliedConfig {
        application: AppliedConfig {
            path,
            identity,
            public_budget: base,
            state_budget: development_state_budget(),
            repository_budget: repository,
            worker_scratch_limit: policy.maximum_scratch_bytes,
            maximum_recovery_payload_bytes: eve_finality_verifier::MAXIMUM_IMPORT_WIRE_BYTES,
        },
        policy,
        codec,
    })
}
