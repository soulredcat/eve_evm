// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{NativeRpcConfig, request_native_json};
use anyhow::{Result, ensure};
use eve_consensus_comet::rpc_decoding::{
    decode_native_block, decode_native_commit, decode_native_header,
};
use eve_finality_verifier::{NativeDataFrame, NativeFrame};
use eve_state::Bytes;

/// Actual native block and matching certificate, still untrusted until canonical
/// history/H+1 verification. The caller keeps its actual working lease throughout.
pub fn fetch_native_frame(
    config: NativeRpcConfig,
    height: u64,
    reserved_bytes: usize,
) -> Result<NativeDataFrame> {
    ensure!(
        height > 0 && height <= i64::MAX as u64,
        "SYNC_NATIVE_HEIGHT"
    );
    let block_value =
        request_native_json(config, &format!("/block?height={height}"), reserved_bytes)?;
    let block = decode_native_block(&block_value)?;
    drop(block_value);
    let commit_value =
        request_native_json(config, &format!("/commit?height={height}"), reserved_bytes)?;
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
    Ok(NativeDataFrame {
        frame: NativeFrame {
            block_id: block.block_id,
            header,
            commit,
        },
        transactions,
    })
}
