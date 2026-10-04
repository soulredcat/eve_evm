// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DownloadedAuthenticatedImport,
    estimate_import_materialization::estimate_import_materialization,
    required_authenticated_import_download_reservation,
};
use crate::{NativeRpcConfig, fetch_native_frame, fetch_state_delta_bytes};
use anyhow::{Result, ensure};
use eve_finality_verifier::{AuthenticatedImportInput, encode_logical_import_wire};
use eve_state::{
    StateBudget, StateDeltaRequest, decode_state_delta_payload, preflight_state_delta_payload,
    state_delta_payload_stats,
};

/// Both roles provide real leases from their own pool. No source metadata or
/// assembled input grants finality; each consumer must canonically prepare it.
pub fn fetch_authenticated_import_wire<L>(
    rpc: NativeRpcConfig,
    request: StateDeltaRequest,
    budget: &StateBudget,
    mut reserve: impl FnMut(usize) -> Result<L>,
) -> Result<DownloadedAuthenticatedImport<L>> {
    crate::validate_native_rpc_config(rpc)?;
    let height = request.target_height;
    let parent = request.parent.clone();
    let lookahead_height = height
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("SYNC_IMPORT_HEIGHT"))?;
    let transport_bytes = required_authenticated_import_download_reservation(rpc)?;
    let transport_lease = reserve(transport_bytes)?;
    let downloaded = fetch_state_delta_bytes(rpc, request, transport_bytes)?;
    let preflight = preflight_state_delta_payload(&downloaded.bytes, budget)
        .map_err(|error| anyhow::anyhow!("SYNC_IMPORT_DELTA: {error:?}"))?;
    let materialization = estimate_import_materialization(state_delta_payload_stats(&preflight))?;
    let materialization_lease = reserve(materialization)?;
    let delta = decode_state_delta_payload(&preflight)
        .map_err(|error| anyhow::anyhow!("SYNC_IMPORT_DELTA: {error:?}"))?;
    ensure!(
        delta.journal.parent == parent && delta.journal.target_height == height,
        "SYNC_IMPORT_PARENT"
    );
    let finalized = fetch_native_frame(rpc, height, transport_bytes)?;
    ensure!(
        finalized
            .transactions
            .iter()
            .eq(delta.execution.transactions.iter()),
        "SYNC_IMPORT_NATIVE_TRANSACTIONS"
    );
    let lookahead = fetch_native_frame(rpc, lookahead_height, transport_bytes)?;
    let input = AuthenticatedImportInput {
        journal: delta.journal,
        execution: delta.execution,
        finalized: finalized.frame,
        lookahead,
    };
    let wire = encode_logical_import_wire(&input, budget)
        .map_err(|error| anyhow::anyhow!("SYNC_IMPORT_WIRE: {error:?}"))?;
    Ok(DownloadedAuthenticatedImport {
        wire,
        target: downloaded.target,
        _leases: [transport_lease, materialization_lease],
    })
}
