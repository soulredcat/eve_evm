use eve_interop::{
    ChainNamespace, InteropError, TransferRequest, U256, decode_chain_address,
    validate_transfer_request,
};
mod support;

#[test]
fn ti06_each_named_direction_converts_exact_amount_without_authorizing_claims() {
    for route in support::named_routes() {
        let request = TransferRequest {
            route_id: route.route_id,
            source_amount: route.maximum_source_amount,
            recipient: route.destination_custody,
        };
        assert_eq!(
            validate_transfer_request(&route, &request),
            Ok(route.maximum_destination_amount)
        );
        assert!(eve_interop::validate_route_admission(&route).is_err());
    }
}

#[test]
fn ti01_recipient_bytes_do_not_override_wrong_genesis_or_namespace() {
    let route = support::named_routes()[0];
    let mut request = TransferRequest {
        route_id: route.route_id,
        source_amount: route.maximum_source_amount,
        recipient: route.destination_custody,
    };
    request.recipient.chain.genesis[0] ^= 1;
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::RecipientNamespaceMismatch)
    );
    let other = support::network(ChainNamespace::Ethereum);
    request.recipient = decode_chain_address(other, &[5; 20]).unwrap();
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::RecipientNamespaceMismatch)
    );
    request.recipient = route.destination_custody;
    request.route_id[0] ^= 1;
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::UnknownRoute)
    );
}

#[test]
fn ti06_transfer_limit_and_non_exact_conversion_fail_before_quotes() {
    let route = support::named_routes()[3];
    let mut request = TransferRequest {
        route_id: route.route_id,
        source_amount: route.maximum_source_amount + U256::from(1),
        recipient: route.destination_custody,
    };
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::AmountOverflow)
    );
    request.source_amount = U256::from(1);
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::NonExactAmount)
    );
    request.source_amount = U256::ZERO;
    assert_eq!(
        validate_transfer_request(&route, &request),
        Err(InteropError::ZeroAmount)
    );
}
