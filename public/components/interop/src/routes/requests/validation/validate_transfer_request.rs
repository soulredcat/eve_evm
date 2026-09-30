use crate::{
    InteropError, RouteManifest, TransferRequest, U256, convert_exact_amount,
    validate_chain_address, validate_route_manifest,
};

/// Check recipient/network and amount metadata, independently of route authorization.
pub fn validate_transfer_request(
    route: &RouteManifest,
    request: &TransferRequest,
) -> Result<U256, InteropError> {
    validate_route_manifest(route)?;
    if request.route_id != route.route_id {
        return Err(InteropError::UnknownRoute);
    }
    validate_chain_address(&request.recipient)?;
    if request.recipient.chain != route.destination {
        return Err(InteropError::RecipientNamespaceMismatch);
    }
    if request.source_amount > route.maximum_source_amount {
        return Err(InteropError::AmountOverflow);
    }
    convert_exact_amount(
        request.source_amount,
        route.source_decimals,
        route.destination_decimals,
        route.maximum_destination_amount,
    )
}
