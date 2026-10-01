// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;

use eve_state::{
    Address, ProofLimits, StateProofError, U256, build_account_proof, capture_state_view,
    development_state_budget, estimate_proof_reservation,
};

#[test]
fn two_hundred_fifty_six_slots_fit_declared_budget_and_next_slot_rejects() {
    let commit = support::genesis();
    let budget = development_state_budget();
    let view = capture_state_view(commit.state, commit.target, &budget).unwrap();
    let limits = ProofLimits {
        maximum_requested_slots: 256,
        maximum_proof_bytes: 1_048_576,
        maximum_rebuild_bytes: 64 * 1_048_576,
    };
    let mut slots: Vec<_> = (0_u64..256).map(U256::from).collect();
    let required = estimate_proof_reservation(&view, slots.len()).unwrap();
    let result =
        build_account_proof(&view, Address::repeat_byte(9), &slots, &limits, required).unwrap();
    assert_eq!(result.storage_proof.len(), 256);
    assert!(result.storage_proof.iter().all(|slot| slot.value.is_zero()));
    assert!(required > estimate_proof_reservation(&view, 64).unwrap());
    assert!(matches!(
        build_account_proof(
            &view,
            Address::repeat_byte(9),
            &slots,
            &limits,
            required - 1
        ),
        Err(StateProofError::Reservation { .. })
    ));
    slots.push(U256::from(256));
    // Test the canonical cap independently of a permissive caller policy.
    let permissive = ProofLimits {
        maximum_requested_slots: 512,
        ..limits
    };
    assert!(matches!(
        build_account_proof(
            &view,
            Address::repeat_byte(9),
            &slots,
            &permissive,
            64 * 1_048_576
        ),
        Err(StateProofError::Limit(_))
    ));
}
