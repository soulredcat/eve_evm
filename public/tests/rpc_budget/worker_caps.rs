// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use alloy_primitives::Bytes;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn ta05_exact_worker_and_active_caps_reject_and_release_without_state_change() {
    let fixture = fixture(Bytes::new());
    let context = fixture.context;
    assert_eq!(context.active.available_permits(), 128);
    assert_eq!(context.simulations.available_permits(), 8);
    assert_eq!(context.proofs.available_permits(), 2);
    assert_eq!(context.histories.available_permits(), 4);
    assert_eq!(context.subscriptions.available_permits(), 256);
    let head =
        eve_storage::state::read_state_service(crate::rpc::durable_rpc_source(&context).unwrap().0)
            .unwrap()
            .commit()
            .clone();
    for (semaphore, permits, method, params) in [
        (Arc::clone(&context.active), 128, "eth_chainId", vec![]),
        (
            Arc::clone(&context.simulations),
            8,
            "eth_call",
            vec![
                json!({"from":fixture.sender,"to":fixture.contract}),
                json!("latest"),
            ],
        ),
        (
            Arc::clone(&context.proofs),
            2,
            "eth_getProof",
            vec![json!(fixture.contract), json!([]), json!("latest")],
        ),
        (Arc::clone(&context.histories), 4, "eth_blockNumber", vec![]),
    ] {
        let held = semaphore.try_acquire_many_owned(permits).unwrap();
        let failed = execute_rpc(Arc::clone(&context), method, params)
            .await
            .unwrap_err();
        assert_eq!(failed.code(), -32005);
        drop(held);
        assert_eq!(context.active.available_permits(), 128);
    }
    assert_eq!(
        execute_rpc(Arc::clone(&context), "eth_chainId", vec![])
            .await
            .unwrap(),
        json!("0x7a69")
    );
    assert_eq!(
        *eve_storage::state::read_state_service(
            crate::rpc::durable_rpc_source(&context).unwrap().0
        )
        .unwrap()
        .commit(),
        head
    );
}

#[tokio::test]
async fn ta05_global_byte_pressure_preserves_reserved_producer_capacity() {
    let fixture = fixture(Bytes::new());
    let context = fixture.context;
    assert_eq!(context.producer_bytes.num_permits(), 256 * 1024);
    let available = context.bytes.available_permits();
    assert_eq!(available, 256 * 1024);
    let held = Arc::clone(&context.bytes)
        .try_acquire_many_owned(available as u32)
        .unwrap();
    let failed = execute_rpc(
        Arc::clone(&context),
        "eth_getBalance",
        vec![json!(fixture.sender), json!("latest")],
    )
    .await
    .unwrap_err();
    assert_eq!(failed.code(), -32005);
    assert_eq!(
        context.producer_bytes.num_permits(),
        256 * 1024,
        "Public hostile read pressure cannot consume the producer's pre-reserved budget"
    );
    drop(held);
    assert_eq!(context.bytes.available_permits(), available);
    assert!(
        execute_rpc(
            context,
            "eth_getBalance",
            vec![json!(fixture.sender), json!("latest")]
        )
        .await
        .is_ok()
    );
}
