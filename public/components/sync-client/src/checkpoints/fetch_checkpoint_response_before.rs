// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DownloadedCheckpointResponse, require_checkpoint_deadline::require_checkpoint_deadline,
    validate_checkpoint_response::validate_checkpoint_response,
};
use crate::{NativeRpcConfig, request_native_json_before, required_native_rpc_reservation};
use anyhow::{Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_storage::checkpoints::messages::{
    CheckpointMessageLimits, CheckpointRequest, checkpoint_response_stats,
    decode_checkpoint_response, encode_checkpoint_request, preflight_checkpoint_response,
    required_checkpoint_response_decode_reservation,
};
use std::time::Instant;

/// Every allocation envelope is leased before materialization, then retained with output.
/// Plain loopback CLASSICAL_DEV query data never authenticates a checkpoint or freshness.
pub fn fetch_checkpoint_response_before<L>(
    rpc: NativeRpcConfig,
    request: &CheckpointRequest,
    limits: &CheckpointMessageLimits,
    reserve: &mut impl FnMut(usize) -> Result<L>,
    caller_deadline: Instant,
) -> Result<DownloadedCheckpointResponse<L>> {
    let configured_deadline = Instant::now()
        .checked_add(rpc.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_DEADLINE"))?;
    let deadline = caller_deadline.min(configured_deadline);
    crate::validate_native_rpc_config(rpc)?;
    require_checkpoint_deadline(deadline)?;
    let transport_bytes = required_native_rpc_reservation(rpc)?;
    let transport_lease = reserve(transport_bytes)?;
    require_checkpoint_deadline(deadline)?;
    let request_bytes = encode_checkpoint_request(request)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_REQUEST"))?;
    let path = format!(
        "/abci_query?path=%22/eve/recovery/v1/checkpoint%22&data=0x{}&height=0&prove=false",
        hex::encode(request_bytes)
    );
    require_checkpoint_deadline(deadline)?;
    let response = request_native_json_before(rpc, &path, transport_bytes, deadline)?;
    require_checkpoint_deadline(deadline)?;
    let response = &response["response"];
    let code = response["code"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_STATUS"))?;
    match code {
        0 => (),
        1 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_WRONG_NETWORK"),
        2 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_UNSUPPORTED"),
        3 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_GAP"),
        4 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_RESOURCE"),
        5 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_NOT_READY"),
        6 => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_MALFORMED"),
        _ => anyhow::bail!("SYNC_CHECKPOINT_REMOTE_REFUSAL"),
    }
    let text = response["value"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_VALUE"))?;
    let maximum = limits
        .maximum_body_bytes
        .checked_add(16_384)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_LIMITS"))?;
    let maximum_base64 = maximum
        .div_ceil(3)
        .checked_mul(4)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_LIMITS"))?;
    ensure!(
        text.len() <= maximum_base64,
        "SYNC_CHECKPOINT_ENCODED_LIMIT"
    );
    require_checkpoint_deadline(deadline)?;
    let bytes = STANDARD
        .decode(text)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_BASE64"))?;
    let preflight = preflight_checkpoint_response(&bytes, limits)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_PREFLIGHT"))?;
    let materialization = required_checkpoint_response_decode_reservation(&preflight)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_RESERVATION"))?;
    require_checkpoint_deadline(deadline)?;
    let materialization_lease = reserve(materialization)?;
    require_checkpoint_deadline(deadline)?;
    let decoded = decode_checkpoint_response(&preflight, materialization)
        .map_err(|_| anyhow::anyhow!("SYNC_CHECKPOINT_DECODE"))?;
    validate_checkpoint_response(request, &decoded, limits)?;
    require_checkpoint_deadline(deadline)?;
    Ok(DownloadedCheckpointResponse {
        response: decoded,
        stats: checkpoint_response_stats(&preflight),
        _leases: [transport_lease, materialization_lease],
    })
}
