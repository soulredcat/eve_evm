use eve_interop::{
    ChainNamespace, CryptoCoverage, InteropError, RouteState, U256, VerificationRequirement,
    encode_route_binding, find_route_manifest, validate_route_admission, validate_route_manifest,
};
mod support;

#[test]
fn ti02_all_named_directions_have_metadata_and_remain_disabled() {
    let routes = support::named_routes();
    for route in &routes {
        assert_eq!(validate_route_manifest(route), Ok(()));
        assert_eq!(find_route_manifest(&routes, route.route_id), Ok(route));
        for state in [
            RouteState::NotImplemented,
            RouteState::LocalFixtureOnly,
            RouteState::VerifierIncomplete,
            RouteState::ReviewRequired,
            RouteState::Paused,
        ] {
            let mut request = *route;
            request.state = state;
            assert_eq!(
                validate_route_admission(&request),
                Err(InteropError::RouteDisabled(state))
            );
        }
    }
    assert_eq!(
        find_route_manifest(&routes, [99; 32]),
        Err(InteropError::UnknownRoute)
    );
}

#[test]
fn ti02_registry_rejects_duplicate_and_oversized_metadata() {
    let route = support::named_routes()[0];
    assert_eq!(
        find_route_manifest(&[route, route], route.route_id),
        Err(InteropError::DuplicateRoute)
    );
    assert_eq!(
        find_route_manifest(&[route; 65], route.route_id),
        Err(InteropError::RegistryCapacityExceeded)
    );
    assert_eq!(
        find_route_manifest(&[], route.route_id),
        Err(InteropError::UnknownRoute)
    );
}

#[test]
fn ti01_wrong_custody_network_and_third_chain_asset_are_rejected() {
    let mut route = support::named_routes()[0];
    route.source_custody.chain.genesis[0] ^= 1;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::CustodyNamespaceMismatch)
    );
    route = support::named_routes()[0];
    route.asset.chain = support::network(ChainNamespace::Solana);
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::UnsupportedAssetForwarding)
    );
    route = support::named_routes()[0];
    route.destination = route.source;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::InvalidRouteIdentity)
    );
    route = support::named_routes()[0];
    route.source_custody.bytes = eve_interop::AddressBytes::Evm([0; 20]);
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::InvalidCustodyAddress)
    );
}

#[test]
fn ti02_unknown_version_zero_route_and_wrong_verifier_are_rejected() {
    let mut route = support::named_routes()[0];
    route.version = 2;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::UnsupportedManifestVersion)
    );
    route = support::named_routes()[0];
    route.route_id = [0; 32];
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::InvalidRouteIdentity)
    );
    route = support::named_routes()[0];
    route.verification = VerificationRequirement::SolanaAuthenticatedReplay;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::VerificationNamespaceMismatch)
    );
}

#[test]
fn ti06_solana_amount_width_and_mapping_limits_are_rejected_before_admission() {
    let mut route = support::named_routes()[2];
    route.maximum_source_amount = U256::from(u64::MAX) + U256::from(1);
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::AmountOverflow)
    );
    route = support::named_routes()[3];
    route.maximum_destination_amount = U256::MAX;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::AmountOverflow)
    );
    route = support::named_routes()[0];
    route.source_decimals = 78;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::InvalidDecimals)
    );
    route = support::named_routes()[0];
    route.maximum_destination_amount = U256::ZERO;
    assert_eq!(
        validate_route_manifest(&route),
        Err(InteropError::ZeroAmount)
    );
}

#[test]
fn ti01_route_binding_matches_four_reviewed_golden_vectors() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/b0-binding-v1.json")).unwrap();
    for (index, route) in support::named_routes().iter().enumerate() {
        assert_eq!(
            hex::encode(encode_route_binding(route).unwrap()),
            fixture["route_bindings"][index].as_str().unwrap()
        );
    }
}

#[test]
fn ti01_route_binding_commits_to_destinations_amounts_and_crypto_disclosure() {
    let route = support::named_routes()[0];
    let original = encode_route_binding(&route).unwrap();
    let mut changed = route;
    changed.destination.genesis[0] ^= 1;
    changed.destination_custody.chain = changed.destination;
    assert_ne!(encode_route_binding(&changed).unwrap(), original);
    changed = route;
    changed.maximum_source_amount += U256::from(1);
    assert_ne!(encode_route_binding(&changed).unwrap(), original);
    changed = route;
    changed.crypto_coverage = CryptoCoverage::MixedTrust;
    assert_ne!(encode_route_binding(&changed).unwrap(), original);
    changed = route;
    changed.state = RouteState::Paused;
    assert_eq!(encode_route_binding(&changed).unwrap(), original);
    assert!(validate_route_admission(&changed).is_err());
}
