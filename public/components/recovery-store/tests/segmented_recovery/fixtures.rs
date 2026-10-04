// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity, OpaqueRecordRepository,
    compare_and_append_opaque_records, development_opaque_record_budget,
    opaque_record_bootstrap_cursor, opaque_record_budget, opaque_record_cursor,
    open_opaque_record_repository,
    segmented::{
        SegmentedCodecLimits, SegmentedCommitMarker, SegmentedLogicalIdentity,
        SegmentedMarkerMetadata, SegmentedRecoveryAnchor, SegmentedRecoveryMode, SegmentedSegment,
        SegmentedSegmentMetadata, encode_segmented_marker, encode_segmented_segment,
        hash_segmented_chunk, hash_segmented_logical_body, hash_segmented_marker,
        recovery::required_segmented_recovery_reservation,
    },
};

pub fn namespace() -> OpaqueRecordIdentity {
    // Inert structural storage fixture, without execution or certificate authority.
    OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    }
}

pub fn budget() -> OpaqueRecordBudget {
    OpaqueRecordBudget {
        maximum_record_bytes: 553,
        maximum_read_bytes: 553,
        maximum_batch_bytes: 4_096,
        maximum_batch_records: 1,
        maximum_retained_records: 128,
        block_cache_bytes: 65_536,
        write_buffer_bytes: 65_536,
        ..development_opaque_record_budget()
    }
}

pub fn limits() -> SegmentedCodecLimits {
    SegmentedCodecLimits {
        maximum_logical_bytes: 384,
        maximum_chunk_bytes: 64,
        maximum_segments: 6,
        maximum_payload_bytes: 465,
    }
}

pub fn open(path: &std::path::Path) -> OpaqueRecordRepository {
    open_opaque_record_repository(path, namespace(), budget()).unwrap()
}

pub fn anchor(repository: &OpaqueRecordRepository) -> SegmentedRecoveryAnchor {
    SegmentedRecoveryAnchor {
        height: 0,
        cursor: opaque_record_bootstrap_cursor(repository),
        state_binding: [7; 32],
    }
}

pub fn charge(repository: &OpaqueRecordRepository) -> usize {
    required_segmented_recovery_reservation(&opaque_record_budget(repository), &limits()).unwrap()
}

pub fn identity(parent: SegmentedRecoveryAnchor, body: &[u8]) -> SegmentedLogicalIdentity {
    SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(body),
        parent,
        target_height: parent.height + 1,
        total_length: body.len() as u64,
    }
}

pub fn segment(identity: SegmentedLogicalIdentity, body: &[u8], index: usize) -> Vec<u8> {
    let limits = limits();
    let offset = index * limits.maximum_chunk_bytes;
    let data = body[offset..body.len().min(offset + limits.maximum_chunk_bytes)].to_vec();
    let metadata = SegmentedSegmentMetadata {
        identity,
        index: index as u32,
        count: body.len().div_ceil(limits.maximum_chunk_bytes) as u32,
        offset: offset as u64,
    };
    let chunk_hash = hash_segmented_chunk(&metadata, &data, &limits).unwrap();
    encode_segmented_segment(
        &SegmentedSegment {
            metadata,
            data,
            chunk_hash,
        },
        &limits,
    )
    .unwrap()
}

pub fn append(repository: &mut OpaqueRecordRepository, payload: Vec<u8>) -> OpaqueRecordCursor {
    let head = opaque_record_cursor(repository).unwrap();
    compare_and_append_opaque_records(repository, head, &[payload])
        .unwrap()
        .appended
}

pub fn append_segments(
    repository: &mut OpaqueRecordRepository,
    identity: SegmentedLogicalIdentity,
    body: &[u8],
    indices: &[usize],
) -> Vec<OpaqueRecordCursor> {
    indices
        .iter()
        .map(|index| append(repository, segment(identity, body, *index)))
        .collect()
}

pub fn marker(
    identity: SegmentedLogicalIdentity,
    references: Vec<OpaqueRecordCursor>,
    binding: [u8; 32],
) -> Vec<u8> {
    let metadata = SegmentedMarkerMetadata {
        identity,
        target_state_binding: binding,
    };
    let marker_hash = hash_segmented_marker(&metadata, &references, &limits()).unwrap();
    encode_segmented_marker(
        &SegmentedCommitMarker {
            metadata,
            references,
            marker_hash,
        },
        &limits(),
    )
    .unwrap()
}

pub fn append_bundle(
    repository: &mut OpaqueRecordRepository,
    parent: SegmentedRecoveryAnchor,
    body: &[u8],
    binding: [u8; 32],
) -> SegmentedRecoveryAnchor {
    let identity = identity(parent, body);
    let count = body.len().div_ceil(limits().maximum_chunk_bytes);
    let references = append_segments(repository, identity, body, &(0..count).collect::<Vec<_>>());
    let cursor = append(repository, marker(identity, references, binding));
    SegmentedRecoveryAnchor {
        height: identity.target_height,
        cursor,
        state_binding: binding,
    }
}
