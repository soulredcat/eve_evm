// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::fixtures::{Fixture, fixture};
use eve_storage::records::segmented::{
    SegmentedCommitMarker, SegmentedLogicalIdentity, SegmentedMarkerMetadata,
    SegmentedRecoveryAnchor, SegmentedRecoveryMode, SegmentedSegment, SegmentedSegmentMetadata,
    encode_segmented_marker, encode_segmented_segment, hash_segmented_chunk, hash_segmented_marker,
};
use eve_storage::records::{
    OpaqueRecordCursor, compare_and_append_opaque_records, opaque_record_cursor,
};

fn identity(fixture: &Fixture) -> SegmentedLogicalIdentity {
    SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: [7; 32],
        parent: fixture.parent,
        target_height: 1,
        total_length: 7,
    }
}

fn append_marker(
    fixture: &mut Fixture,
    identity: SegmentedLogicalIdentity,
    reference: OpaqueRecordCursor,
) -> SegmentedRecoveryAnchor {
    let metadata = SegmentedMarkerMetadata {
        identity,
        target_state_binding: [8; 32],
    };
    let references = vec![reference];
    let marker_hash = hash_segmented_marker(&metadata, &references, &fixture.pool.codec).unwrap();
    let marker = SegmentedCommitMarker {
        metadata,
        references,
        marker_hash,
    };
    let bytes = encode_segmented_marker(&marker, &fixture.pool.codec).unwrap();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    let ack = compare_and_append_opaque_records(&mut fixture.repository, parent, &[bytes]).unwrap();
    SegmentedRecoveryAnchor {
        height: 1,
        cursor: ack.appended,
        state_binding: [8; 32],
    }
}

fn append_segment(fixture: &mut Fixture, identity: SegmentedLogicalIdentity) -> OpaqueRecordCursor {
    let metadata = SegmentedSegmentMetadata {
        identity,
        index: 0,
        count: 1,
        offset: 0,
    };
    let data = b"genuine".to_vec();
    let chunk_hash = hash_segmented_chunk(&metadata, &data, &fixture.pool.codec).unwrap();
    let bytes = encode_segmented_segment(
        &SegmentedSegment {
            metadata,
            data,
            chunk_hash,
        },
        &fixture.pool.codec,
    )
    .unwrap();
    let parent = opaque_record_cursor(&fixture.repository).unwrap();
    compare_and_append_opaque_records(&mut fixture.repository, parent, &[bytes])
        .unwrap()
        .appended
}

#[test]
fn marker_at_first_physical_row_cannot_publish_a_nonexistent_future_segment_reference() {
    let mut fixture = fixture();
    let identity = identity(&fixture);
    let parent = append_marker(
        &mut fixture,
        identity,
        OpaqueRecordCursor {
            sequence: 100,
            content_hash: [9; 32],
        },
    );
    assert_eq!(parent.cursor.sequence, 1);
    assert!(matches!(
        start_segmented_worker(fixture.repository, fixture.pool, parent),
        Err(SegmentedError::WrongCursor)
    ));
}

#[test]
fn canonical_marker_referring_to_real_sequence_with_a_changed_hash_refuses_startup() {
    let mut fixture = fixture();
    let identity = identity(&fixture);
    let mut reference = append_segment(&mut fixture, identity);
    reference.content_hash[0] ^= 1;
    let parent = append_marker(&mut fixture, identity, reference);
    assert_eq!(parent.cursor.sequence, 2);
    assert!(matches!(
        start_segmented_worker(fixture.repository, fixture.pool, parent),
        Err(SegmentedError::WrongCursor)
    ));
}

#[test]
fn marker_with_correct_physical_hash_but_different_real_segment_identity_refuses_startup() {
    let mut fixture = fixture();
    let marker_identity = identity(&fixture);
    let mut segment_identity = marker_identity;
    segment_identity.logical_id = [0x31; 32];
    let reference = append_segment(&mut fixture, segment_identity);
    let parent = append_marker(&mut fixture, marker_identity, reference);
    assert!(matches!(
        start_segmented_worker(fixture.repository, fixture.pool, parent),
        Err(SegmentedError::WrongCursor)
    ));
}
