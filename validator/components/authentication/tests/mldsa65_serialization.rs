use eve_crypto::{
    CryptoError, MAX_ML_DSA_MESSAGE_BYTES, ML_DSA_65_PUBLIC_KEY_BYTES, ML_DSA_65_SIGNATURE_BYTES,
    derive_mldsa65_key, export_mldsa65_public_key, sign_mldsa65, verify_mldsa65,
};

#[test]
fn tp01_deterministic_signatures_round_trip_standard_encoding() {
    let key = derive_mldsa65_key(&[42; 32]).expect("isolated test seed");
    let public = export_mldsa65_public_key(&key);
    let signature = sign_mldsa65(&key, b"canonical message", b"test").expect("sign");
    assert_eq!(public.len(), ML_DSA_65_PUBLIC_KEY_BYTES);
    assert_eq!(signature.len(), ML_DSA_65_SIGNATURE_BYTES);
    assert_eq!(
        signature,
        sign_mldsa65(&key, b"canonical message", b"test").unwrap()
    );
    assert_eq!(
        verify_mldsa65(&public, b"canonical message", b"test", &signature),
        Ok(())
    );
    assert!(verify_mldsa65(&public, b"different message", b"test", &signature).is_err());
    assert!(verify_mldsa65(&public, b"canonical message", b"other", &signature).is_err());
}

#[test]
fn tp01_rejects_wrong_seed_lengths() {
    assert!(matches!(
        derive_mldsa65_key(&[0; 31]),
        Err(CryptoError::InvalidSeedLength)
    ));
    assert!(matches!(
        derive_mldsa65_key(&[0; 33]),
        Err(CryptoError::InvalidSeedLength)
    ));
}

#[test]
fn tp01_rejects_truncated_or_oversized_wire_encodings() {
    let key = derive_mldsa65_key(&[43; 32]).unwrap();
    let public = export_mldsa65_public_key(&key);
    let signature = sign_mldsa65(&key, b"message", b"").unwrap();
    assert_eq!(
        verify_mldsa65(&public[..1951], b"message", b"", &signature),
        Err(CryptoError::InvalidPublicKeyLength)
    );
    let mut oversized_public = public.clone();
    oversized_public.push(0);
    assert_eq!(
        verify_mldsa65(&oversized_public, b"message", b"", &signature),
        Err(CryptoError::InvalidPublicKeyLength)
    );
    assert_eq!(
        verify_mldsa65(&public, b"message", b"", &signature[..3308]),
        Err(CryptoError::InvalidSignatureLength)
    );
    let mut oversized_signature = signature;
    oversized_signature.push(0);
    assert_eq!(
        verify_mldsa65(&public, b"message", b"", &oversized_signature),
        Err(CryptoError::InvalidSignatureLength)
    );
}

#[test]
fn tp01_rejects_noncanonical_signature_hint_encoding() {
    let key = derive_mldsa65_key(&[44; 32]).unwrap();
    let public = export_mldsa65_public_key(&key);
    let mut signature = sign_mldsa65(&key, b"message", b"").unwrap();
    signature[ML_DSA_65_SIGNATURE_BYTES - 1] = 255;
    assert_eq!(
        verify_mldsa65(&public, b"message", b"", &signature),
        Err(CryptoError::InvalidSignatureEncoding)
    );
}

#[test]
fn tp06_enforces_context_and_message_budgets_before_crypto() {
    let key = derive_mldsa65_key(&[45; 32]).unwrap();
    let public = export_mldsa65_public_key(&key);
    let signature = sign_mldsa65(&key, b"message", &[9; 255]).unwrap();
    assert_eq!(
        verify_mldsa65(&public, b"message", &[9; 255], &signature),
        Ok(())
    );
    assert_eq!(
        sign_mldsa65(&key, b"message", &[9; 256]),
        Err(CryptoError::ContextTooLarge)
    );
    assert_eq!(
        verify_mldsa65(&[], b"message", &[9; 256], &[]),
        Err(CryptoError::ContextTooLarge)
    );
    let oversized = vec![0; MAX_ML_DSA_MESSAGE_BYTES + 1];
    assert_eq!(
        sign_mldsa65(&key, &oversized, b""),
        Err(CryptoError::MessageTooLarge)
    );
    assert_eq!(
        verify_mldsa65(&[], &oversized, b"", &[]),
        Err(CryptoError::MessageTooLarge)
    );
}
