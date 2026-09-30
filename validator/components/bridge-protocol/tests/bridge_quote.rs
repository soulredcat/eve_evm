use eve_bridge_protocol::{BridgeProtocolError, validate_bridge_quote};
use eve_interop::U256;
mod support;

#[test]
fn int0_quote_binds_route_amount_recipient_fees_and_protocol_height() {
    let context = support::context();
    let quote = support::quote(&context);
    assert_eq!(validate_bridge_quote(&context, &quote, 5), Ok(()));
    assert_eq!(
        validate_bridge_quote(&context, &quote, 10),
        Err(BridgeProtocolError::ExpiredQuote)
    );
    assert!(eve_interop::validate_route_admission(&context.route).is_err());
}

#[test]
fn int0_quote_rejects_wrong_network_amount_and_disclosure() {
    let context = support::context();
    let mut quote = support::quote(&context);
    quote.estimated_destination_amount = U256::from(11);
    assert_eq!(
        validate_bridge_quote(&context, &quote, 5),
        Err(BridgeProtocolError::InvalidQuote)
    );
    quote = support::quote(&context);
    quote.recipient.chain.genesis[0] ^= 1;
    assert!(validate_bridge_quote(&context, &quote, 5).is_err());
    quote = support::quote(&context);
    quote.source_fee.chain = context.route.destination;
    assert_eq!(
        validate_bridge_quote(&context, &quote, 5),
        Err(BridgeProtocolError::InvalidQuote)
    );
    quote = support::quote(&context);
    quote.crypto_coverage = eve_interop::CryptoCoverage::MixedTrust;
    assert_eq!(
        validate_bridge_quote(&context, &quote, 5),
        Err(BridgeProtocolError::InvalidQuote)
    );
    quote = support::quote(&context);
    quote.required_profile =
        eve_bridge_protocol::RequiredAuthenticationProfile::ClassicalAndMldsa65 {
            encoding_version: 1,
            key_epoch: 1,
        };
    assert_eq!(
        validate_bridge_quote(&context, &quote, 5),
        Err(BridgeProtocolError::UnsupportedProfile)
    );
}
