// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{resolve_height, types::SelectedState};
use crate::rpc::{RpcContext, errors::rpc_error};
use eve_storage::state::{
    capture_history_snapshot, capture_state_snapshot, read_cached_state_service,
    read_snapshot_commit, read_state_service,
};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::Arc;
pub(crate) fn capture_selected_state(
    context: &RpcContext,
    selector: &Value,
) -> Result<SelectedState, ErrorObjectOwned> {
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
        if let Some(view) = read_cached_state_service(&context.service)
            .map_err(|e| rpc_error(-32000, e.to_string()))?
        {
            return Ok(SelectedState::Current { view, _lease: None });
        }
        let lease = Arc::clone(&context.bytes)
            .try_acquire_many_owned(128 * 1024)
            .map_err(|_| rpc_error(-32005, "state cache refresh capacity exceeded"))?;
        let view =
            read_state_service(&context.service).map_err(|e| rpc_error(-32000, e.to_string()))?;
        return Ok(SelectedState::Current {
            view,
            _lease: Some(lease),
        });
    }
    let history = capture_history_snapshot(&context.reader, context.history_budget)
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
    let snapshot =
        capture_state_snapshot(&context.reader).map_err(|e| rpc_error(-32000, e.to_string()))?;
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
