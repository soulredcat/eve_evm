// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SelectedState;
use crate::rpc::{RpcContext, RpcStateSource, errors::rpc_error};
use crate::sync::applied::capture_applied_state;
use eve_storage::state::{read_cached_state_service, read_state_service};
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;

/// Capture one real immutable source. The RAM publication keeps its generation charge.
pub(crate) fn capture_current_rpc_state(
    context: &RpcContext,
) -> Result<SelectedState, ErrorObjectOwned> {
    match &context.source {
        RpcStateSource::Applied { reader } => Ok(SelectedState::Applied {
            publication: capture_applied_state(reader)
                .map_err(|_| rpc_error(-32001, "NOT_READY: applied publication unavailable"))?,
        }),
        RpcStateSource::Durable { service, .. } => {
            if let Some(view) = read_cached_state_service(service)
                .map_err(|error| rpc_error(-32000, error.to_string()))?
            {
                return Ok(SelectedState::Current { view, _lease: None });
            }
            let lease = Arc::clone(&context.bytes)
                .try_acquire_many_owned(128 * 1024)
                .map_err(|_| rpc_error(-32005, "state cache refresh capacity exceeded"))?;
            let view = read_state_service(service)
                .map_err(|error| rpc_error(-32000, error.to_string()))?;
            Ok(SelectedState::Current {
                view,
                _lease: Some(lease),
            })
        }
    }
}
