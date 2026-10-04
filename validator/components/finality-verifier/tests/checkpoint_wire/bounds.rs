// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{component_ranges, execution, fixture, lookahead};
use eve_finality_verifier::{
    CheckpointWitnessWireError, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
    checkpoint_witness_wire_budget, checkpoint_witness_wire_limits, decode_checkpoint_witness_wire,
    encode_checkpoint_witness_wire, preflight_checkpoint_witness_wire,
    required_checkpoint_witness_decode_reservation,
};
use eve_state::Bytes;

#[test]
fn actual_lease_parameter_is_required_before_owned_decode() {
    let (chain, budget, limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    let preflight = preflight_checkpoint_witness_wire(&bytes, &budget, limits).unwrap();
    let required = required_checkpoint_witness_decode_reservation(&preflight).unwrap();
    assert_eq!(
        decode_checkpoint_witness_wire(&preflight, required - 1).err(),
        Some(CheckpointWitnessWireError::ReservationTooSmall)
    );
    assert!(decode_checkpoint_witness_wire(&preflight, required).is_ok());
}
#[test]
fn preflight_freezes_caller_budget_and_limits() {
    let (chain, mut budget, mut limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    let preflight = preflight_checkpoint_witness_wire(&bytes, &budget, limits).unwrap();
    let operations = budget.maximum_journal_operations;
    let witness_bytes = limits.maximum_witness_bytes;
    budget.maximum_journal_operations = 1;
    limits.maximum_witness_bytes = 1;
    assert_eq!(
        checkpoint_witness_wire_budget(&preflight).maximum_journal_operations,
        operations
    );
    assert_eq!(
        checkpoint_witness_wire_limits(&preflight).maximum_witness_bytes,
        witness_bytes
    );
    let required = required_checkpoint_witness_decode_reservation(&preflight).unwrap();
    assert!(decode_checkpoint_witness_wire(&preflight, required).is_ok());
    assert!(preflight_checkpoint_witness_wire(&bytes, &budget, limits).is_err());
}
#[test]
fn zero_invalid_and_oversized_limits_refuse_before_component_allocation() {
    let (chain, budget, limits) = fixture();
    let input = execution(&chain);
    let bytes = encode_checkpoint_witness_wire(&input, &budget, limits).unwrap();
    for value in [
        0,
        bytes.len() - 1,
        MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES + 1,
    ] {
        let changed = eve_finality_verifier::CheckpointLimits {
            maximum_witness_bytes: value,
            ..limits
        };
        assert!(preflight_checkpoint_witness_wire(&bytes, &budget, changed).is_err());
        assert!(encode_checkpoint_witness_wire(&input, &budget, changed).is_err());
    }
    let changed = eve_finality_verifier::CheckpointLimits {
        maximum_height_gap: 0,
        ..limits
    };
    assert_eq!(
        preflight_checkpoint_witness_wire(&bytes, &budget, changed).err(),
        Some(CheckpointWitnessWireError::InvalidLimits)
    );
}
#[test]
fn transaction_counts_are_bounded_for_native_and_execution_fields() {
    let (chain, mut budget, limits) = fixture();
    let mut input = lookahead(&chain);
    let eve_finality_verifier::CheckpointWitness::Lookahead(native) = &mut input else {
        unreachable!()
    };
    native.transactions = vec![Bytes::from_static(&[1]), Bytes::from_static(&[2])];
    let bytes = encode_checkpoint_witness_wire(&input, &budget, limits).unwrap();
    budget.maximum_journal_operations = 1;
    assert_eq!(
        preflight_checkpoint_witness_wire(&bytes, &budget, limits).err(),
        Some(CheckpointWitnessWireError::BudgetExceeded)
    );
    let mut input = execution(&chain);
    let eve_finality_verifier::CheckpointWitness::Execution(witness) = &mut input else {
        unreachable!()
    };
    witness
        .block
        .transactions
        .push(witness.block.transactions[0].clone());
    witness
        .block
        .receipts
        .push(witness.block.receipts[0].clone());
    let mut roomy = budget;
    roomy.maximum_journal_operations = 2;
    let bytes = encode_checkpoint_witness_wire(&input, &roomy, limits).unwrap();
    assert!(preflight_checkpoint_witness_wire(&bytes, &budget, limits).is_err());
}
#[test]
fn malformed_native_count_and_execution_lists_are_scanned_before_decode() {
    let (chain, budget, limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    let ranges = component_ranges(&bytes);
    let mut changed = bytes.clone();
    let native = &ranges[0];
    let frame_length =
        u32::from_be_bytes(changed[native.start..native.start + 4].try_into().unwrap()) as usize;
    let count = native.start + 4 + frame_length;
    changed[count..count + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(preflight_checkpoint_witness_wire(&changed, &budget, limits).is_err());
    let mut changed = bytes;
    changed[ranges[2].start..ranges[2].end].fill(0);
    assert!(preflight_checkpoint_witness_wire(&changed, &budget, limits).is_err());
}

#[test]
fn native_signature_and_header_limits_are_measured_before_owned_encoding() {
    let (chain, budget, limits) = fixture();
    let mut input = execution(&chain);
    let eve_finality_verifier::CheckpointWitness::Execution(witness) = &mut input else {
        unreachable!()
    };
    witness.native.frame.commit.signatures =
        vec![witness.native.frame.commit.signatures[0].clone(); 65];
    assert!(encode_checkpoint_witness_wire(&input, &budget, limits).is_err());
    let mut input = execution(&chain);
    let eve_finality_verifier::CheckpointWitness::Execution(witness) = &mut input else {
        unreachable!()
    };
    witness.native.frame.header.chain_id = "x".repeat(51);
    assert!(encode_checkpoint_witness_wire(&input, &budget, limits).is_err());
}
