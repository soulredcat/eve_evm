use crate::{AddressBytes, ChainAddress, ChainNamespace, InteropError, validate_chain_identity};

pub fn validate_chain_address(address: &ChainAddress) -> Result<(), InteropError> {
    validate_chain_identity(&address.chain)?;
    match (address.chain.namespace, address.bytes) {
        (ChainNamespace::Eve | ChainNamespace::Ethereum, AddressBytes::Evm(_))
        | (ChainNamespace::Solana, AddressBytes::Solana(_)) => Ok(()),
        _ => Err(InteropError::AddressNamespaceMismatch),
    }
}
