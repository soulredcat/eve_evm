// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::execute_rpc::execute_rpc;
use alloy_primitives::{B256, Bytes};
use eve_evm::{ExecutionBlockInput, estimate_clone_reservation, execute_state_block};
use eve_storage::state::{commit_state, read_state_service};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn ta05_real_retained_history_accepts_1000_blocks_and_rejects_1001_block_log_scan() {
    let mut fixture = fixture(Bytes::new());
    let mut parent = read_state_service(&fixture.context.service)
        .unwrap()
        .commit()
        .clone();
    for _ in 0..1001 {
        let input = ExecutionBlockInput {
            timestamp: parent.target.timestamp + 1,
            proposer: fixture.sender,
            previous_consensus_hash: B256::ZERO,
        };
        let prepared = execute_state_block(
            &parent,
            &input,
            &[],
            &fixture.context.state_budget,
            estimate_clone_reservation(&parent.state).unwrap(),
        )
        .unwrap();
        // Actual validated local durable commits, with no validator finality claim.
        commit_state(&mut fixture.repository, &prepared.commit).unwrap();
        parent = prepared.commit;
    }
    assert_eq!(parent.target.height, 1001);
    let accepted = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getLogs",
        vec![json!({"fromBlock":"0x1","toBlock":"0x3e8"})],
    )
    .await
    .unwrap();
    assert_eq!(
        accepted,
        json!([]),
        "All 1000 retained empty blocks were scanned as a real available range"
    );
    let rejected = execute_rpc(
        Arc::clone(&fixture.context),
        "eth_getLogs",
        vec![json!({"fromBlock":"0x1","toBlock":"0x3e9"})],
    )
    .await
    .unwrap_err();
    assert_eq!(rejected.code(), -32005);
    assert_eq!(
        execute_rpc(fixture.context, "eth_blockNumber", vec![])
            .await
            .unwrap(),
        json!("0x3e9")
    );
}
