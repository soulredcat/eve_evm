use eve_interop::{
    AssetKind, AssetOrigin, ChainIdentity, ChainNamespace, CryptoCoverage, InteropError,
    RouteManifest, RouteState, U256, VerificationRequirement, decode_chain_address,
    encode_chain_identity, validate_route_admission, validate_route_manifest,
};

fn reservations() -> [ChainIdentity; 2] {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/two-eve-reservations-v1.json")).unwrap();
    assert_eq!(fixture["route_state"], "VERIFIER_INCOMPLETE");
    assert_eq!(fixture["deployment_approval"], "DISABLED_NOT_APPROVED");
    [0, 1].map(|index| ChainIdentity {
        namespace: ChainNamespace::Eve,
        genesis: hex::decode(
            fixture["networks"][index]["synthetic_genesis"]
                .as_str()
                .unwrap(),
        )
        .unwrap()
        .try_into()
        .unwrap(),
        numeric_id: Some(U256::from(
            fixture["networks"][index]["chain_id"].as_u64().unwrap(),
        )),
    })
}

#[test]
fn sec0_two_eve_reservations_have_distinct_full_network_identity() {
    let [source, destination] = reservations();
    assert_ne!(source, destination);
    assert_ne!(source.genesis, destination.genesis);
    assert_ne!(source.numeric_id, destination.numeric_id);
    assert_ne!(
        encode_chain_identity(&source).unwrap(),
        encode_chain_identity(&destination).unwrap()
    );
}

#[test]
fn sec0_local_fake_native_route_cannot_approve_custody() {
    let [source, destination] = reservations();
    let route = RouteManifest {
        version: 1,
        route_id: [7; 32],
        source,
        destination,
        source_custody: decode_chain_address(source, &[4; 20]).unwrap(),
        destination_custody: decode_chain_address(destination, &[5; 20]).unwrap(),
        asset: AssetOrigin {
            chain: source,
            kind: AssetKind::Native,
        },
        source_decimals: 18,
        destination_decimals: 18,
        maximum_source_amount: U256::from(1_000_000_000_000_000_000u64),
        maximum_destination_amount: U256::from(1_000_000_000_000_000_000u64),
        verification: VerificationRequirement::EveConsensusAndApplicationCommitment,
        crypto_coverage: CryptoCoverage::Classical,
        state: RouteState::VerifierIncomplete,
    };
    assert_eq!(validate_route_manifest(&route), Ok(()));
    assert_eq!(
        validate_route_admission(&route),
        Err(InteropError::RouteDisabled(RouteState::VerifierIncomplete))
    );
}
