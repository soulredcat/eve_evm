// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    codec_mutation::replace_field,
    recovery_support::{envelope, recovery_chain},
};
use eve_finality_verifier::{
    RecoveryError, decode_compact_recovery_envelope, encode_compact_recovery_envelope,
};
use eve_state::{StateError, development_state_budget};
use prost::Message;

#[test]
fn truncated_wrong_schema_and_trailing_bytes_reject() {
    let budget = development_state_budget();
    let source = envelope(&recovery_chain(), 1);
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    for cut in 0..bytes.len() {
        assert!(
            decode_compact_recovery_envelope(&bytes[..cut], &budget).is_err(),
            "cut={cut}"
        );
    }
    let mut wrong_schema = bytes.clone();
    wrong_schema[14] = b'2';
    assert_eq!(
        decode_compact_recovery_envelope(&wrong_schema, &budget),
        Err(RecoveryError::MalformedEncoding)
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        decode_compact_recovery_envelope(&trailing, &budget),
        Err(RecoveryError::NonCanonicalEncoding)
    );
}

#[test]
fn unknown_duplicate_and_nonminimal_native_fields_reject() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 1), &budget).unwrap();
    for duplicate in [false, true] {
        let altered = replace_field(&bytes, 15, 3, |frame| {
            replace_field(frame, 0, 1, |header| {
                if duplicate {
                    let offset = header
                        .windows(2)
                        .position(|bytes| bytes == [0x18, 1])
                        .unwrap();
                    let mut changed = header[..offset].to_vec();
                    changed.extend_from_slice(&[0x18, 1]);
                    changed.extend_from_slice(&header[offset..]);
                    return changed;
                }
                let mut changed = header.to_vec();
                changed.extend_from_slice(&[0x78, 1]);
                changed
            })
        });
        assert_eq!(
            decode_compact_recovery_envelope(&altered, &budget),
            Err(RecoveryError::NonCanonicalEncoding)
        );
    }
    let altered = replace_field(&bytes, 15, 3, |frame| {
        replace_field(frame, 0, 1, |header| {
            let offset = header
                .windows(2)
                .position(|bytes| bytes == [0x18, 1])
                .unwrap();
            let mut changed = header[..offset].to_vec();
            changed.extend_from_slice(&[0x18, 0x81, 0]);
            changed.extend_from_slice(&header[offset + 2..]);
            changed
        })
    });
    assert_eq!(
        decode_compact_recovery_envelope(&altered, &budget),
        Err(RecoveryError::NonCanonicalEncoding)
    );
}

#[test]
fn embedded_state_version_and_execution_trailing_bytes_reject() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 1), &budget).unwrap();
    for field in [0, 2] {
        let altered = replace_field(&bytes, 15, field, |original| {
            let mut changed = original.to_vec();
            changed.push(0);
            changed
        });
        let expected = if field == 0 {
            RecoveryError::State(StateError::NonCanonicalEncoding)
        } else {
            RecoveryError::NonCanonicalEncoding
        };
        assert_eq!(
            decode_compact_recovery_envelope(&altered, &budget),
            Err(expected)
        );
    }
}

#[test]
fn nested_native_signature_bounds_reject_before_state_decoding() {
    let budget = development_state_budget();
    let source = envelope(&recovery_chain(), 1);
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    let malformed_parent = replace_field(&bytes, 15, 0, |_| vec![0]);
    let mut oversized = source.finalized.commit;
    oversized.signatures[0].signature = vec![0; 65];
    let altered = replace_field(&malformed_parent, 15, 3, |frame| {
        replace_field(frame, 0, 2, |_| oversized.encode_to_vec())
    });
    assert_eq!(
        decode_compact_recovery_envelope(&altered, &budget),
        Err(RecoveryError::BudgetExceeded)
    );
}

#[test]
fn declared_transaction_count_rejects_before_state_decoding_or_vector_allocation() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 1), &budget).unwrap();
    let malformed_parent = replace_field(&bytes, 15, 0, |_| vec![0]);
    let altered = replace_field(&malformed_parent, 15, 4, |lookahead| {
        let frame_size = u32::from_be_bytes(lookahead[..4].try_into().unwrap()) as usize;
        let mut changed = lookahead[..4 + frame_size].to_vec();
        changed.extend_from_slice(&1_429_u32.to_be_bytes());
        changed
    });
    assert_eq!(
        decode_compact_recovery_envelope(&altered, &budget),
        Err(RecoveryError::BudgetExceeded)
    );
}
