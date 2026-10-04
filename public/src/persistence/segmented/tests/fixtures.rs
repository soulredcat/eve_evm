// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    pool::estimate_segmented_metadata, types::BATCHES, worker::required_segment_scratch, *,
};
use eve_node_policy::{
    SegmentedRecoveryBounds, available_segmented_allowance, development_public_budget,
    development_segmented_recovery_policy,
};
use eve_storage::records::segmented::{
    SEGMENTED_MAX_CHUNK_BYTES, SegmentedCodecLimits, SegmentedLogicalIdentity,
    SegmentedRecoveryAnchor, SegmentedRecoveryMode,
};
use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordIdentity, OpaqueRecordRepository,
    development_opaque_record_budget, opaque_record_cursor, open_opaque_record_repository,
};
use std::{path::PathBuf, sync::Arc};

pub(super) struct Fixture {
    pub(super) _directory: tempfile::TempDir,
    pub(super) path: PathBuf,
    pub(super) budget: OpaqueRecordBudget,
    pub(super) namespace: OpaqueRecordIdentity,
    pub(super) pool: Arc<SegmentedPartPool>,
    pub(super) repository: OpaqueRecordRepository,
    pub(super) parent: SegmentedRecoveryAnchor,
}
pub(super) fn fixture() -> Fixture {
    fixture_with_retention(None)
}
pub(super) fn fixture_with_retention(retention: Option<u64>) -> Fixture {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("segmented");
    let namespace = OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    };
    let mut budget = development_opaque_record_budget();
    budget.maximum_open_files = 32;
    budget.maximum_batch_records = 1;
    if let Some(retention) = retention {
        budget.maximum_retained_records = retention;
    }
    let repository = open_opaque_record_repository(&path, namespace, budget).unwrap();
    let parent = SegmentedRecoveryAnchor {
        height: 0,
        cursor: opaque_record_cursor(&repository).unwrap(),
        state_binding: [4; 32],
    };
    let codec = SegmentedCodecLimits {
        maximum_logical_bytes: 21_025_569,
        maximum_chunk_bytes: SEGMENTED_MAX_CHUNK_BYTES,
        maximum_segments: 6,
        maximum_payload_bytes: 4_194_304,
    };
    let base = development_public_budget();
    let (pool_bytes, batch_bytes, worker_bytes) = estimate_segmented_metadata();
    let bounds = SegmentedRecoveryBounds {
        maximum_logical_bytes: codec.maximum_logical_bytes as u64,
        maximum_segment_bytes: codec.maximum_payload_bytes as u64,
        segment_header_bytes: 177,
        segment_footer_bytes: 32,
        maximum_segments: 6,
        maximum_marker_bytes: 465,
        opaque_record_header_bytes: 88,
        maximum_opaque_record_bytes: budget.maximum_record_bytes as u64,
        maximum_opaque_read_bytes: budget.maximum_read_bytes as u64,
        maximum_opaque_batch_bytes: budget.maximum_batch_bytes as u64,
        maximum_opaque_batch_records: 1,
        required_metadata_bytes: (pool_bytes + BATCHES * batch_bytes + worker_bytes) as u64,
        required_scratch_bytes: required_segment_scratch(codec.maximum_payload_bytes, budget)
            .unwrap(),
        metadata_limit_bytes: 1_048_576,
        scratch_limit_bytes: 40 * 1_048_576,
        available_auxiliary_bytes: available_segmented_allowance(base).unwrap(),
    };
    let policy = development_segmented_recovery_policy(base, bounds).unwrap();
    let pool = create_segmented_part_pool(base, policy, codec, budget, namespace).unwrap();
    Fixture {
        _directory: directory,
        path,
        budget,
        namespace,
        pool,
        repository,
        parent,
    }
}
pub(super) fn batch(
    pool: &Arc<SegmentedPartPool>,
    parent: SegmentedRecoveryAnchor,
    expected: eve_storage::records::OpaqueRecordCursor,
    data: &[u8],
) -> SealedSegmentedBatch {
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: eve_storage::records::segmented::hash_segmented_logical_body(data),
        parent,
        target_height: parent.height + 1,
        total_length: data.len() as u64,
    };
    let plan = plan_segmented_batch(pool, identity, [8; 32]).unwrap();
    let reservation = reserve_segmented_batch(pool, &plan).unwrap();
    write_and_seal_segmented_batch(reservation, data, expected).unwrap()
}
