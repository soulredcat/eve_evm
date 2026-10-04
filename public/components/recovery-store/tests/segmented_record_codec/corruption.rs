// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{limits, marker, segment};
use eve_storage::records::segmented::{
    SEGMENTED_MARKER_HEADER_BYTES, SegmentedCodecError, encode_segmented_marker,
    encode_segmented_segment, hash_segmented_chunk, hash_segmented_marker,
    match_segmented_identity, preflight_segmented_record,
};

#[test]
fn every_truncation_trailing_byte_unknown_kind_version_or_mode_is_rejected() {
    let bounds = limits();
    for bytes in [
        encode_segmented_segment(&segment(0, 123), &bounds).unwrap(),
        encode_segmented_marker(&marker(123), &bounds).unwrap(),
    ] {
        for end in 0..bytes.len() {
            assert!(preflight_segmented_record(&bytes[..end], &bounds).is_err());
        }
        let mut changed = bytes.clone();
        changed.push(0);
        assert!(preflight_segmented_record(&changed, &bounds).is_err());
        for offset in [0, 25, 26, 28] {
            let mut changed = bytes.clone();
            changed[offset] = 0xff;
            assert!(preflight_segmented_record(&changed, &bounds).is_err());
        }
    }
}

#[test]
fn chunk_identity_parent_data_and_hash_corruption_is_not_normalized() {
    let bounds = limits();
    let source = segment(0, 123);
    let bytes = encode_segmented_segment(&source, &bounds).unwrap();
    for offset in [29, 77, 109, 177, bytes.len() - 1] {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        assert_eq!(
            preflight_segmented_record(&changed, &bounds).unwrap_err(),
            SegmentedCodecError::HashMismatch
        );
    }
    let mut bad = source;
    bad.chunk_hash[0] ^= 1;
    assert_eq!(
        encode_segmented_segment(&bad, &bounds),
        Err(SegmentedCodecError::HashMismatch)
    );
}

#[test]
fn marker_reference_hash_target_binding_and_reference_order_corruption_rejects() {
    let bounds = limits();
    let source = marker(123);
    let bytes = encode_segmented_marker(&source, &bounds).unwrap();
    for offset in [
        29,
        109,
        149,
        SEGMENTED_MARKER_HEADER_BYTES + 8,
        bytes.len() - 1,
    ] {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        assert_eq!(
            preflight_segmented_record(&changed, &bounds).unwrap_err(),
            SegmentedCodecError::HashMismatch
        );
    }
    let mut reordered = source.clone();
    reordered.references.swap(0, 1);
    assert_eq!(
        hash_segmented_marker(&reordered.metadata, &reordered.references, &bounds),
        Err(SegmentedCodecError::InvalidReferences)
    );
    let mut skipped = source.clone();
    skipped.references[1].sequence += 1;
    assert_eq!(
        hash_segmented_marker(&skipped.metadata, &skipped.references, &bounds),
        Err(SegmentedCodecError::InvalidReferences)
    );
    let mut duplicated = source;
    duplicated.references[1] = duplicated.references[0];
    assert!(encode_segmented_marker(&duplicated, &bounds).is_err());
}

#[test]
fn rehashed_attacker_identity_still_requires_matching_the_callers_local_expected_identity() {
    let bounds = limits();
    let mut source = segment(0, 123);
    let expected = source.metadata.identity;
    source.metadata.identity.logical_id = [0x81; 32];
    source.chunk_hash = hash_segmented_chunk(&source.metadata, &source.data, &bounds).unwrap();
    let bytes = encode_segmented_segment(&source, &bounds).unwrap();
    let preflight = preflight_segmented_record(&bytes, &bounds).unwrap();
    assert_eq!(
        match_segmented_identity(&preflight, &expected),
        Err(SegmentedCodecError::InvalidIdentity)
    );
    // Local codec integrity is neither actual segment membership nor finality.
    match_segmented_identity(&preflight, &source.metadata.identity).unwrap();
}

#[test]
fn invalid_chunk_index_count_offset_length_and_parent_height_fail_before_encoding() {
    let bounds = limits();
    let source = segment(0, 123);
    for field in 0..7 {
        let mut changed = source.clone();
        match field {
            0 => changed.metadata.index = 2,
            1 => changed.metadata.count += 1,
            2 => changed.metadata.offset += 1,
            3 => {
                changed.data.pop();
            }
            4 => changed.metadata.identity.target_height += 1,
            5 => changed.metadata.identity.parent.height = u64::MAX,
            6 => changed.metadata.identity.logical_id = [0; 32],
            _ => unreachable!(),
        }
        assert!(encode_segmented_segment(&changed, &bounds).is_err());
    }
}
