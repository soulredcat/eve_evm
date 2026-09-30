use eve_bridge_protocol::{
    BridgeProtocolError, InclusionLocator, ProofSegment, validate_source_observation,
};
mod support;

#[test]
fn int0_bounded_metadata_never_claims_authenticated_source() {
    let context = support::context();
    let raw = support::observation(&context);
    let bounded = validate_source_observation(&context, raw.clone()).unwrap();
    assert_eq!(bounded.observation(), &raw);
    assert!(eve_interop::validate_route_admission(&context.route).is_err());
}

#[test]
fn int0_observation_rejects_wrong_network_custody_asset_recipient_and_locator() {
    let context = support::context();
    let mut raw = support::observation(&context);
    raw.source.genesis[0] ^= 1;
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::WrongSourceNetwork)
    );
    raw = support::observation(&context);
    raw.source_custody.chain.genesis[0] ^= 1;
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::WrongSourceCustody)
    );
    raw = support::observation(&context);
    raw.asset.chain = context.route.destination;
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::WrongOriginAsset)
    );
    raw = support::observation(&context);
    raw.recipient.chain.genesis[0] ^= 1;
    assert!(matches!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::InvalidMetadata(_))
    ));
    raw = support::observation(&context);
    raw.inclusion = InclusionLocator::SolanaInstruction {
        transaction_index: 0,
        instruction_index: 0,
        inner_instruction_index: None,
    };
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::WrongInclusionLocator)
    );
}

#[test]
fn int0_observation_caps_segments_total_bytes_and_empty_proofs() {
    let context = support::context();
    let mut raw = support::observation(&context);
    raw.proof.clear();
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::EmptyProof)
    );
    raw = support::observation(&context);
    raw.proof[0].bytes = vec![1; 17];
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::ProofLimitExceeded)
    );
    raw = support::observation(&context);
    raw.proof = vec![ProofSegment { bytes: vec![1; 16] }; 3];
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::ProofLimitExceeded)
    );
    raw = support::observation(&context);
    raw.proof = vec![ProofSegment { bytes: vec![1] }; 5];
    assert_eq!(
        validate_source_observation(&context, raw),
        Err(BridgeProtocolError::ProofLimitExceeded)
    );
    let mut invalid = context;
    invalid.proof_limits.maximum_total_bytes = u32::MAX;
    assert_eq!(
        validate_source_observation(&invalid, support::observation(&invalid)),
        Err(BridgeProtocolError::InvalidProofLimits)
    );
}

#[test]
fn int0_policy_anchor_and_destination_references_must_bind_expected_network() {
    let mut context = support::context();
    context.source_anchor.chain = context.route.destination;
    assert_eq!(
        validate_source_observation(&context, support::observation(&context)),
        Err(BridgeProtocolError::WrongSourceNetwork)
    );
    context = support::context();
    context.destination_deployment.address = context.route.source_custody;
    assert_eq!(
        validate_source_observation(&context, support::observation(&context)),
        Err(BridgeProtocolError::WrongDeployment)
    );
    context = support::context();
    context.source_verifier_identity = [0; 32];
    assert_eq!(
        validate_source_observation(&context, support::observation(&context)),
        Err(BridgeProtocolError::AuthenticationUnavailable)
    );
}
