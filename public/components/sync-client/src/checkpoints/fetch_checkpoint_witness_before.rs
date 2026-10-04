// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointWitnessHeights, DownloadedCheckpointResponse, DownloadedCheckpointWitness,
    estimate_checkpoint_witness_materialization::estimate_checkpoint_witness_materialization,
    fetch_checkpoint_response_before::fetch_checkpoint_response_before,
    require_checkpoint_deadline::require_checkpoint_deadline, types::CheckpointWitnessLeases,
};
use crate::{NativeRpcConfig, fetch_native_frame_before, required_native_rpc_reservation};
use anyhow::{Result, ensure};
use eve_finality_verifier::{
    CheckpointExecutionWitness, CheckpointLimits, CheckpointWitness,
    MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES, encode_checkpoint_witness_wire,
    measure_checkpoint_witness_wire,
};
use eve_state::{StateBudget, StateVersion};
use eve_storage::checkpoints::messages::{
    CheckpointMessageLimits, CheckpointRequest, CheckpointRequestKind, CheckpointResponse,
    MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES, MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES,
    MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES,
};
use std::time::Instant;

/// Heights through H fetch execution metadata; only H+1 produces a closing lookahead.
/// Moving owned metadata retains its original leases; returned bytes grant no finality.
pub fn fetch_checkpoint_witness_before<L>(
    rpc: NativeRpcConfig,
    genesis: &StateVersion,
    heights: CheckpointWitnessHeights,
    budget: &StateBudget,
    limits: CheckpointLimits,
    reserve: &mut impl FnMut(usize) -> Result<L>,
    caller_deadline: Instant,
) -> Result<DownloadedCheckpointWitness<L>> {
    let configured_deadline = Instant::now()
        .checked_add(rpc.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_DEADLINE"))?;
    let deadline = caller_deadline.min(configured_deadline);
    crate::validate_native_rpc_config(rpc)?;
    require_checkpoint_deadline(deadline)?;
    let checkpoint_height = heights.checkpoint;
    let witness_height = heights.witness;
    eve_state::validate_state_budget(budget)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_LIMITS"))?;
    ensure!(
        genesis.height == 0 && genesis.identity.network_name.len() <= 1_024,
        "SYNC_CHECKPOINT_GENESIS"
    );
    let closing = checkpoint_height
        .checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_HEIGHT"))?;
    ensure!(
        checkpoint_height > 0
            && closing <= i64::MAX as u64
            && witness_height > 0
            && witness_height <= closing,
        "SYNC_CHECKPOINT_HEIGHT"
    );
    ensure!(
        limits.maximum_height_gap > 0
            && limits.maximum_witness_bytes > 0
            && limits.maximum_witness_bytes <= MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES,
        "SYNC_CHECKPOINT_LIMITS"
    );
    let native_bytes = required_native_rpc_reservation(rpc)?;
    let native_lease = reserve(native_bytes)?;
    require_checkpoint_deadline(deadline)?;
    let metadata = if witness_height <= checkpoint_height {
        let request = CheckpointRequest {
            genesis: genesis.clone(),
            height: witness_height,
            kind: CheckpointRequestKind::Execution,
        };
        let message_limits = CheckpointMessageLimits {
            logical: *budget,
            maximum_body_bytes: budget
                .maximum_commit_bytes
                .min(MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES),
            maximum_manifest_bytes: MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES,
            maximum_chunk_bytes: MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES,
        };
        Some(fetch_checkpoint_response_before(
            rpc,
            &request,
            &message_limits,
            reserve,
            deadline,
        )?)
    } else {
        None
    };
    require_checkpoint_deadline(deadline)?;
    let native = fetch_native_frame_before(rpc, witness_height, native_bytes, deadline)?;
    require_checkpoint_deadline(deadline)?;
    if let Some(metadata) = &metadata {
        let CheckpointResponse::Execution { block, .. } = &metadata.response else {
            anyhow::bail!("SYNC_CHECKPOINT_KIND")
        };
        ensure!(
            native.transactions.iter().eq(block.transactions.iter()),
            "SYNC_CHECKPOINT_NATIVE_TRANSACTIONS"
        );
    }
    let charge = estimate_checkpoint_witness_materialization(
        &native,
        metadata
            .as_ref()
            .map_or(0, |downloaded| downloaded.stats.encoded_bytes),
    )?;
    let materialization_lease = reserve(charge)?;
    require_checkpoint_deadline(deadline)?;
    let (witness, metadata_leases) = match metadata {
        Some(DownloadedCheckpointResponse {
            response, _leases, ..
        }) => {
            let CheckpointResponse::Execution { target, block } = response else {
                anyhow::bail!("SYNC_CHECKPOINT_KIND")
            };
            (
                CheckpointWitness::Execution(Box::new(CheckpointExecutionWitness {
                    native,
                    version: target,
                    block: *block,
                })),
                Some(_leases),
            )
        }
        None => (CheckpointWitness::Lookahead(Box::new(native)), None),
    };
    let measured = measure_checkpoint_witness_wire(&witness, budget, limits)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_WITNESS_BOUNDS"))?;
    ensure!(measured <= charge, "SYNC_CHECKPOINT_WITNESS_RESERVATION");
    require_checkpoint_deadline(deadline)?;
    let wire = encode_checkpoint_witness_wire(&witness, budget, limits)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_WITNESS_ENCODING"))?;
    require_checkpoint_deadline(deadline)?;
    Ok(DownloadedCheckpointWitness {
        wire,
        _leases: CheckpointWitnessLeases {
            _metadata: metadata_leases,
            _native: native_lease,
            _materialization: materialization_lease,
        },
    })
}
