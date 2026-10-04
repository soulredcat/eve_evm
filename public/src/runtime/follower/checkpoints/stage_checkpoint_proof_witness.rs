// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    check_checkpoint_bootstrap_deadline::check_checkpoint_bootstrap_deadline,
    checkpoint_rpc_before::checkpoint_rpc_before,
    reserve_checkpoint_ingress::reserve_checkpoint_ingress, types::CheckpointDownloadContext,
};
use crate::sync::applied::checkpoints::{
    ChargedCheckpointProofTransfer, CheckpointAppliedError, observe_applied_checkpoint_witness,
    repair_applied_checkpoint_witness, write_applied_checkpoint_witness,
};
use anyhow::{Context, Result};
use eve_storage::checkpoints::{CheckpointChunkStatus, CheckpointError};
use eve_sync_client::{
    CheckpointWitnessHeights, downloaded_checkpoint_witness_wire, fetch_checkpoint_witness_before,
};

pub(super) fn stage_checkpoint_proof_witness(
    transfer: &mut ChargedCheckpointProofTransfer,
    height: u64,
    index: usize,
    context: &CheckpointDownloadContext<'_>,
) -> Result<()> {
    check_checkpoint_bootstrap_deadline(context.deadline)?;
    match observe_applied_checkpoint_witness(transfer, index) {
        Ok(CheckpointChunkStatus::Present) => {
            return check_checkpoint_bootstrap_deadline(context.deadline);
        }
        Ok(CheckpointChunkStatus::Missing) => {}
        Ok(CheckpointChunkStatus::Corrupt)
        | Err(CheckpointAppliedError::Storage(CheckpointError::UnsafeEntry)) => {
            repair_applied_checkpoint_witness(transfer, index)
                .map_err(|error| anyhow::anyhow!("checkpoint witness repair: {error:?}"))?;
            check_checkpoint_bootstrap_deadline(context.deadline)?;
        }
        Err(error) => anyhow::bail!("checkpoint witness observation: {error:?}"),
    }
    let downloaded = fetch_checkpoint_witness_before(
        checkpoint_rpc_before(context.address, context.deadline)?,
        context.genesis,
        CheckpointWitnessHeights {
            checkpoint: height,
            witness: u64::try_from(index)?
                .checked_add(1)
                .context("checkpoint witness height overflow")?,
        },
        &context.limits.content.logical,
        context.limits.verification,
        &mut |bytes| reserve_checkpoint_ingress(context.reader, bytes),
        context.deadline,
    )?;
    write_applied_checkpoint_witness(
        transfer,
        index,
        downloaded_checkpoint_witness_wire(&downloaded),
    )
    .map_err(|error| anyhow::anyhow!("checkpoint witness source binding or sync: {error:?}"))?;
    check_checkpoint_bootstrap_deadline(context.deadline)
}
