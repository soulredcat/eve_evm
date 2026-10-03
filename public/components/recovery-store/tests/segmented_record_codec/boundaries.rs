// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{identity, limits, marker, segment};
use eve_storage::records::segmented::{
    SEGMENTED_MAX_CHUNK_BYTES, SEGMENTED_MAX_MARKER_BYTES, SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES,
    SegmentedCodecError, SegmentedCodecLimits, SegmentedSegment, SegmentedSegmentMetadata,
    SegmentedSegmentView, encode_segmented_marker, encode_segmented_segment, hash_segmented_chunk,
    hash_segmented_marker, preflight_segmented_record, validate_segmented_codec_limits,
    write_segmented_marker, write_segmented_segment,
};

#[test]
fn full_four_mib_segment_target_is_below_opaque_cap_and_two_targets_fit_part_limit() {
    let bounds = SegmentedCodecLimits {
        maximum_logical_bytes: 21_025_569,
        maximum_chunk_bytes: SEGMENTED_MAX_CHUNK_BYTES,
        maximum_segments: 6,
        maximum_payload_bytes: SEGMENTED_MAX_PHYSICAL_PAYLOAD_BYTES,
    };
    let metadata = SegmentedSegmentMetadata {
        identity: identity(21_025_569),
        index: 0,
        count: 6,
        offset: 0,
    };
    let data = vec![0x11; SEGMENTED_MAX_CHUNK_BYTES];
    let chunk_hash = hash_segmented_chunk(&metadata, &data, &bounds).unwrap();
    let source = SegmentedSegment {
        metadata,
        data,
        chunk_hash,
    };
    let bytes = encode_segmented_segment(&source, &bounds).unwrap();
    assert_eq!(bytes.len(), 4_194_304);
    assert!(bytes.len() + 88 <= 4_198_400);
    assert_eq!(2 * bytes.len(), 8 * 1_048_576);
    preflight_segmented_record(&bytes, &bounds).unwrap();
    let mut over = bytes.clone();
    over.push(0);
    assert_eq!(
        preflight_segmented_record(&over, &bounds).unwrap_err(),
        SegmentedCodecError::LimitExceeded
    );
}

#[test]
fn six_reference_marker_has_exact_465_byte_bound_and_seventh_reference_is_refused() {
    let bounds = limits();
    let source = marker(384);
    let bytes = encode_segmented_marker(&source, &bounds).unwrap();
    assert_eq!(bytes.len(), SEGMENTED_MAX_MARKER_BYTES);
    preflight_segmented_record(&bytes, &bounds).unwrap();
    let mut changed = source;
    changed
        .references
        .push(eve_storage::records::OpaqueRecordCursor {
            sequence: 11,
            content_hash: [0x22; 32],
        });
    assert!(encode_segmented_marker(&changed, &bounds).is_err());
}

#[test]
fn local_limits_refuse_zero_oversized_or_inconsistent_capacity_before_allocation() {
    for field in 0..6 {
        let mut bounds = limits();
        match field {
            0 => bounds.maximum_logical_bytes = 0,
            1 => bounds.maximum_chunk_bytes = 0,
            2 => bounds.maximum_segments = 7,
            3 => bounds.maximum_payload_bytes = 4_194_305,
            4 => bounds.maximum_logical_bytes = 385,
            5 => bounds.maximum_payload_bytes = 200,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_segmented_codec_limits(&bounds),
            Err(SegmentedCodecError::InvalidLimits)
        );
    }
}

#[test]
fn direct_writers_reject_wrong_capacity_nonempty_output_and_bad_hash_without_mutation() {
    let bounds = limits();
    let source = segment(0, 123);
    let view = SegmentedSegmentView {
        metadata: source.metadata,
        data: &source.data,
        chunk_hash: source.chunk_hash,
    };
    let size = encode_segmented_segment(&source, &bounds).unwrap().len();
    let mut short = Vec::with_capacity(size - 1);
    assert_eq!(
        write_segmented_segment(&view, &mut short, &bounds),
        Err(SegmentedCodecError::OutputCapacity)
    );
    assert!(short.is_empty());
    let mut occupied = Vec::with_capacity(size);
    occupied.push(0x11);
    assert_eq!(
        write_segmented_segment(&view, &mut occupied, &bounds),
        Err(SegmentedCodecError::OutputCapacity)
    );
    assert_eq!(occupied, vec![0x11]);
    let mut invalid = view;
    invalid.chunk_hash[0] ^= 1;
    let mut exact = Vec::with_capacity(size);
    assert_eq!(
        write_segmented_segment(&invalid, &mut exact, &bounds),
        Err(SegmentedCodecError::HashMismatch)
    );
    assert!(exact.is_empty());
    let mut source = marker(123);
    source.marker_hash[0] ^= 1;
    let mut output = Vec::with_capacity(305);
    assert_eq!(
        write_segmented_marker(&source, &mut output, &bounds),
        Err(SegmentedCodecError::HashMismatch)
    );
    assert!(output.is_empty());
}

#[test]
fn references_may_start_after_known_orphans_but_never_at_or_before_the_parent_cursor() {
    let bounds = limits();
    let mut source = marker(123);
    for reference in &mut source.references {
        reference.sequence += 5;
    }
    source.marker_hash =
        hash_segmented_marker(&source.metadata, &source.references, &bounds).unwrap();
    let bytes = encode_segmented_marker(&source, &bounds).unwrap();
    preflight_segmented_record(&bytes, &bounds).unwrap();
    source.references[0].sequence = source.metadata.identity.parent.cursor.sequence;
    assert!(hash_segmented_marker(&source.metadata, &source.references, &bounds).is_err());
}
