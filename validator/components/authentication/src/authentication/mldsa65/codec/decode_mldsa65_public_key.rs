use ml_dsa::{EncodedVerifyingKey, MlDsa65, VerifyingKey};

use crate::{CryptoError, ML_DSA_65_PUBLIC_KEY_BYTES};

pub(crate) fn decode_mldsa65_public_key(
    bytes: &[u8],
) -> Result<VerifyingKey<MlDsa65>, CryptoError> {
    if bytes.len() != ML_DSA_65_PUBLIC_KEY_BYTES {
        return Err(CryptoError::InvalidPublicKeyLength);
    }
    let encoded = EncodedVerifyingKey::<MlDsa65>::try_from(bytes)
        .map_err(|_| CryptoError::InvalidPublicKeyLength)?;
    Ok(VerifyingKey::<MlDsa65>::decode(&encoded))
}
