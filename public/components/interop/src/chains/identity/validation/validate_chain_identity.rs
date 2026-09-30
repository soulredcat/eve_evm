use crate::{ChainIdentity, ChainNamespace, InteropError, U256};

/// Validate identity metadata without treating a supplied genesis as a trust anchor.
pub fn validate_chain_identity(identity: &ChainIdentity) -> Result<(), InteropError> {
    if identity.genesis == [0; 32] {
        return Err(InteropError::InvalidGenesis);
    }
    match (identity.namespace, identity.numeric_id) {
        (ChainNamespace::Eve | ChainNamespace::Ethereum, Some(id)) if id != U256::ZERO => Ok(()),
        (ChainNamespace::Solana, None) => Ok(()),
        _ => Err(InteropError::InvalidNumericChainId),
    }
}
