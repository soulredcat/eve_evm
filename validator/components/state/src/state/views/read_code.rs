use super::StateView;
use crate::StateError;
use alloy_primitives::{B256, Bytes};

pub fn read_code(view: &StateView, code_hash: B256) -> Result<Bytes, StateError> {
    if code_hash == alloy_trie::KECCAK_EMPTY {
        return Ok(Bytes::new());
    }
    view.state
        .codes
        .get(&code_hash)
        .cloned()
        .ok_or(StateError::MissingCode(code_hash))
}
