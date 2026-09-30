use crate::{BridgeProtocolError, BridgeQuote, SourceVerificationContext};
use eve_interop::{
    TransferRequest, U256, validate_asset_origin, validate_chain_identity,
    validate_transfer_request,
};

pub fn validate_bridge_quote(
    context: &SourceVerificationContext,
    quote: &BridgeQuote,
    current_source_height: u64,
) -> Result<(), BridgeProtocolError> {
    crate::verification::validate_source_context(context)?;
    let route = &context.route;
    if quote.required_profile != context.required_profile {
        return Err(BridgeProtocolError::UnsupportedProfile);
    }
    if quote.route_version != route.version
        || quote.asset != route.asset
        || quote.crypto_coverage != route.crypto_coverage
    {
        return Err(BridgeProtocolError::InvalidQuote);
    }
    let converted = validate_transfer_request(
        route,
        &TransferRequest {
            route_id: quote.route_id,
            source_amount: quote.source_amount,
            recipient: quote.recipient,
        },
    )
    .map_err(BridgeProtocolError::InvalidMetadata)?;
    if quote.estimated_destination_amount != converted
        || quote.minimum_destination_amount == U256::ZERO
        || quote.minimum_destination_amount > quote.estimated_destination_amount
    {
        return Err(BridgeProtocolError::InvalidQuote);
    }
    if current_source_height >= quote.expiry_source_height {
        return Err(BridgeProtocolError::ExpiredQuote);
    }
    if quote.source_fee.chain != route.source || quote.destination_fee.chain != route.destination {
        return Err(BridgeProtocolError::InvalidQuote);
    }
    for fee in [
        Some(quote.source_fee),
        Some(quote.destination_fee),
        quote.relayer_fee,
        quote.account_creation_fee,
    ]
    .into_iter()
    .flatten()
    {
        validate_chain_identity(&fee.chain).map_err(BridgeProtocolError::InvalidMetadata)?;
        validate_asset_origin(&fee.asset).map_err(BridgeProtocolError::InvalidMetadata)?;
        if fee.asset.chain != fee.chain {
            return Err(BridgeProtocolError::InvalidQuote);
        }
        if fee.chain != route.source && fee.chain != route.destination {
            return Err(BridgeProtocolError::InvalidQuote);
        }
    }
    Ok(())
}
