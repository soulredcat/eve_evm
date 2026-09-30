use crate::{
    AddressBytes, ChainNamespace, InteropError, RouteManifest, U256, VerificationRequirement,
    validate_asset_origin, validate_chain_address, validate_chain_identity,
};

/// Metadata validity is independent of finality verification and deployment approval.
pub fn validate_route_manifest(route: &RouteManifest) -> Result<(), InteropError> {
    if route.version != 1 {
        return Err(InteropError::UnsupportedManifestVersion);
    }
    validate_chain_identity(&route.source)?;
    validate_chain_identity(&route.destination)?;
    if route.route_id == [0; 32] || route.source == route.destination {
        return Err(InteropError::InvalidRouteIdentity);
    }
    validate_chain_address(&route.source_custody)?;
    validate_chain_address(&route.destination_custody)?;
    if route.source_custody.chain != route.source
        || route.destination_custody.chain != route.destination
    {
        return Err(InteropError::CustodyNamespaceMismatch);
    }
    if [route.source_custody, route.destination_custody]
        .iter()
        .any(|custody| {
            custody.bytes == AddressBytes::Evm([0; 20])
                || custody.bytes == AddressBytes::Solana([0; 32])
        })
    {
        return Err(InteropError::InvalidCustodyAddress);
    }
    validate_asset_origin(&route.asset)?;
    if route.asset.chain != route.source && route.asset.chain != route.destination {
        return Err(InteropError::UnsupportedAssetForwarding);
    }
    if route.source_decimals > 77 || route.destination_decimals > 77 {
        return Err(InteropError::InvalidDecimals);
    }
    if route.maximum_source_amount == U256::ZERO || route.maximum_destination_amount == U256::ZERO {
        return Err(InteropError::ZeroAmount);
    }
    if (route.source.namespace == ChainNamespace::Solana
        && route.maximum_source_amount > U256::from(u64::MAX))
        || (route.destination.namespace == ChainNamespace::Solana
            && route.maximum_destination_amount > U256::from(u64::MAX))
    {
        return Err(InteropError::AmountOverflow);
    }
    match (route.source.namespace, route.verification) {
        (ChainNamespace::Eve, VerificationRequirement::EveConsensusAndApplicationCommitment)
        | (ChainNamespace::Ethereum, VerificationRequirement::EthereumBeaconFinalityAndReceipt)
        | (ChainNamespace::Solana, VerificationRequirement::SolanaAuthenticatedReplay) => Ok(()),
        _ => Err(InteropError::VerificationNamespaceMismatch),
    }
}
