// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportedState;
use super::{CheckpointError, CheckpointLimits};
use eve_state::{StateBudget, StateCommit, StateJournal, estimate_journal_candidate_reservation};

/// Conservative logical full-state/root/encoding and bounded witness-copy envelope.
/// Caller owns decoded input and bounded canonical sizing scratch BEFORE this operation.
pub fn required_checkpoint_reservation(
    parent: &ImportedState,
    target: &StateCommit,
    budget: &StateBudget,
    limits: CheckpointLimits,
) -> Result<usize, CheckpointError> {
    if limits.maximum_height_gap == 0
        || limits.maximum_witness_bytes == 0
        || target.target.height <= parent.commit.target.height
    {
        return Err(CheckpointError::InvalidLimit);
    }
    let empty = StateJournal {
        parent: target.target.clone(),
        target_height: target
            .target
            .height
            .checked_add(1)
            .ok_or(CheckpointError::Overflow)?,
        operations: Vec::new(),
    };
    let state = estimate_journal_candidate_reservation(&target.state, &empty, budget)
        .map_err(CheckpointError::State)?;
    let raw = target
        .block
        .transactions
        .iter()
        .chain(&target.block.receipts)
        .try_fold(0_usize, |sum, bytes| sum.checked_add(bytes.len()))
        .ok_or(CheckpointError::Overflow)?;
    state
        .checked_add(raw.checked_mul(2).ok_or(CheckpointError::Overflow)?)
        .and_then(|bytes| bytes.checked_add(limits.maximum_witness_bytes.checked_mul(4)?))
        .and_then(|bytes| {
            bytes.checked_add(std::mem::size_of::<super::CheckpointSession>() + 65_536)
        })
        .ok_or(CheckpointError::Overflow)
}
