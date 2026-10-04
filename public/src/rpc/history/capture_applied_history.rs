// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{RpcContext, RpcStateSource, errors::rpc_error};
use crate::sync::applied::{AppliedPublication, capture_applied_state};
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;

pub(crate) fn capture_applied_history(
    context: &RpcContext,
) -> Result<Arc<AppliedPublication>, ErrorObjectOwned> {
    match &context.source {
        RpcStateSource::Applied { reader } => capture_applied_state(reader)
            .map_err(|_| rpc_error(-32001, "NOT_READY: applied publication unavailable")),
        _ => Err(rpc_error(-32603, "applied history source mismatch")),
    }
}
