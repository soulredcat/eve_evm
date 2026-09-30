use super::{CompleteExecutionError, project_revm_accounts};
use alloy_primitives::{B256, Bytes, keccak256};
use eve_state::{CompleteState, StateBudget, StateError, validate_complete_state};
use revm::database::InMemoryDB;

pub fn from_revm_state(
    parent: &CompleteState,
    cache: &InMemoryDB,
    budget: &StateBudget,
) -> Result<CompleteState, CompleteExecutionError> {
    let mut state = CompleteState {
        identity: parent.identity.clone(),
        accounts: project_revm_accounts(cache),
        codes: parent.codes.clone(),
        system: parent.system.clone(),
        block_hashes: parent.block_hashes.clone(),
    };
    for (hash, code) in &cache.cache.contracts {
        if *hash == B256::ZERO || *hash == alloy_trie::KECCAK_EMPTY {
            continue;
        }
        let bytes = Bytes::copy_from_slice(&code.original_bytes());
        if keccak256(&bytes) != *hash {
            return Err(CompleteExecutionError::State(StateError::CodeHashMismatch(
                *hash,
            )));
        }
        state.codes.insert(*hash, bytes);
    }
    validate_complete_state(&state, budget).map_err(CompleteExecutionError::State)?;
    Ok(state)
}
