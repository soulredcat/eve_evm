mod support;

use eve_crypto::{
    AuthorizationPurpose, CryptoError, HybridAuthorization, MAX_AUTHORIZATION_PAYLOAD_BYTES,
    derive_mldsa65_key, encode_hybrid_message, export_mldsa65_public_key,
    verify_hybrid_authorization,
};
use support::hybrid_fixture::{fixture, proof, sign};

#[test]
fn tp02_requires_both_signatures_for_the_same_enrolled_identity() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::ConsensusVote,
        payload: b"height=11;round=0;step=precommit;block=abc",
    };
    let signed = sign(&fixture, &authorization);
    let mut signatures = proof(&fixture, &signed);
    assert_eq!(
        verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures),
        Ok(())
    );
    signatures.mldsa65_signature = &[];
    assert_eq!(
        verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures),
        Err(CryptoError::InvalidSignatureLength)
    );
    signatures = proof(&fixture, &signed);
    signatures.ed25519_signature = &[];
    assert_eq!(
        verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures),
        Err(CryptoError::InvalidClassicalSignatureLength)
    );
}

#[test]
fn tp02_rejects_changed_identity_and_key_epoch() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::AccountOperation,
        payload: b"account nonce and complete operation",
    };
    let signed = sign(&fixture, &authorization);
    let mut signatures = proof(&fixture, &signed);
    signatures.signer_identity = [51; 32];
    assert_eq!(
        verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures),
        Err(CryptoError::WrongIdentity)
    );
    signatures = proof(&fixture, &signed);
    signatures.key_epoch += 1;
    assert_eq!(
        verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures),
        Err(CryptoError::WrongKeyEpoch)
    );
    let mut changed_enrollment = fixture.enrollment.clone();
    changed_enrollment.key_epoch += 1;
    signatures.key_epoch = changed_enrollment.key_epoch;
    assert!(verify_hybrid_authorization(&changed_enrollment, &authorization, &signatures).is_err());
}

#[test]
fn tp02_rejects_changed_network_purpose_or_payload() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::AccountRecovery,
        payload: b"complete recovery operation",
    };
    let signed = sign(&fixture, &authorization);
    let signatures = proof(&fixture, &signed);
    let mut changed = authorization;
    changed.genesis_hash = [52; 32];
    assert!(verify_hybrid_authorization(&fixture.enrollment, &changed, &signatures).is_err());
    changed = authorization;
    changed.purpose = AuthorizationPurpose::Release;
    assert!(verify_hybrid_authorization(&fixture.enrollment, &changed, &signatures).is_err());
    changed = authorization;
    changed.payload = b"different recovery operation";
    assert!(verify_hybrid_authorization(&fixture.enrollment, &changed, &signatures).is_err());
}

#[test]
fn tp02_rejects_valid_components_signed_over_different_messages() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::BridgeTransfer,
        payload: b"route and complete transfer identity A",
    };
    let signed = sign(&fixture, &authorization);
    let other = HybridAuthorization {
        payload: b"route and complete transfer identity B",
        ..authorization
    };
    let other_signed = sign(&fixture, &other);
    let mut signatures = proof(&fixture, &signed);
    signatures.mldsa65_signature = &other_signed.pq;
    assert!(verify_hybrid_authorization(&fixture.enrollment, &authorization, &signatures).is_err());
}

#[test]
fn tp02_binds_both_registered_public_keys() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::Enrollment,
        payload: b"proof of possession",
    };
    let signed = sign(&fixture, &authorization);
    let signatures = proof(&fixture, &signed);
    let mut changed = fixture.enrollment.clone();
    changed.mldsa65_public_key = export_mldsa65_public_key(&derive_mldsa65_key(&[53; 32]).unwrap());
    assert!(verify_hybrid_authorization(&changed, &authorization, &signatures).is_err());
    changed = fixture.enrollment.clone();
    changed.ed25519_public_key = ed25519_dalek::SigningKey::from_bytes(&[54; 32])
        .verifying_key()
        .to_bytes();
    assert!(verify_hybrid_authorization(&changed, &authorization, &signatures).is_err());
}

#[test]
fn tp02_rejects_invalid_signature_even_when_other_component_is_valid() {
    let fixture = fixture();
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::ValidatorTransition,
        payload: b"complete next validator set",
    };
    let mut signed = sign(&fixture, &authorization);
    signed.pq[0] ^= 1;
    assert!(
        verify_hybrid_authorization(
            &fixture.enrollment,
            &authorization,
            &proof(&fixture, &signed)
        )
        .is_err()
    );
    signed = sign(&fixture, &authorization);
    signed.classical[0] ^= 1;
    assert!(
        verify_hybrid_authorization(
            &fixture.enrollment,
            &authorization,
            &proof(&fixture, &signed)
        )
        .is_err()
    );
}

#[test]
fn tp06_hybrid_payload_limit_preserves_the_full_key_bound_envelope() {
    let fixture = fixture();
    let payload = vec![0; MAX_AUTHORIZATION_PAYLOAD_BYTES];
    let authorization = HybridAuthorization {
        genesis_hash: [50; 32],
        purpose: AuthorizationPurpose::AccountOperation,
        payload: &payload,
    };
    let signed = sign(&fixture, &authorization);
    assert_eq!(
        verify_hybrid_authorization(
            &fixture.enrollment,
            &authorization,
            &proof(&fixture, &signed)
        ),
        Ok(())
    );
    let oversized = vec![0; MAX_AUTHORIZATION_PAYLOAD_BYTES + 1];
    let oversized_authorization = HybridAuthorization {
        payload: &oversized,
        ..authorization
    };
    assert_eq!(
        encode_hybrid_message(&fixture.enrollment, &oversized_authorization),
        Err(CryptoError::MessageTooLarge)
    );
}
