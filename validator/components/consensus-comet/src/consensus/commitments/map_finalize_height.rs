use super::{CommitmentHeightMapping, ExecutionHeight, HeightMappingError};

/// Map `FinalizeBlock(H)` outputs to the pinned engine's later header metadata.
pub fn map_finalize_height(
    height: ExecutionHeight,
) -> Result<CommitmentHeightMapping, HeightMappingError> {
    if height.0 <= 0 {
        return Err(HeightMappingError::NonPositiveExecutionHeight);
    }
    let last_commit = height
        .0
        .checked_add(3)
        .ok_or(HeightMappingError::HeightOverflow)?;
    Ok(CommitmentHeightMapping {
        execution_height: height,
        app_hash_header_height: height.0 + 1,
        update_next_validators_hash_height: height.0 + 1,
        updated_validator_set_height: height.0 + 2,
        updated_last_commit_metadata_height: last_commit,
    })
}
