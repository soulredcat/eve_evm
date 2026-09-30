#![allow(dead_code)] // Separate integration binaries select relevant fixture builders.
use eve_bridge_protocol::{
    BridgeQuote, DeploymentIdentity, InclusionLocator, ProofLimits, ProofSegment, QuotedFee,
    RequiredAuthenticationProfile, SourceObservation, SourceVerificationContext,
    TrustAnchorReference,
};
use eve_interop::{
    AssetKind, AssetOrigin, ChainIdentity, ChainNamespace, CryptoCoverage, RouteManifest,
    RouteState, U256, VerificationRequirement, decode_chain_address,
};

pub fn context() -> SourceVerificationContext {
    let source = ChainIdentity {
        namespace: ChainNamespace::Ethereum,
        genesis: [2; 32],
        numeric_id: Some(U256::from(1)),
    };
    let destination = ChainIdentity {
        namespace: ChainNamespace::Eve,
        genesis: [1; 32],
        numeric_id: Some(U256::from(31337)),
    };
    let source_custody = decode_chain_address(source, &[4; 20]).unwrap();
    let destination_custody = decode_chain_address(destination, &[5; 20]).unwrap();
    SourceVerificationContext {
        route: RouteManifest {
            version: 1,
            route_id: [7; 32],
            source,
            destination,
            source_custody,
            destination_custody,
            asset: AssetOrigin {
                chain: source,
                kind: AssetKind::Native,
            },
            source_decimals: 18,
            destination_decimals: 18,
            maximum_source_amount: U256::from(100),
            maximum_destination_amount: U256::from(100),
            verification: VerificationRequirement::EthereumBeaconFinalityAndReceipt,
            crypto_coverage: CryptoCoverage::Classical,
            state: RouteState::VerifierIncomplete,
        },
        required_profile: RequiredAuthenticationProfile::ClassicalDevelopment,
        source_anchor: TrustAnchorReference {
            chain: source,
            height: 1,
            block_hash: [8; 32],
            historical_key_set_commitment: [9; 32],
        },
        source_verifier_identity: [10; 32],
        destination_deployment: DeploymentIdentity {
            address: destination_custody,
            implementation_commitment: [11; 32],
        },
        proof_limits: ProofLimits {
            maximum_total_bytes: 32,
            maximum_segment_bytes: 16,
            maximum_segments: 4,
        },
    }
}

pub fn observation(context: &SourceVerificationContext) -> SourceObservation {
    let route = context.route;
    SourceObservation {
        source: route.source,
        source_height: 2,
        source_block_hash: [12; 32],
        source_custody: route.source_custody,
        source_sequence: 1,
        route_id: route.route_id,
        asset: route.asset,
        source_amount: U256::from(10),
        recipient: route.destination_custody,
        inclusion: InclusionLocator::EvmReceipt {
            transaction_index: 1,
            log_index: 0,
        },
        // Deliberately untrusted bytes. Passing metadata bounds is not proof validation.
        proof: vec![ProofSegment { bytes: vec![1; 8] }],
    }
}

pub fn quote(context: &SourceVerificationContext) -> BridgeQuote {
    let route = context.route;
    BridgeQuote {
        route_id: route.route_id,
        route_version: route.version,
        asset: route.asset,
        source_amount: U256::from(10),
        estimated_destination_amount: U256::from(10),
        minimum_destination_amount: U256::from(9),
        recipient: route.destination_custody,
        source_fee: QuotedFee {
            chain: route.source,
            asset: route.asset,
            amount: U256::from(1),
        },
        destination_fee: QuotedFee {
            chain: route.destination,
            asset: AssetOrigin {
                chain: route.destination,
                kind: AssetKind::Native,
            },
            amount: U256::from(1),
        },
        relayer_fee: None,
        account_creation_fee: None,
        expiry_source_height: 10,
        required_profile: context.required_profile,
        crypto_coverage: route.crypto_coverage,
    }
}
