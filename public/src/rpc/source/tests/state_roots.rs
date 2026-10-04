// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixture::fixture;
use crate::rpc::{create_rpc_module, execute_rpc::execute_rpc};
use crate::sync::applied::{finish_applied_state_service, try_apply_recovery_bytes};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn state_roots_reports_exact_actual_applied_commit_and_distinguishes_uncertified_genesis() {
    let mut fixture = fixture(None);
    let module = create_rpc_module(Arc::clone(&fixture.context)).unwrap();
    assert!(
        module
            .method_names()
            .any(|name| name == "eve_getStateRoots")
    );
    let genesis = execute_rpc(Arc::clone(&fixture.context), "eve_getStateRoots", vec![])
        .await
        .unwrap();
    let initial = &fixture.chain.commits[0].target;
    assert_eq!(
        genesis,
        json!({
            "height":"0x0", "evmRoot":initial.evm_root.0, "systemRoot":initial.system_root.0,
            "executionHash":initial.execution_hash.0, "contentDigest":initial.content_digest,
            "applicationCommitment":null, "verificationMode":"LOCAL_GENESIS_TRUSTED",
            "authenticatedFinality":false,
        })
    );
    try_apply_recovery_bytes(&mut fixture.owner, &fixture.chain.records[0]).unwrap();
    let roots = execute_rpc(Arc::clone(&fixture.context), "eve_getStateRoots", vec![])
        .await
        .unwrap();
    let expected = &fixture.chain.commits[1].target;
    assert_eq!(
        roots,
        json!({
            "height":"0x1", "evmRoot":expected.evm_root.0, "systemRoot":expected.system_root.0,
            "executionHash":expected.execution_hash.0, "contentDigest":expected.content_digest,
            "applicationCommitment":expected.application.unwrap().0,
            "verificationMode":"AUTHENTICATED_IMPORT", "authenticatedFinality":true,
        })
    );
    assert_eq!(fixture.context.bytes.available_permits(), 32 * 1024);
    let arity = execute_rpc(
        Arc::clone(&fixture.context),
        "eve_getStateRoots",
        vec![json!("latest")],
    )
    .await
    .unwrap_err();
    assert_eq!(arity.code(), -32602);
    drop(finish_applied_state_service(fixture.owner));
}

#[tokio::test]
async fn locally_constructed_nonzero_commit_cannot_claim_authenticated_roots() {
    let fixture = fixture(None);
    let parent = &fixture.chain.commits[0];
    let budget = eve_storage::state::development_state_storage_budget();
    let mut repository = eve_storage::state::open_state_repository(
        &fixture._directory.path().join("durable"),
        parent,
        budget,
    )
    .unwrap();
    assert!(
        eve_storage::state::ensure_history_index(
            &mut repository,
            eve_storage::state::HistoryReadBudget {
                maximum_block_bytes: 16 * 1_048_576,
                maximum_rebuild_blocks: 128,
                maximum_index_batch_bytes: 16 * 1_048_576,
            }
        )
        .unwrap()
        .complete
    );
    let context = crate::rpc::create_rpc_context(
        &repository,
        crate::mempool::start_mempool(
            Arc::new(parent.clone()),
            crate::mempool::MempoolLimits::default(),
        ),
        budget.logical,
        eve_node_policy::ZoneId(1),
    );
    let prepared = eve_evm::execute_state_block(
        parent,
        &eve_evm::ExecutionBlockInput {
            timestamp: parent.target.timestamp + 1,
            proposer: alloy_primitives::Address::ZERO,
            previous_consensus_hash: alloy_primitives::B256::ZERO,
        },
        &[],
        &context.state_budget,
        eve_evm::estimate_clone_reservation(&parent.state).unwrap(),
    )
    .unwrap();
    eve_storage::state::commit_state(&mut repository, &prepared.commit).unwrap();
    let roots = execute_rpc(context, "eve_getStateRoots", vec![])
        .await
        .unwrap();
    assert_eq!(roots["height"], "0x1");
    assert_eq!(
        roots["applicationCommitment"],
        json!(prepared.commit.target.application.unwrap().0)
    );
    assert_eq!(
        roots["systemRoot"],
        json!(prepared.commit.target.system_root.0)
    );
    assert_eq!(roots["verificationMode"], "LOCAL_DEV_UNAUTHENTICATED");
    assert_eq!(roots["authenticatedFinality"], false);
    drop(finish_applied_state_service(fixture.owner));
}
