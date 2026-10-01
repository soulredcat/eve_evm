// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[test]
fn ta02_real_pinned_typescript_solidity_fixture_verifies_two_public_processes_and_restart() {
    let input = |name: &str| {
        let path = PathBuf::from(
            std::env::var_os(name)
                .unwrap_or_else(|| panic!("Missing mandatory verified acceptance input {name}")),
        );
        assert!(
            std::fs::symlink_metadata(&path).unwrap().is_file(),
            "{name} must name a regular file"
        );
        path
    };
    let node = input("EVE_ACCEPTANCE_NODE");
    let entry = input("EVE_ACCEPTANCE_CLIENT_ENTRY");
    let public = input("EVE_PUBLIC_DEV_BINARY");
    let solc = input("EVE_ACCEPTANCE_SOLC");
    let master = input("EVE_MASTER_DEV_BINARY");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap()
        .canonicalize()
        .unwrap();
    let local = root.join("local-tests/b2-client-acceptance");
    std::fs::create_dir_all(&local).unwrap();
    assert!(local.canonicalize().unwrap().starts_with(&root));
    let directory = tempfile::tempdir_in(local).unwrap();
    let output = Command::new(node)
        .arg(entry)
        .arg(&root)
        .arg(directory.path())
        .arg(public)
        .arg(solc)
        .arg(master)
        .current_dir(&root)
        .output()
        .expect("Run the actual pinned test client");
    assert!(
        output.status.success(),
        "Actual client failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let reports: Vec<Value> = stdout
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let complete: Vec<_> = reports
        .iter()
        .filter(|report| report["acceptance_complete"] == true)
        .collect();
    assert_eq!(
        complete.len(),
        1,
        "A single actual completed report is mandatory"
    );
    let report = complete[0];
    assert_eq!(report["result"], "PASS");
    assert_eq!(report["public_processes"], 2);
    assert_eq!(report["master_compositions"], 1);
    assert_eq!(report["solidity_sources"], 12);
    assert_eq!(report["verification_mode"], "LOCAL_DEV_UNAUTHENTICATED");
    assert_eq!(report["authenticated_finality"], false);
    assert!(report["signed_transactions"].as_u64().unwrap() >= 30);
    let expected = [
        "ta02_native_transfer_exact_fee_value_nonce_supply",
        "ta02_real_solidity_erc20_deployment_abi_transfer_and_seed",
        "ta02_two_pool_success_literal_reserves_and_ordered_events",
        "te02_second_leg_revert_rolls_back_all_token_pool_allowance_effects",
        "te02_create2_delegatecall_shanghai_destruction_reentry_refund_out_of_gas",
        "te03_selected_header_environment_and_blockhash_are_exact",
        "ta05_ws_oversize_response_string_ids_data_errors_and_http_fallback",
        "te06_call_estimation_revert_data_and_inactive_native_preserve_canonical_state",
        "ta06_independent_ethereumjs_account_storage_absence_and_corruption_proofs",
        "te01_rpc_signed_envelope_rejections_preserve_canonical_parent",
        "ta04_rpc_pending_gap_duplicate_replacement_and_multi_receipt_order",
        "ta06_real_websocket_heads_logs_history_and_unsubscribe",
        "ta01_rpc_encoding_canonical_header_transaction_and_receipt_roots",
        "ta03_unknown_future_history_and_unauthenticated_finality_are_distinct",
        "ta05_exact_request_body_batch_and_historical_query_limits",
        "ta06_actual_two_process_restart_reopens_exact_durable_history",
        "te03_independent_processes_preserve_exact_replay_after_restart",
        "ta02_actual_master_composition_receipt_history_restart_and_production_refusal",
    ];
    let cases = report["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        cases, expected,
        "No final client assertion may be silently skipped, renamed or duplicated"
    );
    assert_eq!(report["native_lifecycle"], "NOT_IMPLEMENTED before B5");
    assert_eq!(
        report["bft_consensus_starvation"],
        "NOT_IMPLEMENTED before B3/B6"
    );
}
