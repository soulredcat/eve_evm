use crate::{
    CryptoError, EnrolledHybridIdentity, HybridAuthorization, MAX_AUTHORIZATION_PAYLOAD_BYTES,
    ML_DSA_65_PUBLIC_KEY_BYTES,
};

/// Bind the fixed experimental profile, network, purpose, identity, epoch, both keys and payload.
/// This envelope is not a replacement for an external consensus engine's canonical sign bytes.
pub fn encode_hybrid_message(
    enrollment: &EnrolledHybridIdentity,
    authorization: &HybridAuthorization<'_>,
) -> Result<Vec<u8>, CryptoError> {
    if authorization.payload.len() > MAX_AUTHORIZATION_PAYLOAD_BYTES {
        return Err(CryptoError::MessageTooLarge);
    }
    if enrollment.mldsa65_public_key.len() != ML_DSA_65_PUBLIC_KEY_BYTES {
        return Err(CryptoError::InvalidPublicKeyLength);
    }
    let domain = b"EVE_HYBRID_EXPERIMENTAL_V1";
    let profile = b"ED25519_AND_ML_DSA_65";
    let header_bytes = domain.len() + profile.len() + 32 + 1 + 32 + 8 + 32 + 1952 + 4;
    let payload_bytes =
        u32::try_from(authorization.payload.len()).map_err(|_| CryptoError::MessageTooLarge)?;
    let mut bytes = Vec::with_capacity(authorization.payload.len() + header_bytes);
    bytes.extend_from_slice(domain);
    bytes.extend_from_slice(profile);
    bytes.extend_from_slice(&authorization.genesis_hash);
    bytes.push(authorization.purpose as u8);
    bytes.extend_from_slice(&enrollment.identity);
    bytes.extend_from_slice(&enrollment.key_epoch.to_be_bytes());
    bytes.extend_from_slice(&enrollment.ed25519_public_key);
    bytes.extend_from_slice(&enrollment.mldsa65_public_key);
    bytes.extend_from_slice(&payload_bytes.to_be_bytes());
    bytes.extend_from_slice(authorization.payload);
    Ok(bytes)
}
