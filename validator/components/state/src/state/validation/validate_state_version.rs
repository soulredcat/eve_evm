use eve_protocol_config::records::{ApplicationCommitmentInput, hash_application_commitment};

use super::validate_complete_state;
use crate::{
    CompleteState, StateBudget, StateError, StateVersion, compute_evm_root,
    compute_state_content_digest, compute_system_root,
};

pub fn validate_state_version(
    state: &CompleteState,
    version: &StateVersion,
    budget: &StateBudget,
) -> Result<(), StateError> {
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
    let application = if version.height == 0 {
        None
    } else {
        Some(
            hash_application_commitment(ApplicationCommitmentInput {
                genesis: version.identity.genesis,
                protocol_version: version.identity.protocol_version,
                execution_height: version.height,
                evm_root: version.evm_root,
                system_root: version.system_root,
                execution_hash: version.execution_hash,
            })
            .map_err(StateError::SystemRecord)?,
        )
    };
    if application != version.application
        || compute_state_content_digest(state, budget)? != version.content_digest
    {
        return Err(StateError::VersionMismatch);
    }
    Ok(())
}
