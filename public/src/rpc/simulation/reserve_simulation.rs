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
    // Applied profiles include the VM arena and its possible copied RETURN/REVERT
    // output before the RPC response limit is checked. Preserve the legacy profile.
    let memory = usize::try_from(context.simulation_memory_bytes)
        .map_err(|_| rpc_error(-32005, "simulation memory accounting overflow"))?;
    let memory = if matches!(&context.source, crate::rpc::RpcStateSource::Applied { .. }) {
        memory
            .checked_mul(2)
            .ok_or_else(|| rpc_error(-32005, "simulation output accounting overflow"))?
    } else {
        memory
    };
    let charge = reservation
        .checked_add(memory)
        .and_then(|bytes| u32::try_from(bytes.div_ceil(1024)).ok())
        .ok_or_else(|| rpc_error(-32005, "simulation byte accounting overflow"))?;
    let permit = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "simulation byte capacity exceeded"))?;
    Ok((reservation, permit))
}
