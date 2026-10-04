// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DownloadedStateDelta, fetch_state_delta_chunk::fetch_state_delta_chunk,
    required_state_delta_download_reservation,
};
use crate::NativeRpcConfig;
use anyhow::{Context, Result, ensure};
use eve_state::{MAXIMUM_STATE_DELTA_PAYLOAD_BYTES, StateDeltaRequest, hash_state_delta_bytes};
use std::time::Instant;

/// Consecutive, identity-bound chunks from one explicit source. A caller holds
/// actual working leases before network/buffer allocation and while output survives.
pub fn fetch_state_delta_bytes(
    config: NativeRpcConfig,
    request: StateDeltaRequest,
    reserved_bytes: usize,
) -> Result<DownloadedStateDelta> {
    ensure!(
        reserved_bytes >= required_state_delta_download_reservation(config)?,
        "SYNC_DELTA_RESERVATION"
    );
    ensure!(
        request.offset == 0
            && request.maximum_chunk_bytes > 0
            && request.parent.height.checked_add(1) == Some(request.target_height),
        "SYNC_DELTA_BASE"
    );
    let deadline = Instant::now()
        .checked_add(config.timeout)
        .context("SYNC_DELTA_DEADLINE")?;
    let mut next = request.clone();
    let first = fetch_state_delta_chunk(config, &next, reserved_bytes)?;
    ensure!(Instant::now() < deadline, "SYNC_DELTA_DEADLINE");
    ensure!(
        first.parent == request.parent
            && first.target.height == request.target_height
            && first.offset == 0
            && first.total_length > 0
            && first.total_length <= MAXIMUM_STATE_DELTA_PAYLOAD_BYTES as u64,
        "SYNC_DELTA_FIRST_CHUNK"
    );
    let length = usize::try_from(first.total_length).context("SYNC_DELTA_LENGTH")?;
    let maximum_requests = length.div_ceil(request.maximum_chunk_bytes as usize);
    ensure!(maximum_requests <= 4096, "SYNC_DELTA_REQUEST_COUNT");
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(length)
        .context("SYNC_DELTA_ALLOCATION")?;
    ensure!(bytes.capacity() == length, "SYNC_DELTA_CAPACITY");
    let parent = first.parent.clone();
    let target = first.target.clone();
    let identity = first.body_sha256;
    let total = first.total_length;
    let mut chunk = first;
    for index in 0..maximum_requests {
        ensure!(
            chunk.parent == parent
                && chunk.target == target
                && chunk.body_sha256 == identity
                && chunk.total_length == total
                && chunk.offset == bytes.len() as u64
                && chunk.data.len()
                    == (length - bytes.len()).min(request.maximum_chunk_bytes as usize),
            "SYNC_DELTA_CHUNK_SEQUENCE"
        );
        bytes.extend_from_slice(&chunk.data);
        if bytes.len() == length {
            break;
        }
        ensure!(index + 1 < maximum_requests, "SYNC_DELTA_INCOMPLETE");
        next.offset = bytes.len() as u64;
        let mut remaining = config;
        remaining.timeout = deadline.saturating_duration_since(Instant::now());
        ensure!(!remaining.timeout.is_zero(), "SYNC_DELTA_DEADLINE");
        chunk = fetch_state_delta_chunk(remaining, &next, reserved_bytes)?;
        ensure!(Instant::now() < deadline, "SYNC_DELTA_DEADLINE");
    }
    ensure!(
        bytes.len() == length && hash_state_delta_bytes(&bytes) == identity,
        "SYNC_DELTA_BODY_IDENTITY"
    );
    ensure!(Instant::now() < deadline, "SYNC_DELTA_DEADLINE");
    Ok(DownloadedStateDelta {
        parent,
        target,
        durable_tip: chunk.durable_tip,
        body_sha256: identity,
        bytes,
    })
}
