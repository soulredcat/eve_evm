// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportedState;
use super::{
    CheckpointError, CheckpointLimits, CheckpointSession, required_checkpoint_reservation,
};
use eve_state::{StateBudget, StateCommit, validate_state_commit};
use std::sync::Arc;

/// The actual private imported parent supplies genesis/set/history trust, never a peer root.
pub fn begin_authenticated_checkpoint(
    parent: &ImportedState,
    target: Arc<StateCommit>,
    budget: &StateBudget,
    limits: CheckpointLimits,
    reserved_bytes: usize,
) -> Result<CheckpointSession, CheckpointError> {
    let required = required_checkpoint_reservation(parent, &target, budget, limits)?;
    if reserved_bytes < required {
        return Err(CheckpointError::ReservationTooSmall);
    }
    if target.target.identity != parent.commit.target.identity {
        return Err(CheckpointError::WrongIdentity);
    }
    let gap = target
        .target
        .height
        .checked_sub(parent.commit.target.height)
        .ok_or(CheckpointError::WrongHeight)?;
    if gap == 0 || gap > limits.maximum_height_gap {
        return Err(CheckpointError::WrongHeight);
    }
    target
        .target
        .height
        .checked_add(1)
        .ok_or(CheckpointError::Overflow)?;
    validate_state_commit(&target, budget).map_err(CheckpointError::State)?;
    let base = parent.commit.target.height;
    for (height, hash) in &parent.commit.state.block_hashes {
        if *height <= base && target.state.block_hashes.get(height) != Some(hash) {
            return Err(CheckpointError::WrongHistory);
        }
    }
    for (height, hash) in &target.state.block_hashes {
        if *height <= base && parent.commit.state.block_hashes.get(height) != Some(hash) {
            return Err(CheckpointError::WrongHistory);
        }
    }
    for height in base.checked_add(1).ok_or(CheckpointError::Overflow)?..=target.target.height {
        if !target.state.block_hashes.contains_key(&height) {
            return Err(CheckpointError::WrongHistory);
        }
    }
    Ok(CheckpointSession {
        target,
        budget: *budget,
        limits,
        required,
        base_height: base,
        next_height: base.checked_add(1).ok_or(CheckpointError::Overflow)?,
        previous_version: parent.commit.target.clone(),
        previous_header: parent.commit.block.header.clone(),
        finality: parent.finality.clone(),
        policy: Arc::clone(&parent.policy),
        seed_data: parent.lookahead.clone(),
        seed_header: parent.lookahead_header.clone(),
        closing_data: None,
        closing_header: None,
    })
}
