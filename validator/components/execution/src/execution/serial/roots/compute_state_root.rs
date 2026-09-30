use alloy_primitives::B256;
use revm::database::InMemoryDB;

/// Root of a complete EmptyDB-backed state, never a partial remote-state cache.
pub fn compute_state_root(state: &InMemoryDB) -> B256 {
    eve_state::compute_evm_root(&crate::state::project_revm_accounts(state)).0
}
