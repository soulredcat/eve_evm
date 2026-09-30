use crate::{AddressBytes, ChainAddress, ChainIdentity, ChainNamespace, InteropError};

pub fn decode_chain_address(
    chain: ChainIdentity,
    bytes: &[u8],
) -> Result<ChainAddress, InteropError> {
    crate::validate_chain_identity(&chain)?;
    let bytes = match chain.namespace {
        ChainNamespace::Eve | ChainNamespace::Ethereum => AddressBytes::Evm(
            bytes
                .try_into()
                .map_err(|_| InteropError::InvalidAddressWidth)?,
        ),
        ChainNamespace::Solana => AddressBytes::Solana(
            bytes
                .try_into()
                .map_err(|_| InteropError::InvalidAddressWidth)?,
        ),
    };
    Ok(ChainAddress { chain, bytes })
}
