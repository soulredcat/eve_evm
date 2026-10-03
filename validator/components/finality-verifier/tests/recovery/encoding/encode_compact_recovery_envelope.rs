// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery_support::{envelope, recovery_chain};
use eve_finality_verifier::{
    RecoveryError, decode_compact_recovery_envelope, encode_compact_recovery_envelope,
};
use eve_state::{B256, StateError, development_state_budget};

#[test]
fn actual_execution_and_certificates_roundtrip_without_normalizing_auxiliary_identity() {
    let chain = recovery_chain();
    let mut source = envelope(&chain, 1);
    assert_eq!(source.execution.transactions.len(), 1);
    assert_eq!(source.execution.receipts.len(), 1);
    assert_eq!(source.finalized.commit.signatures.len(), 4);
    source.parent.timestamp += 7;
    source.parent.content_digest = B256::repeat_byte(0x9a);
    source.expected.content_digest = B256::repeat_byte(0xb4);
    let budget = development_state_budget();
    let bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    assert!(bytes.starts_with(b"EVE_RECOVERY_V1"));
    let decoded = decode_compact_recovery_envelope(&bytes, &budget).unwrap();
    assert_eq!(decoded, source);
    assert_eq!(
        encode_compact_recovery_envelope(&decoded, &budget).unwrap(),
        bytes
    );
}

#[test]
fn canonical_codec_honors_local_operator_budget() {
    let source = envelope(&recovery_chain(), 1);
    let bytes = encode_compact_recovery_envelope(&source, &development_state_budget()).unwrap();
    let mut budget = development_state_budget();
    budget.maximum_commit_bytes = 1;
    assert_eq!(
        encode_compact_recovery_envelope(&source, &budget),
        Err(RecoveryError::State(StateError::BudgetExceeded)),
    );
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget),
        Err(RecoveryError::State(StateError::BudgetExceeded)),
    );
}

#[test]
fn changing_ordered_native_data_changes_exact_recovery_bytes() {
    let mut source = envelope(&recovery_chain(), 1);
    source.lookahead.transactions = vec![vec![0x11, 0x22].into(), vec![0x33, 0x44].into()];
    let budget = development_state_budget();
    let original = encode_compact_recovery_envelope(&source, &budget).unwrap();
    source.lookahead.transactions.reverse();
    let reordered = encode_compact_recovery_envelope(&source, &budget).unwrap();
    assert_ne!(original, reordered);
    assert_eq!(
        decode_compact_recovery_envelope(&reordered, &budget).unwrap(),
        source
    );
}
