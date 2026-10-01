// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use alloy_consensus::{Receipt, ReceiptEnvelope};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Bloom, Bytes};
use eve_state::{StateError, development_state_budget, validate_block_payload};
use support::{blocks::execution_payload, fixtures::identity};

#[test]
fn ts02_literal_signed_transaction_and_receipt_are_canonical_recovery_inputs() {
    validate_block_payload(
        &execution_payload(),
        &identity(),
        &development_state_budget(),
    )
    .unwrap();
}

#[test]
fn ts02_garbage_or_trailing_envelopes_are_not_replayable_payloads() {
    let original = execution_payload();
    let mut garbage = original.clone();
    garbage.transactions[0] = Bytes::from_static(&[0xff]);
    assert_eq!(
        validate_block_payload(&garbage, &identity(), &development_state_budget()),
        Err(StateError::MalformedEncoding)
    );
    let mut trailing_transaction = original.clone();
    let mut transaction = trailing_transaction.transactions[0].to_vec();
    transaction.push(0);
    trailing_transaction.transactions[0] = transaction.into();
    assert_eq!(
        validate_block_payload(
            &trailing_transaction,
            &identity(),
            &development_state_budget()
        ),
        Err(StateError::NonCanonicalEncoding)
    );
    let mut trailing_receipt = original;
    let mut receipt = trailing_receipt.receipts[0].to_vec();
    receipt.push(0);
    trailing_receipt.receipts[0] = receipt.into();
    assert_eq!(
        validate_block_payload(&trailing_receipt, &identity(), &development_state_budget()),
        Err(StateError::NonCanonicalEncoding)
    );
}

#[test]
fn ts02_receipt_type_cumulative_gas_and_header_bloom_must_match() {
    let original = execution_payload();
    let mut mismatched_type = original.clone();
    mismatched_type.receipts[0] = [vec![1], mismatched_type.receipts[0].to_vec()]
        .concat()
        .into();
    assert_eq!(
        validate_block_payload(&mismatched_type, &identity(), &development_state_budget()),
        Err(StateError::NonCanonicalEncoding)
    );
    let mut excessive_gas = original.clone();
    excessive_gas.receipts[0] = ReceiptEnvelope::Legacy(
        Receipt {
            status: true.into(),
            cumulative_gas_used: 100_001,
            logs: Vec::new(),
        }
        .with_bloom(),
    )
    .encoded_2718()
    .into();
    assert_eq!(
        validate_block_payload(&excessive_gas, &identity(), &development_state_budget()),
        Err(StateError::CommitMismatch)
    );
    let mut wrong_bloom = original.clone();
    wrong_bloom.header.logs_bloom = Bloom::from([1; 256]);
    assert_eq!(
        validate_block_payload(&wrong_bloom, &identity(), &development_state_budget()),
        Err(StateError::CommitMismatch)
    );
    let mut wrong_gas = original;
    wrong_gas.header.gas_used += 1;
    assert_eq!(
        validate_block_payload(&wrong_gas, &identity(), &development_state_budget()),
        Err(StateError::CommitMismatch)
    );
}

#[test]
fn ts02_canonical_envelope_cannot_substitute_another_chain_identity() {
    let mut wrong = identity();
    wrong.evm_chain_id = 999;
    assert_eq!(
        validate_block_payload(&execution_payload(), &wrong, &development_state_budget()),
        Err(StateError::NonCanonicalEncoding)
    );
}
