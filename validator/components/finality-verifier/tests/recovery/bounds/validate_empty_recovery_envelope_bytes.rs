// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    codec_mutation::replace_field,
    recovery_support::{envelope, recovery_chain, signed_transaction},
};
use eve_finality_verifier::{
    RecoveryError, decode_compact_recovery_envelope, encode_compact_recovery_envelope,
    validate_empty_recovery_envelope_bytes,
};
use eve_state::development_state_budget;

#[test]
fn canonical_empty_execution_and_lookahead_pass_borrowed_admission() {
    let budget = development_state_budget();
    let source = envelope(&recovery_chain(), 2);
    assert!(source.execution.transactions.is_empty());
    assert!(source.lookahead.transactions.is_empty());
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    validate_empty_recovery_envelope_bytes(&bytes).unwrap();
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget).unwrap(),
        source
    );
}

#[test]
fn nonempty_execution_exceeds_service_budget_without_restricting_canonical_decode() {
    let budget = development_state_budget();
    let source = envelope(&recovery_chain(), 1);
    assert!(!source.execution.transactions.is_empty());
    assert!(source.lookahead.transactions.is_empty());
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    assert_eq!(
        validate_empty_recovery_envelope_bytes(&bytes),
        Err(RecoveryError::BudgetExceeded)
    );
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget).unwrap(),
        source
    );
}

#[test]
fn nonempty_lookahead_exceeds_service_budget_without_restricting_canonical_decode() {
    let budget = development_state_budget();
    let mut source = envelope(&recovery_chain(), 2);
    source.lookahead.transactions.push(signed_transaction());
    assert!(source.execution.transactions.is_empty());
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    assert_eq!(
        validate_empty_recovery_envelope_bytes(&bytes),
        Err(RecoveryError::BudgetExceeded)
    );
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget).unwrap(),
        source
    );
}

#[test]
fn either_nonempty_list_rejects_before_embedded_state_decoding() {
    let budget = development_state_budget();
    let chain = recovery_chain();
    for execution_nonempty in [false, true] {
        let mut source = envelope(&chain, if execution_nonempty { 1 } else { 2 });
        if !execution_nonempty {
            source.lookahead.transactions.push(signed_transaction());
        }
        let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
        for state_field in [0, 1] {
            let altered = replace_field(&bytes, 15, state_field, |_| vec![0]);
            assert_eq!(
                validate_empty_recovery_envelope_bytes(&altered),
                Err(RecoveryError::BudgetExceeded)
            );
            assert!(matches!(
                decode_compact_recovery_envelope(&altered, &budget),
                Err(RecoveryError::State(_))
            ));
        }
    }
}

#[test]
fn truncated_wrong_schema_and_trailing_empty_envelope_bytes_reject() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 2), &budget).unwrap();
    for cut in 0..bytes.len() {
        assert!(
            validate_empty_recovery_envelope_bytes(&bytes[..cut]).is_err(),
            "cut={cut}"
        );
    }
    let mut wrong_schema = bytes.clone();
    wrong_schema[14] = b'2';
    assert_eq!(
        validate_empty_recovery_envelope_bytes(&wrong_schema),
        Err(RecoveryError::MalformedEncoding)
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        validate_empty_recovery_envelope_bytes(&trailing),
        Err(RecoveryError::NonCanonicalEncoding)
    );
}

#[test]
fn empty_service_preflight_preserves_nested_native_canonical_scans() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 2), &budget).unwrap();
    for field in [3, 4] {
        let mutate_header = |frame: &[u8]| {
            replace_field(frame, 0, 1, |header| {
                let mut changed = header.to_vec();
                changed.extend_from_slice(&[0x78, 1]);
                changed
            })
        };
        let altered = replace_field(&bytes, 15, field, |frame| {
            if field == 3 {
                mutate_header(frame)
            } else {
                replace_field(frame, 0, 0, mutate_header)
            }
        });
        assert_eq!(
            validate_empty_recovery_envelope_bytes(&altered),
            Err(RecoveryError::NonCanonicalEncoding)
        );
    }
}

#[test]
fn empty_service_preflight_preserves_execution_and_lookahead_framing_scans() {
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&envelope(&recovery_chain(), 2), &budget).unwrap();
    for field in [2, 4] {
        let altered = replace_field(&bytes, 15, field, |payload| {
            let mut changed = payload.to_vec();
            changed.push(0);
            changed
        });
        assert_eq!(
            validate_empty_recovery_envelope_bytes(&altered),
            Err(RecoveryError::NonCanonicalEncoding)
        );
    }
}
