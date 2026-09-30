use crate::wire::tendermint::types::Header;

use super::{ExecutionHeight, HeaderCommitmentMatch, HeightMappingError, map_finalize_height};

/// Check an H post-state commitment against the H+1 header's application hash.
///
/// This validates only the binding. It does not verify a commit certificate,
/// validator-set history, trust freshness, or the execution that produced the root.
pub fn match_next_header_commitment(
    expected_chain: &str,
    execution_height: ExecutionHeight,
    commitment: &[u8; 32],
    header: &Header,
) -> Result<HeaderCommitmentMatch, HeightMappingError> {
    let mapping = map_finalize_height(execution_height)?;
    if header.chain_id != expected_chain {
        return Err(HeightMappingError::WrongChain);
    }
    if header.height != mapping.app_hash_header_height {
        return Err(HeightMappingError::WrongHeaderHeight);
    }
    if header.app_hash.as_slice() != commitment {
        return Err(HeightMappingError::WrongApplicationHash);
    }
    Ok(HeaderCommitmentMatch {
        execution_height,
        consensus_header_height: header.height,
    })
}
