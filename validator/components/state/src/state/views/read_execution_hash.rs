use super::StateView;
use crate::StateError;
use eve_protocol_config::records::ExecutionBlockHash;

pub fn read_execution_hash(
    view: &StateView,
    execution_height: u64,
    requested_height: u64,
) -> Result<Option<ExecutionBlockHash>, StateError> {
    if requested_height >= execution_height || execution_height - requested_height > 256 {
        return Ok(None);
    }
    view.state
        .block_hashes
        .get(&requested_height)
        .copied()
        .map(Some)
        .ok_or(StateError::MissingHistory(requested_height))
}
