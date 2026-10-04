// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{RpcContext, RpcStateSource, errors::rpc_error};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, StateCommit, StateJournal,
    estimate_journal_candidate_reservation,
};
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;

/// The proof view clones both EVM and system domains; applied sources charge the
/// canonical complete-candidate model, with no journal mutation or execution.
pub(super) fn reserve_proof_state_clone(
    context: &RpcContext,
    state: &StateCommit,
) -> Result<OwnedSemaphorePermit, ErrorObjectOwned> {
    let reservation = if matches!(&context.source, RpcStateSource::Applied { .. }) {
        let _sizing = Arc::clone(&context.bytes)
            .try_acquire_many_owned(
                u32::try_from(BOUNDED_STATE_CODEC_SCRATCH_BYTES.div_ceil(1024))
                    .map_err(|_| rpc_error(-32005, "proof sizing accounting overflow"))?,
            )
            .map_err(|_| rpc_error(-32005, "proof sizing capacity exceeded"))?;
        let journal = StateJournal {
            parent: state.target.clone(),
            target_height: state
                .target
                .height
                .checked_add(1)
                .ok_or_else(|| rpc_error(-32005, "proof height overflow"))?,
            operations: Vec::new(),
        };
        estimate_journal_candidate_reservation(&state.state, &journal, &context.state_budget)
            .map_err(|error| rpc_error(-32005, format!("proof clone reservation: {error:?}")))?
    } else {
        eve_evm::estimate_clone_reservation(&state.state)
            .map_err(|error| rpc_error(-32005, format!("proof clone reservation: {error:?}")))?
    };
    let charge = u32::try_from(reservation.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "proof clone accounting overflow"))?;
    Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "proof clone capacity exceeded"))
}
