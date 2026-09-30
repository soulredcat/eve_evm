use ml_dsa::{MlDsa65, Signature};

use crate::{CryptoError, ML_DSA_65_SIGNATURE_BYTES};

pub(crate) fn decode_mldsa65_signature(bytes: &[u8]) -> Result<Signature<MlDsa65>, CryptoError> {
    if bytes.len() != ML_DSA_65_SIGNATURE_BYTES {
        return Err(CryptoError::InvalidSignatureLength);
    }
    Signature::<MlDsa65>::try_from(bytes).map_err(|_| CryptoError::InvalidSignatureEncoding)
}
