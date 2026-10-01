// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use alloy_primitives::Bytes;
use serde_json::json;
use std::{sync::Arc, time::Duration};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ta05_cancelled_real_evm_call_keeps_leases_until_blocking_execution_exits() {
    // JUMPDEST, PUSH1(0), JUMP is a real Shanghai gas-bounded busy loop.
    let fixture = fixture(Bytes::from_static(&[0x5b, 0x60, 0, 0x56]));
    let context = fixture.context;
    let before = eve_storage::state::read_state_service(&context.service)
        .unwrap()
        .commit()
        .clone();
    let baseline = context.bytes.available_permits();
    let shared = Arc::clone(&context);
    let params = vec![
        json!({"from":fixture.sender,"to":fixture.contract,"gas":"0x1c9c380"}),
        json!("latest"),
    ];
    let task = tokio::spawn(async move { execute_rpc(shared, "eth_call", params).await });
    tokio::time::timeout(Duration::from_secs(10), async {
        while context.simulations.available_permits() != 7
            || context.bytes.available_permits() >= baseline - 32 * 1024
        {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("The actual blocking EVM must begin and acquire its clone/memory leases");
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(
        context.simulations.available_permits(),
        7,
        "Cancelling a caller cannot create another execution slot while its blocking worker still runs"
    );
    assert!(context.bytes.available_permits() < baseline);
    assert_eq!(
        execute_rpc(Arc::clone(&context), "eth_chainId", vec![])
            .await
            .unwrap(),
        json!("0x7a69"),
        "Ordinary reads remain available during the cancelled local simulation"
    );
    tokio::time::timeout(Duration::from_secs(30), async {
        while context.simulations.available_permits() != 8
            || context.bytes.available_permits() != baseline
        {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("Gas-bounded blocking execution must eventually release every lease");
    assert_eq!(
        *eve_storage::state::read_state_service(&context.service)
            .unwrap()
            .commit(),
        before
    );
}
