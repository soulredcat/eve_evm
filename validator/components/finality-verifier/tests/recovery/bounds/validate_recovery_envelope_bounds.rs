// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery_support::{envelope, recovery_chain};
use eve_finality_verifier::{
    RecoveryError, decode_compact_recovery_envelope, encode_compact_recovery_envelope,
    validate_recovery_envelope_bounds,
};
use eve_state::{Bytes, development_state_budget};

#[test]
fn exact_opaque_payload_ceiling_passes_and_one_extra_byte_rejects() {
    const PAYLOAD_LIMIT: usize = 4_198_312;
    const TRANSACTION_LIMIT: usize = 131_072;
    let mut source = envelope(&recovery_chain(), 1);
    source.execution.receipts[0] = vec![0; 16_384].into();
    let budget = development_state_budget();
    let baseline = encode_compact_recovery_envelope(&source, &budget)
        .unwrap()
        .len();
    let available = PAYLOAD_LIMIT - baseline;
    let count = available.div_ceil(TRANSACTION_LIMIT + 4);
    let mut remaining = available - count * 4;
    for _ in 0..count {
        let size = remaining.min(TRANSACTION_LIMIT);
        source.lookahead.transactions.push(vec![0; size].into());
        remaining -= size;
    }
    assert_eq!(remaining, 0);
    validate_recovery_envelope_bounds(&source).unwrap();
    let mut bytes = encode_compact_recovery_envelope(&source, &budget).unwrap();
    assert_eq!(bytes.len(), PAYLOAD_LIMIT);
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget).unwrap(),
        source
    );
    let last = source.lookahead.transactions.last_mut().unwrap();
    let mut changed = last.to_vec();
    changed.push(0);
    *last = changed.into();
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
    assert_eq!(
        encode_compact_recovery_envelope(&source, &budget),
        Err(RecoveryError::BudgetExceeded)
    );
    bytes.push(0);
    assert_eq!(
        decode_compact_recovery_envelope(&bytes, &budget),
        Err(RecoveryError::BudgetExceeded)
    );
}

#[test]
fn individual_transaction_receipt_and_validator_limits_are_admitted_before_encoding() {
    let original = envelope(&recovery_chain(), 1);
    let mut source = original.clone();
    source.lookahead.transactions = vec![Bytes::new(); 1_429];
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
    source = original.clone();
    source.lookahead.transactions = vec![vec![0; 131_073].into()];
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
    source = original.clone();
    source.execution.receipts[0] = vec![0; 4_194_305].into();
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
    source = original;
    source.finalized.commit.signatures = vec![source.finalized.commit.signatures[0].clone(); 65];
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
}

#[test]
fn protocol_sized_fields_can_still_exceed_recovery_payload_limit() {
    let mut source = envelope(&recovery_chain(), 1);
    source.execution.receipts[0] = vec![0; 4_194_304].into();
    source.lookahead.transactions = vec![vec![0; 4_096].into()];
    assert_eq!(
        validate_recovery_envelope_bounds(&source),
        Err(RecoveryError::BudgetExceeded)
    );
}
