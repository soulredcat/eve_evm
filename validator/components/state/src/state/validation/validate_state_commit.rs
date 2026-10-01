// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{build_state_version, validate_retained_block, validate_state_version};
use crate::{StateBudget, StateCommit, StateError};

pub fn validate_state_commit(commit: &StateCommit, budget: &StateBudget) -> Result<(), StateError> {
    validate_state_version(&commit.state, &commit.target, budget)?;
    validate_retained_block(
        &commit.target,
        commit.parent.as_ref(),
        &commit.block,
        budget,
    )?;
    if build_state_version(&commit.state, &commit.block.header, budget)? != commit.target {
        return Err(StateError::CommitMismatch);
    }
    if let Some(parent) = &commit.parent
        && commit.state.block_hashes.get(&parent.height) != Some(&parent.execution_hash)
    {
        return Err(StateError::ParentMismatch);
    }
    Ok(())
}
