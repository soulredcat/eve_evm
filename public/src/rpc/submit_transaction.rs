// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcContext,
    encoding::{parse_data, require_arity},
    errors::rpc_error,
    run_signature_validation::run_signature_validation,
    worker_types::SignatureLeases,
};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
pub(crate) async fn submit_transaction(
    context: Arc<RpcContext>,
    params: Vec<Value>,
    active: OwnedSemaphorePermit,
) -> Result<Value, ErrorObjectOwned> {
    require_arity(&params, 1, 1)?;
    let raw = parse_data(&params[0], 131_072)?;
    let signature = Arc::clone(&context.signatures)
        .try_acquire_owned()
        .map_err(|_| rpc_error(-32005, "signature worker capacity exceeded"))?;
    let charge = u32::try_from(
        raw.len()
            .checked_mul(3)
            .and_then(|bytes| bytes.checked_add(4096))
            .ok_or_else(|| rpc_error(-32005, "signature byte accounting overflow"))?
            .div_ceil(1024),
    )
    .map_err(|_| rpc_error(-32005, "signature reservation overflow"))?;
    let bytes = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "signature byte capacity exceeded"))?;
    let shared = Arc::clone(&context);
    let input = raw.clone();
    let leases = SignatureLeases {
        active,
        signature,
        bytes,
    };
    let (validated, active) =
        tokio::task::spawn_blocking(move || run_signature_validation(shared, input, leases))
            .await
            .map_err(|_| rpc_error(-32603, "signature worker failed"))??;
    let _active = active;
    let hash = context
        .pool
        .admit(raw, validated)
        .await
        .map_err(|e| rpc_error(-32000, e.0))?;
    serde_json::to_value(hash).map_err(|e| rpc_error(-32603, e.to_string()))
}
