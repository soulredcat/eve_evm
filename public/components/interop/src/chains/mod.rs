//! Chain identity and address namespace contracts.
mod addresses;
mod identity;

pub use addresses::{
    AddressBytes, ChainAddress, decode_chain_address, format_chain_address, parse_chain_address,
    validate_chain_address,
};
pub use identity::{ChainIdentity, ChainNamespace, encode_chain_identity, validate_chain_identity};
