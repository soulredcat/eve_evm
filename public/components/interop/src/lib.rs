//! Public interoperability metadata and bounded admission contracts.
//! No metadata check authenticates source finality or authorizes custody effects.
#![forbid(unsafe_code)]

mod assets;
mod chains;
mod routes;

pub use alloy_primitives::U256;
pub use assets::{
    AssetKind, AssetOrigin, TokenBehavior, convert_exact_amount, validate_asset_origin,
};
pub use chains::{
    AddressBytes, ChainAddress, ChainIdentity, ChainNamespace, decode_chain_address,
    encode_chain_identity, format_chain_address, parse_chain_address, validate_chain_address,
    validate_chain_identity,
};
pub use routes::{
    CryptoCoverage, InteropError, RouteManifest, RouteState, TransferRequest,
    VerificationRequirement, encode_route_binding, find_route_manifest, validate_route_admission,
    validate_route_manifest, validate_transfer_request,
};
