// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::{execute_rpc::execute_rpc, selectors::capture_current_rpc_state};
use crate::sync::applied::{
    capture_applied_state, finish_applied_state_service, observe_estimated_working,
    try_apply_recovery_bytes,
};
use eve_state::estimate_genesis_initialization_reservation;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn captured_rpc_state_retains_the_actual_applied_generation_until_its_last_owner_drops() {
    let mut fixture = fixture(None);
    let old = capture_current_rpc_state(&fixture.context).unwrap();
    let genesis_charge =
        estimate_genesis_initialization_reservation(&fixture.chain.genesis, &fixture.budget)
            .unwrap() as u64;
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[0]).unwrap();
    fixture
        .context
        .pool
        .applied_committed(capture_applied_state(&fixture.reader).unwrap())
        .await
        .unwrap();
    let charged = observe_estimated_working(&fixture.reader)
        .unwrap()
        .reserved_estimated_bytes;
    assert_eq!(old.commit(), &fixture.chain.commits[0]);
    drop(old);
    assert_eq!(
        observe_estimated_working(&fixture.reader)
            .unwrap()
            .reserved_estimated_bytes,
        charged - genesis_charge
    );
    assert_eq!(
        capture_current_rpc_state(&fixture.context)
            .unwrap()
            .commit(),
        &fixture.chain.commits[1]
    );
    drop(finish_applied_state_service(fixture.owner));
}

#[tokio::test]
async fn explicit_applied_vm_profile_and_proof_workers_refuse_pressure_and_release_every_rpc_permit()
 {
    let fixture = fixture(None);
    let before = capture_current_rpc_state(&fixture.context).unwrap();
    assert_eq!(fixture.context.simulation_memory_bytes, 8 * 1_048_576);
    let held = Arc::clone(&fixture.context.bytes)
        .try_acquire_many_owned(4 * 1024)
        .unwrap();
    let pressure = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_call",
        vec![
            json!({"from":fixture.sender,"to":fixture.contract}),
            json!("latest"),
        ],
    )
    .await
    .unwrap_err();
    assert_eq!(pressure.code(), -32005);
    assert_eq!(fixture.context.bytes.available_permits(), 28 * 1024);
    assert_eq!(fixture.context.simulations.available_permits(), 8);
    drop(held);
    let proof_slots = Arc::clone(&fixture.context.proofs)
        .try_acquire_many_owned(2)
        .unwrap();
    assert_eq!(
        execute_rpc(
            Arc::clone(&fixture.context),
            "eth_getProof",
            vec![json!(fixture.contract), json!([]), json!("latest")]
        )
        .await
        .unwrap_err()
        .code(),
        -32005
    );
    drop(proof_slots);
    assert_eq!(fixture.context.bytes.available_permits(), 32 * 1024);
    assert_eq!(fixture.context.active.available_permits(), 128);
    assert_eq!(
        capture_current_rpc_state(&fixture.context)
            .unwrap()
            .commit(),
        before.commit()
    );
    drop(finish_applied_state_service(fixture.owner));
}
