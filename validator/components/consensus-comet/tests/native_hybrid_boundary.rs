use eve_consensus_comet::{
    consensus::authentication::{
        ConsensusAuthenticationRequirement, UnsupportedHybridConsensus,
        require_supported_authentication,
    },
    wire::tendermint::crypto::{PublicKey, public_key::Sum},
};
use prost::Message;

#[test]
fn native_key_oneof_retains_one_algorithm_instead_of_both() {
    let classical = PublicKey {
        sum: Some(Sum::Ed25519(vec![0x11; 32])),
    };
    let pq = PublicKey {
        sum: Some(Sum::Mldsa65(vec![0x22; 1952])),
    };
    let mut wire = classical.encode_to_vec();
    wire.extend(pq.encode_to_vec());
    assert_eq!(PublicKey::decode(wire.as_slice()).unwrap(), pq);
    let mut reverse = pq.encode_to_vec();
    reverse.extend(classical.encode_to_vec());
    assert_eq!(PublicKey::decode(reverse.as_slice()).unwrap(), classical);
}

#[test]
fn activated_hybrid_requirement_fails_closed_for_native_engine() {
    assert_eq!(
        require_supported_authentication(ConsensusAuthenticationRequirement::ClassicalDev),
        Ok(())
    );
    assert_eq!(
        require_supported_authentication(ConsensusAuthenticationRequirement::ClassicalAndMldsa65),
        Err(UnsupportedHybridConsensus)
    );
}
