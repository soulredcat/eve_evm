use alloy_primitives::{B256, keccak256};

use super::{DevelopmentGenesis, GenesisError, encode_development_genesis};
use crate::network::LaunchMode;

pub fn hash_development_genesis(
    mode: LaunchMode,
    genesis: &DevelopmentGenesis,
) -> Result<B256, GenesisError> {
    Ok(keccak256(encode_development_genesis(mode, genesis)?))
}
