// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures;
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_budget,
    segmented::{
        SegmentedCodecLimits, SegmentedCommitMarker, SegmentedLogicalIdentity,
        SegmentedMarkerMetadata, SegmentedRecoveryAnchor, SegmentedRecoveryMode, SegmentedSegment,
        SegmentedSegmentMetadata, encode_segmented_marker, encode_segmented_segment,
        hash_segmented_chunk, hash_segmented_logical_body, hash_segmented_marker,
        recovery::required_segmented_checkpoint_recovery_reservation,
    },
};

pub fn codec() -> SegmentedCodecLimits {
    SegmentedCodecLimits {
        maximum_logical_bytes: 128,
        maximum_chunk_bytes: 64,
        maximum_segments: 2,
        maximum_payload_bytes: 465,
    }
}

pub fn charge(repository: &OpaqueRecordRepository) -> usize {
    required_segmented_checkpoint_recovery_reservation(&opaque_record_budget(repository), &codec())
        .unwrap()
}

pub fn suffix(
    repository: &mut OpaqueRecordRepository,
    parent: SegmentedRecoveryAnchor,
    body: &[u8],
) -> SegmentedRecoveryAnchor {
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(body),
        parent,
        target_height: parent.height + 1,
        total_length: body.len() as u64,
    };
    let mut references = Vec::new();
    for (index, chunk) in body.chunks(64).enumerate() {
        let metadata = SegmentedSegmentMetadata {
            identity,
            index: index as u32,
            count: body.len().div_ceil(64) as u32,
            offset: (index * 64) as u64,
        };
        let segment = SegmentedSegment {
            metadata,
            data: chunk.to_vec(),
            chunk_hash: hash_segmented_chunk(&metadata, chunk, &codec()).unwrap(),
        };
        references.push(fixtures::append(
            repository,
            encode_segmented_segment(&segment, &codec()).unwrap(),
        ));
    }
    let metadata = SegmentedMarkerMetadata {
        identity,
        target_state_binding: [11; 32],
    };
    let marker_hash = hash_segmented_marker(&metadata, &references, &codec()).unwrap();
    let marker = SegmentedCommitMarker {
        metadata,
        references,
        marker_hash,
    };
    let cursor = fixtures::append(
        repository,
        encode_segmented_marker(&marker, &codec()).unwrap(),
    );
    SegmentedRecoveryAnchor {
        height: parent.height + 1,
        cursor,
        state_binding: [11; 32],
    }
}
