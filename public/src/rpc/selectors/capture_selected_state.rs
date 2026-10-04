// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{resolve_height, types::SelectedState};
use crate::rpc::{RpcContext, RpcStateSource, durable_rpc_source, errors::rpc_error};
use eve_storage::state::{capture_history_snapshot, capture_state_snapshot, read_snapshot_commit};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::Arc;
pub(crate) fn capture_selected_state(
    context: &RpcContext,
    selector: &Value,
) -> Result<SelectedState, ErrorObjectOwned> {
    if matches!(&context.source, RpcStateSource::Applied { .. }) {
        return super::capture_applied_selected_state::capture_applied_selected_state(
            context, selector,
        );
    }
    if selector.as_str() == Some("pending") {
        let lease = Arc::clone(&context.bytes)
            .try_acquire_many_owned(128 * 1024)
            .map_err(|_| rpc_error(-32005, "pending state capacity exceeded"))?;
        return Ok(SelectedState::Owned {
            commit: super::prepare_pending_state(context)?,
            _lease: Some(lease),
        });
    }
    if selector.as_str() == Some("latest") {
        return super::capture_current_rpc_state(context);
    }
    let (_, reader) = durable_rpc_source(context)?;
    let history = capture_history_snapshot(reader, context.history_budget)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let height = resolve_height(&history, selector)?;
    if height > history.version().height {
        return Err(rpc_error(-32001, "UNKNOWN_BLOCK"));
    }
    {
        let cache = context
            .historical_cache
            .lock()
            .map_err(|_| rpc_error(-32603, "history cache poisoned"))?;
        if let Some(commit) = cache.get(&height) {
            return Ok(SelectedState::Owned {
                commit: Arc::clone(commit),
                _lease: None,
            });
        }
    }
    drop(history);
    let lease = Arc::clone(&context.bytes)
        .try_acquire_many_owned(128 * 1024)
        .map_err(|_| rpc_error(-32005, "historical state load capacity exceeded"))?;
    let snapshot = capture_state_snapshot(reader).map_err(|e| rpc_error(-32000, e.to_string()))?;
    let commit = Arc::new(
        read_snapshot_commit(&snapshot, height)
            .map_err(|e| rpc_error(-32000, e.to_string()))?
            .ok_or_else(|| rpc_error(-32001, "UNKNOWN_BLOCK"))?,
    );
    let cached_bytes =
        eve_state::measure_complete_state_bytes(&commit.state, &context.state_budget)
            .map_err(|e| rpc_error(-32000, format!("historical state size: {e:?}")))?
            .saturating_add(
                commit
                    .block
                    .transactions
                    .iter()
                    .chain(&commit.block.receipts)
                    .map(|bytes| bytes.len())
                    .sum::<usize>(),
            );
    if cached_bytes <= 64 * 1_048_576 {
        let mut cache = context
            .historical_cache
            .lock()
            .map_err(|_| rpc_error(-32603, "history cache poisoned"))?;
        if cache.len() == 2
            && let Some(first) = cache.keys().next().copied()
        {
            cache.remove(&first);
        }
        cache.insert(height, Arc::clone(&commit));
    }
    Ok(SelectedState::Owned {
        commit,
        _lease: Some(lease),
    })
}
