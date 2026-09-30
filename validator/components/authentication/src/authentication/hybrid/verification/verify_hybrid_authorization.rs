use ed25519_dalek::{Signature, VerifyingKey};

use crate::{
    CryptoError, EnrolledHybridIdentity, HybridAuthorization, HybridSignature,
    ML_DSA_65_SIGNATURE_BYTES, encode_hybrid_message, verify_mldsa65,
};

/// Verify both components over one key-bound logical message for one expected enrolled identity.
/// Callers must authenticate registry/profile history and enforce signing durability separately.
pub fn verify_hybrid_authorization(
    enrollment: &EnrolledHybridIdentity,
    authorization: &HybridAuthorization<'_>,
    signature: &HybridSignature<'_>,
) -> Result<(), CryptoError> {
    if signature.signer_identity != enrollment.identity {
        return Err(CryptoError::WrongIdentity);
    }
    if signature.key_epoch != enrollment.key_epoch {
        return Err(CryptoError::WrongKeyEpoch);
    }
    if signature.ed25519_signature.len() != 64 {
        return Err(CryptoError::InvalidClassicalSignatureLength);
    }
    if signature.mldsa65_signature.len() != ML_DSA_65_SIGNATURE_BYTES {
        return Err(CryptoError::InvalidSignatureLength);
    }
    let message = encode_hybrid_message(enrollment, authorization)?;
    let classical_key = VerifyingKey::from_bytes(&enrollment.ed25519_public_key)
        .map_err(|_| CryptoError::InvalidClassicalPublicKey)?;
    let classical_signature = Signature::from_slice(signature.ed25519_signature)
        .map_err(|_| CryptoError::InvalidClassicalSignatureLength)?;
    classical_key
        .verify_strict(&message, &classical_signature)
        .map_err(|_| CryptoError::InvalidClassicalSignature)?;
    verify_mldsa65(
        &enrollment.mldsa65_public_key,
        &message,
        b"EVE_HYBRID_V1",
        signature.mldsa65_signature,
    )
}
