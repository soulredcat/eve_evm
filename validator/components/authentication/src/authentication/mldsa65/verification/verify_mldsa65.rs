// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::authentication::mldsa65::codec::{decode_mldsa65_public_key, decode_mldsa65_signature};
use crate::authentication::mldsa65::validation::validate_mldsa65_input;
use crate::{CryptoError, ML_DSA_65_PUBLIC_KEY_BYTES, ML_DSA_65_SIGNATURE_BYTES};

/// Verify external pure ML-DSA-65, including context and strict bounded wire encodings.
pub fn verify_mldsa65(
    public_key: &[u8],
    message: &[u8],
    context: &[u8],
    signature: &[u8],
) -> Result<(), CryptoError> {
    validate_mldsa65_input(message, context)?;
    if public_key.len() != ML_DSA_65_PUBLIC_KEY_BYTES {
        return Err(CryptoError::InvalidPublicKeyLength);
    }
    if signature.len() != ML_DSA_65_SIGNATURE_BYTES {
        return Err(CryptoError::InvalidSignatureLength);
    }
    let signature = decode_mldsa65_signature(signature)?;
    let public_key = decode_mldsa65_public_key(public_key)?;
    if !public_key.verify_with_context(message, context, &signature) {
        return Err(CryptoError::InvalidSignature);
    }
    Ok(())
}
