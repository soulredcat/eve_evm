use eve_crypto::{derive_mldsa65_key, export_mldsa65_public_key, sign_mldsa65, verify_mldsa65};
use pqcrypto_mldsa::mldsa65;
use pqcrypto_traits::sign::{DetachedSignature as _, PublicKey as _};

#[test]
fn tp01_rustcrypto_signature_verifies_with_pqclean() {
    let key = derive_mldsa65_key(&[46; 32]).unwrap();
    let public = export_mldsa65_public_key(&key);
    let signature = sign_mldsa65(&key, b"cross implementation", b"shared context").unwrap();
    let public = mldsa65::PublicKey::from_bytes(&public).unwrap();
    let signature = mldsa65::DetachedSignature::from_bytes(&signature).unwrap();
    assert!(
        mldsa65::verify_detached_signature_ctx(
            &signature,
            b"cross implementation",
            b"shared context",
            &public
        )
        .is_ok()
    );
    assert!(
        mldsa65::verify_detached_signature_ctx(
            &signature,
            b"different message",
            b"shared context",
            &public
        )
        .is_err()
    );
    assert!(
        mldsa65::verify_detached_signature_ctx(
            &signature,
            b"cross implementation",
            b"other context",
            &public
        )
        .is_err()
    );
}

#[test]
fn tp01_pqclean_signature_verifies_with_rustcrypto() {
    let (public, secret) = mldsa65::keypair();
    let signature = mldsa65::detached_sign_ctx(b"cross implementation", b"shared context", &secret);
    assert_eq!(
        verify_mldsa65(
            public.as_bytes(),
            b"cross implementation",
            b"shared context",
            signature.as_bytes()
        ),
        Ok(())
    );
    assert!(
        verify_mldsa65(
            public.as_bytes(),
            b"different message",
            b"shared context",
            signature.as_bytes()
        )
        .is_err()
    );
    assert!(
        verify_mldsa65(
            public.as_bytes(),
            b"cross implementation",
            b"other context",
            signature.as_bytes()
        )
        .is_err()
    );
}
