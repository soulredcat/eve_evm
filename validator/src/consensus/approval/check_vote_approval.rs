// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ExecutionApproval;
use crate::consensus::signing::DurableSigner;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::types::Vote;
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn check_vote_approval(
    approval: &ExecutionApproval,
    signer: &DurableSigner,
    vote: &Vote,
) -> Result<()> {
    ensure!(
        approval.config == signer.config && approval.request.height == vote.height,
        "execution approval identity/height mismatch"
    );
    let block = vote
        .block_id
        .as_ref()
        .context("non-nil vote block missing")?;
    ensure!(
        block.hash.as_slice() == approval.consensus_hash,
        "vote block differs from executed proposal"
    );
    let current = read_state_service(&signer.service)?;
    ensure!(
        current.sequence() == approval.database_sequence
            && current.commit().target == approval.parent,
        "execution approval parent is stale"
    );
    ensure!(
        approval.prepared.commit.parent.as_ref() == Some(&approval.parent),
        "execution approval prepared base mismatch"
    );
    Ok(())
}
