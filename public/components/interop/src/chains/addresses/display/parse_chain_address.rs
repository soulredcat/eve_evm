use crate::{ChainAddress, ChainIdentity, ChainNamespace, InteropError, decode_chain_address};

/// Parse only the canonical machine encoding, with length bounds before allocation.
pub fn parse_chain_address(
    chain: ChainIdentity,
    display: &str,
) -> Result<ChainAddress, InteropError> {
    let bytes = match chain.namespace {
        ChainNamespace::Eve | ChainNamespace::Ethereum => {
            if display.len() != 42 || !display.starts_with("0x") {
                return Err(InteropError::InvalidAddressEncoding);
            }
            hex::decode(&display[2..]).map_err(|_| InteropError::InvalidAddressEncoding)?
        }
        ChainNamespace::Solana => {
            if !(32..=44).contains(&display.len()) {
                return Err(InteropError::InvalidAddressEncoding);
            }
            bs58::decode(display)
                .into_vec()
                .map_err(|_| InteropError::InvalidAddressEncoding)?
        }
    };
    let address = decode_chain_address(chain, &bytes)?;
    if crate::format_chain_address(&address)? != display {
        return Err(InteropError::NonCanonicalAddress);
    }
    Ok(address)
}
