// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{build_state_version, validate_state_commit};
use crate::{BlockPayload, CompleteState, StateBudget, StateCommit, StateError, StateVersion};

pub fn build_state_commit(
    parent: Option<StateVersion>,
    mut state: CompleteState,
    block: BlockPayload,
    budget: &StateBudget,
) -> Result<StateCommit, StateError> {
    let hash = eve_protocol_config::records::ExecutionBlockHash(block.header.hash_slow());
    // The target entry is derived candidate metadata; past entries are untouched.
    state.block_hashes.insert(block.header.number, hash);
    let target = build_state_version(&state, &block.header, budget)?;
    let commit = StateCommit {
        parent,
        target,
        state,
        block,
    };
    validate_state_commit(&commit, budget)?;
    Ok(commit)
}
