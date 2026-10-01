// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{validate_complete_state, validate_version_metadata};
use crate::{
    CompleteState, StateBudget, StateError, StateVersion, compute_evm_root,
    compute_state_content_digest, compute_system_root,
};

pub fn validate_state_version(
    state: &CompleteState,
    version: &StateVersion,
    budget: &StateBudget,
) -> Result<(), StateError> {
    validate_version_metadata(version)?;
    validate_complete_state(state, budget)?;
    for height in version.height.saturating_sub(256)..version.height {
        if !state.block_hashes.contains_key(&height) {
            return Err(StateError::MissingHistory(height));
        }
    }
    if state.identity != version.identity
        || compute_evm_root(&state.accounts) != version.evm_root
        || compute_system_root(&state.system)? != version.system_root
        || state.block_hashes.get(&version.height) != Some(&version.execution_hash)
        || state
            .block_hashes
            .keys()
            .any(|height| *height > version.height)
    {
        return Err(StateError::VersionMismatch);
    }
    if compute_state_content_digest(state, budget)? != version.content_digest {
        return Err(StateError::VersionMismatch);
    }
    Ok(())
}
