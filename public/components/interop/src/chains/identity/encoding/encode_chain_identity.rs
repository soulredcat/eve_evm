use crate::{ChainIdentity, InteropError, validate_chain_identity};

/// Version-one bytes: domain, namespace, genesis, optional full-width EVM chain ID.
pub fn encode_chain_identity(identity: &ChainIdentity) -> Result<Vec<u8>, InteropError> {
    validate_chain_identity(identity)?;
    let mut bytes = b"eve-chain-v1\0".to_vec();
    bytes.push(identity.namespace as u8);
    bytes.extend_from_slice(&identity.genesis);
    bytes.push(u8::from(identity.numeric_id.is_some()));
    if let Some(id) = identity.numeric_id {
        bytes.extend_from_slice(&id.to_be_bytes::<32>());
    }
    Ok(bytes)
}
