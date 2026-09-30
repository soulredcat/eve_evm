use crate::{
    AddressBytes, AssetKind, InteropError, RouteManifest, encode_chain_identity,
    validate_route_manifest,
};

/// Canonical immutable binding bytes. Status/disclosure is deliberately not an approval signature.
pub fn encode_route_binding(route: &RouteManifest) -> Result<Vec<u8>, InteropError> {
    validate_route_manifest(route)?;
    let mut bytes = b"eve-route-binding-v1\0".to_vec();
    bytes.extend_from_slice(&route.version.to_be_bytes());
    bytes.extend_from_slice(&route.route_id);
    bytes.extend_from_slice(&encode_chain_identity(&route.source)?);
    bytes.extend_from_slice(&encode_chain_identity(&route.destination)?);
    for custody in [route.source_custody, route.destination_custody] {
        match custody.bytes {
            AddressBytes::Evm(address) => {
                bytes.push(20);
                bytes.extend_from_slice(&address);
            }
            AddressBytes::Solana(address) => {
                bytes.push(32);
                bytes.extend_from_slice(&address);
            }
        }
    }
    bytes.extend_from_slice(&encode_chain_identity(&route.asset.chain)?);
    match route.asset.kind {
        AssetKind::Native => bytes.push(0),
        AssetKind::Erc20 { contract, .. } => {
            bytes.push(1);
            if let AddressBytes::Evm(address) = contract.bytes {
                bytes.extend_from_slice(&address);
            }
        }
        AssetKind::SplToken {
            mint,
            token_program,
            ..
        } => {
            bytes.push(2);
            if let AddressBytes::Solana(address) = mint.bytes {
                bytes.extend_from_slice(&address);
            }
            if let AddressBytes::Solana(address) = token_program.bytes {
                bytes.extend_from_slice(&address);
            }
        }
    }
    bytes.push(route.source_decimals);
    bytes.push(route.destination_decimals);
    bytes.extend_from_slice(&route.maximum_source_amount.to_be_bytes::<32>());
    bytes.extend_from_slice(&route.maximum_destination_amount.to_be_bytes::<32>());
    bytes.push(route.verification as u8);
    bytes.push(route.crypto_coverage as u8);
    Ok(bytes)
}
