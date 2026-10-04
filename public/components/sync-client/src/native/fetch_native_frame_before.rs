// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{NativeRpcConfig, request_native_json_before};
use anyhow::{Result, ensure};
use eve_consensus_comet::rpc_decoding::{
    decode_native_block, decode_native_commit, decode_native_header,
};
use eve_finality_verifier::{NativeDataFrame, NativeFrame};
use eve_state::Bytes;
use std::time::Instant;

/// One caller deadline spans both native queries and owned canonical parser materialization.
pub fn fetch_native_frame_before(
    config: NativeRpcConfig,
    height: u64,
    reserved_bytes: usize,
    caller_deadline: Instant,
) -> Result<NativeDataFrame> {
    let configured_deadline = Instant::now()
        .checked_add(config.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_NATIVE_DEADLINE"))?;
    let deadline = caller_deadline.min(configured_deadline);
    crate::validate_native_rpc_config(config)?;
    ensure!(
        height > 0 && height <= i64::MAX as u64,
        "SYNC_NATIVE_HEIGHT"
    );
    ensure!(Instant::now() < deadline, "SYNC_NATIVE_DEADLINE");
    let block_value = request_native_json_before(
        config,
        &format!("/block?height={height}"),
        reserved_bytes,
        deadline,
    )?;
    ensure!(Instant::now() < deadline, "SYNC_NATIVE_DEADLINE");
    let block = decode_native_block(&block_value)?;
    drop(block_value);
    ensure!(Instant::now() < deadline, "SYNC_NATIVE_DEADLINE");
    let commit_value = request_native_json_before(
        config,
        &format!("/commit?height={height}"),
        reserved_bytes,
        deadline,
    )?;
    ensure!(Instant::now() < deadline, "SYNC_NATIVE_DEADLINE");
    let signed = &commit_value["signed_header"];
    let header = decode_native_header(&signed["header"])?;
    let commit = decode_native_commit(&signed["commit"])?;
    ensure!(
        header == block.header
            && header.height == height as i64
            && commit.height == height as i64
            && commit.block_id.as_ref() == Some(&block.block_id),
        "SYNC_NATIVE_BLOCK_CERTIFICATE_MISMATCH"
    );
    let transactions = block.transactions.into_iter().map(Bytes::from).collect();
    ensure!(Instant::now() < deadline, "SYNC_NATIVE_DEADLINE");
    Ok(NativeDataFrame {
        frame: NativeFrame {
            block_id: block.block_id,
            header,
            commit,
        },
        transactions,
    })
}
