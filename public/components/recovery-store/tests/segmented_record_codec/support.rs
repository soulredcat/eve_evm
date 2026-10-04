// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::records::{
    OpaqueRecordCursor,
    segmented::{
        SegmentedCodecLimits, SegmentedCommitMarker, SegmentedLogicalIdentity,
        SegmentedMarkerMetadata, SegmentedRecoveryAnchor, SegmentedRecoveryMode, SegmentedSegment,
        SegmentedSegmentMetadata, hash_segmented_chunk, hash_segmented_marker,
    },
};

pub fn limits() -> SegmentedCodecLimits {
    SegmentedCodecLimits {
        maximum_logical_bytes: 384,
        maximum_chunk_bytes: 64,
        maximum_segments: 6,
        maximum_payload_bytes: 465,
    }
}

pub fn identity(total: u64) -> SegmentedLogicalIdentity {
    SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: [0x31; 32],
        parent: SegmentedRecoveryAnchor {
            height: 1,
            cursor: OpaqueRecordCursor {
                sequence: 4,
                content_hash: [0x41; 32],
            },
            state_binding: [0x51; 32],
        },
        target_height: 2,
        total_length: total,
    }
}

pub fn segment(index: u32, total: u64) -> SegmentedSegment {
    let bounds = limits();
    let offset = u64::from(index) * bounds.maximum_chunk_bytes as u64;
    let length = usize::try_from((total - offset).min(bounds.maximum_chunk_bytes as u64)).unwrap();
    let metadata = SegmentedSegmentMetadata {
        identity: identity(total),
        index,
        count: u32::try_from(total.div_ceil(bounds.maximum_chunk_bytes as u64)).unwrap(),
        offset,
    };
    let data = vec![0x61; length];
    let chunk_hash = hash_segmented_chunk(&metadata, &data, &bounds).unwrap();
    SegmentedSegment {
        metadata,
        data,
        chunk_hash,
    }
}

pub fn marker(total: u64) -> SegmentedCommitMarker {
    let bounds = limits();
    let metadata = SegmentedMarkerMetadata {
        identity: identity(total),
        target_state_binding: [0x71; 32],
    };
    let count = total.div_ceil(bounds.maximum_chunk_bytes as u64);
    let references = (0..count)
        .map(|index| OpaqueRecordCursor {
            sequence: 5 + index,
            content_hash: [u8::try_from(index + 1).unwrap(); 32],
        })
        .collect::<Vec<_>>();
    let marker_hash = hash_segmented_marker(&metadata, &references, &bounds).unwrap();
    SegmentedCommitMarker {
        metadata,
        references,
        marker_hash,
    }
}
