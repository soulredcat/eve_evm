// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, SegmentedPartPool,
    types::{BATCHES, PARTS, PartAccounting},
    worker::required_segment_scratch,
};
use super::estimate_segmented_metadata;
use eve_node_policy::{
    PublicBudget, SegmentedRecoveryBounds, SegmentedRecoveryPolicy, available_segmented_allowance,
    validate_segmented_recovery_policy,
};
use eve_storage::records::segmented::{
    SEGMENTED_MAX_MARKER_BYTES, SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES, SEGMENTED_MAX_REFERENCES,
    SEGMENTED_SEGMENT_HEADER_BYTES, SegmentedCodecLimits,
};
use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordIdentity, validate_opaque_record_budget,
};
use std::sync::{Arc, Mutex, atomic::AtomicBool};

pub fn create_segmented_part_pool(
    base: PublicBudget,
    policy: SegmentedRecoveryPolicy,
    codec: SegmentedCodecLimits,
    repository: OpaqueRecordBudget,
    namespace: OpaqueRecordIdentity,
) -> Result<Arc<SegmentedPartPool>, SegmentedError> {
    validate_opaque_record_budget(&repository).map_err(|_| SegmentedError::InvalidConfiguration)?;
    if policy.retained_parts > PARTS as u64
        || policy.maximum_segment_bytes != codec.maximum_payload_bytes as u64
        || codec.maximum_segments > SEGMENTED_MAX_REFERENCES
        || codec.maximum_payload_bytes > SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES
        || namespace.genesis_hash == [0; 32]
        || namespace.owner == [0; 32]
        || namespace.domain == [0; 32]
        || repository.block_cache_bytes as u64 > base.block_cache_bytes
        || repository.write_buffer_bytes as u64 > base.write_buffer_bytes
        || repository.write_buffer_count as u64 > base.write_buffer_count
        || repository.maximum_background_jobs as u64 > base.background_jobs
        || repository.maximum_open_files as u64 > base.open_files
    {
        return Err(SegmentedError::InvalidConfiguration);
    }
    let (pool, batch, worker) = estimate_segmented_metadata();
    let metadata = pool
        .checked_add(batch.checked_mul(BATCHES).ok_or(SegmentedError::Overflow)?)
        .and_then(|bytes| bytes.checked_add(worker))
        .ok_or(SegmentedError::Overflow)?;
    let bounds = SegmentedRecoveryBounds {
        maximum_logical_bytes: codec.maximum_logical_bytes as u64,
        maximum_segment_bytes: codec.maximum_payload_bytes as u64,
        segment_header_bytes: SEGMENTED_SEGMENT_HEADER_BYTES as u64,
        segment_footer_bytes: 32,
        maximum_segments: codec.maximum_segments as u64,
        maximum_marker_bytes: SEGMENTED_MAX_MARKER_BYTES as u64,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: repository.maximum_record_bytes as u64,
        maximum_opaque_read_bytes: repository.maximum_read_bytes as u64,
        maximum_opaque_batch_bytes: repository.maximum_batch_bytes as u64,
        maximum_opaque_batch_records: repository.maximum_batch_records as u64,
        required_metadata_bytes: metadata as u64,
        required_scratch_bytes: required_segment_scratch(codec.maximum_payload_bytes, repository)?,
        metadata_limit_bytes: policy.maximum_metadata_bytes,
        scratch_limit_bytes: policy.maximum_scratch_bytes,
        available_auxiliary_bytes: available_segmented_allowance(base)
            .map_err(|_| SegmentedError::InvalidConfiguration)?,
    };
    validate_segmented_recovery_policy(base, policy, bounds)
        .map_err(|_| SegmentedError::InvalidConfiguration)?;
    if codec.maximum_chunk_bytes.checked_add(209) != Some(codec.maximum_payload_bytes) {
        return Err(SegmentedError::InvalidConfiguration);
    }
    Ok(Arc::new(SegmentedPartPool {
        policy,
        codec,
        repository,
        namespace,
        worker_active: AtomicBool::new(false),
        accounting: Mutex::new(PartAccounting {
            next_id: 0,
            bytes: 0,
            metadata: pool as u64,
            slots: [None; PARTS],
        }),
    }))
}
