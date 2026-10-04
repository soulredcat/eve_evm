// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{limits, marker, segment};
use eve_storage::records::segmented::{
    SEGMENTED_MARKER_OVERHEAD_BYTES, SEGMENTED_SEGMENT_HEADER_BYTES,
    SEGMENTED_SEGMENT_OVERHEAD_BYTES, SegmentedRecord, SegmentedSegmentView,
    decode_segmented_record, encode_segmented_marker, encode_segmented_segment,
    match_segmented_identity, preflight_segmented_record, segmented_marker_reference_at,
    segmented_marker_view, segmented_record_bytes, segmented_record_limits, segmented_record_stats,
    segmented_segment_view, write_segmented_marker, write_segmented_segment,
};
use sha2::{Digest, Sha256};

#[test]
fn canonical_segment_roundtrip_borrows_exact_data_and_binds_all_metadata() {
    let source = segment(1, 123);
    let bounds = limits();
    let bytes = encode_segmented_segment(&source, &bounds).unwrap();
    assert_eq!(
        bytes.len(),
        SEGMENTED_SEGMENT_OVERHEAD_BYTES + source.data.len()
    );
    assert_eq!(&bytes[..25], b"EVE_SEGMENTED_RECOVERY_V1");
    assert_eq!(bytes[25], 1);
    assert_eq!(&bytes[26..28], &[0, 1]);
    assert_eq!(bytes[28], 1);
    let expected: [u8; 32] = Sha256::digest(&bytes[..bytes.len() - 32]).into();
    assert_eq!(expected, source.chunk_hash);
    let preflight = preflight_segmented_record(&bytes, &bounds).unwrap();
    assert_eq!(segmented_record_bytes(&preflight).as_ptr(), bytes.as_ptr());
    let view = segmented_segment_view(&preflight).unwrap();
    assert_eq!(
        view.data.as_ptr(),
        bytes[SEGMENTED_SEGMENT_HEADER_BYTES..].as_ptr()
    );
    assert_eq!(view.metadata, source.metadata);
    assert_eq!(view.data, source.data);
    assert!(segmented_marker_view(&preflight).is_none());
    assert_eq!(segmented_record_stats(&preflight).data_bytes, 59);
    match_segmented_identity(&preflight, &source.metadata.identity).unwrap();
    assert_eq!(
        decode_segmented_record(&preflight).unwrap(),
        SegmentedRecord::Segment(source.clone())
    );
    let mut output = Vec::with_capacity(bytes.len());
    write_segmented_segment(
        &SegmentedSegmentView {
            metadata: source.metadata,
            data: &source.data,
            chunk_hash: source.chunk_hash,
        },
        &mut output,
        &bounds,
    )
    .unwrap();
    assert_eq!(output, bytes);
}

#[test]
fn canonical_marker_roundtrip_exposes_exact_references_without_finality_authority() {
    let source = marker(123);
    let bounds = limits();
    let bytes = encode_segmented_marker(&source, &bounds).unwrap();
    assert_eq!(
        bytes.len(),
        SEGMENTED_MARKER_OVERHEAD_BYTES + source.references.len() * 40
    );
    let expected: [u8; 32] = Sha256::digest(&bytes[..bytes.len() - 32]).into();
    assert_eq!(expected, source.marker_hash);
    let preflight = preflight_segmented_record(&bytes, &bounds).unwrap();
    let view = segmented_marker_view(&preflight).unwrap();
    assert_eq!(view.reference_count, 2);
    assert_eq!(view.metadata, source.metadata);
    for (index, reference) in source.references.iter().enumerate() {
        assert_eq!(
            &segmented_marker_reference_at(&view, index).unwrap(),
            reference
        );
    }
    assert!(segmented_marker_reference_at(&view, 2).is_err());
    assert!(segmented_segment_view(&preflight).is_none());
    let stats = segmented_record_stats(&preflight);
    assert_eq!(stats.reference_count, 2);
    assert_eq!(
        stats.reference_allocation_bytes,
        2 * std::mem::size_of_val(&source.references[0])
    );
    assert_eq!(
        decode_segmented_record(&preflight).unwrap(),
        SegmentedRecord::CommitMarker(source.clone())
    );
    let mut output = Vec::with_capacity(bytes.len());
    write_segmented_marker(&source, &mut output, &bounds).unwrap();
    assert_eq!(output, bytes);
}

#[test]
fn frozen_limits_and_detached_views_cannot_change_the_preflight_record() {
    let source = segment(0, 123);
    let mut bounds = limits();
    let bytes = encode_segmented_segment(&source, &bounds).unwrap();
    let original = bounds;
    let preflight = preflight_segmented_record(&bytes, &bounds).unwrap();
    bounds.maximum_logical_bytes = 1;
    let mut view = segmented_segment_view(&preflight).unwrap();
    view.metadata.identity.logical_id = [0x91; 32];
    assert_ne!(
        view.metadata,
        segmented_segment_view(&preflight).unwrap().metadata
    );
    assert_eq!(segmented_record_limits(&preflight), original);
    assert_ne!(segmented_record_limits(&preflight), bounds);
    assert_eq!(
        decode_segmented_record(&preflight).unwrap(),
        SegmentedRecord::Segment(source)
    );
}
