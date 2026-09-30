use crate::{AddressBytes, ChainAddress, InteropError, validate_chain_address};

/// Machine-canonical EVM display is lower-case hex; Solana display is base58.
pub fn format_chain_address(address: &ChainAddress) -> Result<String, InteropError> {
    validate_chain_address(address)?;
    Ok(match address.bytes {
        AddressBytes::Evm(bytes) => format!("0x{}", hex::encode(bytes)),
        AddressBytes::Solana(bytes) => bs58::encode(bytes).into_string(),
    })
}
