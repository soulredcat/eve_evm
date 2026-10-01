// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;

use std::{
    net::TcpListener,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result, ensure};
use sha2::{Digest, Sha256};
use support::{
    API_TRANSACTION, FixtureState, assert_actual_header_mapping, load_fixture_state, rpc_json,
    start_engine, start_fixture_server, validate_local_artifact_directory, wait_for_fixture_commit,
    wait_for_height,
};

#[test]
fn actual_pinned_engine_lifecycle_and_next_height_commitment() -> Result<()> {
    let binary = PathBuf::from(
        std::env::var_os("COMETBFT_BINARY")
            .context("COMETBFT_BINARY is required; absent actual-engine coverage cannot pass")?,
    );
    let expected_digest = std::env::var("COMETBFT_SHA256")
        .context("COMETBFT_SHA256 is required for the actual source-built binary")?;
    ensure!(
        hex::encode(Sha256::digest(std::fs::read(&binary)?)) == expected_digest,
        "CometBFT binary digest mismatch"
    );
    let version = Command::new(&binary).arg("version").output()?;
    ensure!(version.status.success(), "CometBFT version command failed");
    ensure!(
        String::from_utf8(version.stdout)?.trim()
            == "0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62",
        "API lifecycle requires exact v0.40.0 source identity and its upstream version string"
    );
    let artifact_root = PathBuf::from(
        std::env::var_os("EVE_B0_LOCAL_ARTIFACT_DIR")
            .context("EVE_B0_LOCAL_ARTIFACT_DIR must select ignored root local-tests storage")?,
    );
    let package = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = package.ancestors().nth(3).context("repository root")?;
    let artifact_root = validate_local_artifact_directory(&artifact_root, repository)?;
    let artifacts = tempfile::Builder::new()
        .prefix("abci-api-")
        .tempdir_in(artifact_root)?
        .keep();
    let home = artifacts.join("engine-home");
    let initialized = Command::new(&binary)
        .args(["init", "--home"])
        .arg(&home)
        .output()?;
    ensure!(
        initialized.status.success(),
        "CometBFT init failed: {}",
        String::from_utf8_lossy(&initialized.stderr)
    );
    let genesis_path = home.join("config/genesis.json");
    let mut genesis: serde_json::Value = serde_json::from_slice(&std::fs::read(&genesis_path)?)?;
    genesis["chain_id"] = "eve-b0-api-v1".into();
    genesis["genesis_time"] = "2026-09-30T00:00:00Z".into();
    genesis["validators"][0]["power"] = "10".into();
    std::fs::write(&genesis_path, serde_json::to_vec_pretty(&genesis)?)?;
    let rpc_listener = TcpListener::bind("127.0.0.1:0")?;
    let p2p_listener = TcpListener::bind("127.0.0.1:0")?;
    let rpc = rpc_listener.local_addr()?;
    let p2p = p2p_listener.local_addr()?;
    drop((rpc_listener, p2p_listener));
    let config_path = home.join("config/config.toml");
    let config = std::fs::read_to_string(&config_path)?
        .replace("timeout_commit = \"1s\"", "timeout_commit = \"100ms\"")
        .replace("addr_book_strict = true", "addr_book_strict = false");
    std::fs::write(config_path, config)?;
    let checkpoint = artifacts.join("application-checkpoint.json");
    let state = Arc::new(Mutex::new(FixtureState::default()));
    let server = start_fixture_server("127.0.0.1:0".parse()?, state.clone(), checkpoint.clone())?;
    let mut engine = start_engine(
        &binary,
        &home,
        server.address,
        rpc,
        p2p,
        &artifacts.join("engine-first.log"),
    )?;
    wait_for_height(rpc, 1, &mut engine.0)?;
    let transaction = rpc_json(
        rpc,
        &format!("/broadcast_tx_commit?tx=0x{}", hex::encode(API_TRANSACTION)),
    )?;
    ensure!(
        transaction["check_tx"]["code"] == 0,
        "fixture CheckTx failed"
    );
    ensure!(
        transaction["tx_result"]["code"] == 0,
        "fixture FinalizeBlock failed"
    );
    wait_for_height(rpc, 6, &mut engine.0)?;
    // Comet's block-store RPC height may advance before ABCI Commit is durable.
    wait_for_fixture_commit(&state, 6, Duration::from_secs(40))?;
    let mut headers = Vec::new();
    for height in 2..=5 {
        let result = rpc_json(rpc, &format!("/block?height={height}"))?;
        headers.push(result["block"]["header"].clone());
    }
    let first = state.lock().unwrap().clone();
    assert_actual_header_mapping(&headers, &first)?;
    for operation in [
        "Info",
        "InitChain",
        "CheckTx",
        "PrepareProposal",
        "ProcessProposal",
        "FinalizeBlock",
        "Commit",
    ] {
        ensure!(
            first.operations.contains(operation),
            "actual engine omitted {operation}"
        );
    }
    ensure!(
        first.transaction_count == 1,
        "fixture transaction effects duplicated"
    );
    drop(engine);
    drop(server);
    let restored = load_fixture_state(&checkpoint)?;
    ensure!(
        restored.committed_height >= 6,
        "API-test checkpoint did not persist"
    );
    let resumed_height = restored.committed_height + 2;
    let restarted_state = Arc::new(Mutex::new(restored));
    let restarted =
        start_fixture_server("127.0.0.1:0".parse()?, restarted_state.clone(), checkpoint)?;
    let mut engine = start_engine(
        &binary,
        &home,
        restarted.address,
        rpc,
        p2p,
        &artifacts.join("engine-restarted.log"),
    )?;
    wait_for_height(rpc, resumed_height, &mut engine.0)?;
    wait_for_fixture_commit(&restarted_state, resumed_height, Duration::from_secs(40))?;
    let resumed = restarted_state.lock().unwrap().clone();
    ensure!(
        resumed.operations.contains("Info"),
        "restart omitted application Info handshake"
    );
    ensure!(
        !resumed.operations.contains("InitChain"),
        "restart repeated InitChain"
    );
    ensure!(
        resumed.transaction_count == 0,
        "restart replay duplicated the fixture transaction"
    );
    std::fs::write(
        artifacts.join("summary.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "scope": "single-engine ABCI API fixture; not production EVM or B3 finality acceptance",
            "engine_release_tag": "v0.40.0", "engine_version_output": "0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62", "engine_sha256": expected_digest,
            "genesis_fixture_file_sha256": hex::encode(Sha256::digest(std::fs::read(genesis_path)?)),
            "operations": first.operations, "header_mapping": "H app hash at H+1; update at H+2; last commit at H+3",
            "restarted_committed_height": resumed.committed_height, "transaction_count": first.transaction_count
        }))?,
    )?;
    println!("API lifecycle evidence: {}", artifacts.display());
    Ok(())
}
