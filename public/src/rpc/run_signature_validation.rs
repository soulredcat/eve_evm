// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RpcContext, errors::rpc_error, worker_types::SignatureLeases};
use alloy_primitives::Bytes;
use eve_evm::ValidatedTransaction;
use eve_storage::state::{read_cached_state_service, read_state_service};
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
pub(crate) fn run_signature_validation(
    context: Arc<RpcContext>,
    raw: Bytes,
    leases: SignatureLeases,
) -> Result<(ValidatedTransaction, OwnedSemaphorePermit), ErrorObjectOwned> {
    let cached = read_cached_state_service(&context.service)
        .map_err(|e| rpc_error(-32000, e.to_string()))?;
    let (head, _refresh) = if let Some(head) = cached {
        (head, None)
    } else {
        let lease = Arc::clone(&context.bytes)
            .try_acquire_many_owned(128 * 1024)
            .map_err(|_| rpc_error(-32005, "signature state refresh capacity exceeded"))?;
        (
            read_state_service(&context.service).map_err(|e| rpc_error(-32000, e.to_string()))?,
            Some(lease),
        )
    };
    let validated = eve_evm::decode_signed_transaction(
        &raw,
        head.commit().target.identity.evm_chain_id,
        131_072,
    )
    .map_err(|e| rpc_error(-32000, format!("invalid signed transaction: {e:?}")))?;
    let _leases = (leases.signature, leases.bytes);
    Ok((validated, leases.active))
}
