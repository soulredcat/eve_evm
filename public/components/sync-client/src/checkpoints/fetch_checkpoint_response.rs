// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DownloadedCheckpointResponse,
    fetch_checkpoint_response_before::fetch_checkpoint_response_before,
};
use crate::NativeRpcConfig;
use anyhow::Result;
use eve_storage::checkpoints::messages::{CheckpointMessageLimits, CheckpointRequest};
use std::time::Instant;

/// Configured deadline includes reservation, encoding, HTTP, parsing and final materialization.
pub fn fetch_checkpoint_response<L>(
    rpc: NativeRpcConfig,
    request: &CheckpointRequest,
    limits: &CheckpointMessageLimits,
    reserve: &mut impl FnMut(usize) -> Result<L>,
) -> Result<DownloadedCheckpointResponse<L>> {
    let deadline = Instant::now()
        .checked_add(rpc.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_DEADLINE"))?;
    fetch_checkpoint_response_before(rpc, request, limits, reserve, deadline)
}
