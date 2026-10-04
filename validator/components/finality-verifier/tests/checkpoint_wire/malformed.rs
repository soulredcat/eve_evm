// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{component_ranges, execution, fixture};
use eve_finality_verifier::{
    CheckpointWitnessWireError, encode_checkpoint_witness_wire, preflight_checkpoint_witness_wire,
};

#[test]
fn unknown_kind_wrong_domain_and_trailing_bytes_are_rejected() {
    let (chain, budget, limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    for mutation in 0..4 {
        let mut changed = bytes.clone();
        match mutation {
            0 => changed[0] ^= 1,
            1 => changed[b"EVE_CHECKPOINT_WITNESS_V1".len()] = 0,
            2 => changed[b"EVE_CHECKPOINT_WITNESS_V1".len()] = 2,
            _ => changed.push(0),
        }
        assert!(preflight_checkpoint_witness_wire(&changed, &budget, limits).is_err());
    }
}
#[test]
fn every_truncation_and_oversized_component_length_refuses_preflight() {
    let (chain, budget, limits) = fixture();
    let bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    for length in 0..bytes.len() {
        assert!(
            preflight_checkpoint_witness_wire(&bytes[..length], &budget, limits).is_err(),
            "{length}"
        );
    }
    for range in component_ranges(&bytes) {
        let mut changed = bytes.clone();
        changed[range.start - 4..range.start].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(preflight_checkpoint_witness_wire(&changed, &budget, limits).is_err());
    }
}
#[test]
fn bounded_invalid_target_version_is_rejected_by_the_state_owned_borrowed_scanner() {
    let (chain, budget, limits) = fixture();
    let mut bytes = encode_checkpoint_witness_wire(&execution(&chain), &budget, limits).unwrap();
    let version = component_ranges(&bytes)[1].clone();
    bytes[version.start..version.end].fill(0);
    assert!(matches!(
        preflight_checkpoint_witness_wire(&bytes, &budget, limits),
        Err(CheckpointWitnessWireError::State(_))
    ));
}
