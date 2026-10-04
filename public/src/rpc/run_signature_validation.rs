// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RpcContext, errors::rpc_error, worker_types::SignatureLeases};
use alloy_primitives::Bytes;
use eve_evm::ValidatedTransaction;
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
pub(crate) fn run_signature_validation(
    context: Arc<RpcContext>,
    raw: Bytes,
    leases: SignatureLeases,
) -> Result<(ValidatedTransaction, OwnedSemaphorePermit), ErrorObjectOwned> {
    let head = super::selectors::capture_current_rpc_state(&context)?;
    let validated = eve_evm::decode_signed_transaction(
        &raw,
        head.commit().target.identity.evm_chain_id,
        131_072,
    )
    .map_err(|e| rpc_error(-32000, format!("invalid signed transaction: {e:?}")))?;
    let _leases = (leases.signature, leases.bytes);
    Ok((validated, leases.active))
}
