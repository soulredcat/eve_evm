use crate::{StateCommit, StateError};
use eve_protocol_config::records::ExecutionBlockHash;
use std::collections::BTreeMap;

/// Compare local material to caller-selected retained header hashes; this does
/// not authenticate those headers or establish consensus finality.
pub fn validate_block_hash_history(
    commit: &StateCommit,
    authoritative: &BTreeMap<u64, ExecutionBlockHash>,
) -> Result<(), StateError> {
    let height = commit.target.height;
    for required in height.saturating_sub(256)..height {
        if !commit.state.block_hashes.contains_key(&required) {
            return Err(StateError::MissingHistory(required));
        }
    }
    for (entry_height, hash) in &commit.state.block_hashes {
        if *entry_height > height {
            return Err(StateError::VersionMismatch);
        }
        if *entry_height == height {
            if *hash != commit.target.execution_hash {
                return Err(StateError::CommitMismatch);
            }
        } else if authoritative.get(entry_height) != Some(hash) {
            return Err(StateError::MissingHistory(*entry_height));
        }
    }
    Ok(())
}
