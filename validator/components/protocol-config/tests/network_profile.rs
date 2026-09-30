#[path = "support/mod.rs"]
mod support;

use alloy_primitives::B256;
use eve_protocol_config::network::{
    ProfileError, SecurityProfile, decode_security_profile, validate_network_profile,
    validate_security_profile,
};

#[test]
fn matches_trusted_applicable_height_binding_and_refuses_network_or_epoch_substitution() {
    let expected = support::binding();
    assert!(validate_network_profile(&expected, &expected).is_ok());
    let mut received = expected.clone();
    received.genesis_hash = B256::repeat_byte(2);
    assert_eq!(
        validate_network_profile(&expected, &received),
        Err(ProfileError::WrongNetwork)
    );
    received = expected.clone();
    received.key_epoch = 1;
    assert_eq!(
        validate_network_profile(&expected, &received),
        Err(ProfileError::WrongProfileHistory)
    );
    received = expected.clone();
    received.protocol_version = 2;
    assert_eq!(
        validate_network_profile(&expected, &received),
        Err(ProfileError::WrongProtocol)
    );
    received = expected.clone();
    received.genesis_hash = B256::ZERO;
    assert_eq!(
        validate_network_profile(&received, &received),
        Err(ProfileError::InvalidBinding)
    );
}

#[test]
fn activated_hybrid_and_verified_pq_profiles_refuse_native_consensus_even_if_both_peers_agree() {
    for tag in [0, 4, 255] {
        assert_eq!(
            decode_security_profile(tag),
            Err(ProfileError::InvalidBinding)
        );
    }
    for profile in [
        SecurityProfile::HybridExperimental,
        SecurityProfile::PqProfileVerified,
    ] {
        assert_eq!(
            validate_security_profile(profile),
            Err(ProfileError::UnsupportedHybrid)
        );
        let mut expected = support::binding();
        expected.profile = profile;
        assert_eq!(
            validate_network_profile(&expected, &expected),
            Err(ProfileError::UnsupportedHybrid)
        );
    }
}
