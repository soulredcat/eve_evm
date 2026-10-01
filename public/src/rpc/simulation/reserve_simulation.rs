// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{RpcContext, errors::rpc_error};
use eve_evm::estimate_clone_reservation;
use eve_state::StateCommit;
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
pub(crate) fn reserve_simulation(
    context: &RpcContext,
    state: &StateCommit,
) -> Result<(usize, OwnedSemaphorePermit), ErrorObjectOwned> {
    let reservation = estimate_clone_reservation(&state.state)
        .map_err(|e| rpc_error(-32000, format!("clone reservation: {e:?}")))?;
    let charge = reservation
        .checked_add(32 * 1_048_576)
        .and_then(|bytes| u32::try_from(bytes.div_ceil(1024)).ok())
        .ok_or_else(|| rpc_error(-32005, "simulation byte accounting overflow"))?;
    let permit = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "simulation byte capacity exceeded"))?;
    Ok((reservation, permit))
}
