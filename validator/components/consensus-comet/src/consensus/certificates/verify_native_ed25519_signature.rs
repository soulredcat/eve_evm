// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CertificateError;
use ed25519_zebra::{Signature, VerificationKey};

/// Deterministic individual ZIP215 verification, matching native Comet criteria.
/// This verifies bytes only; membership/admission and the active profile are separate.
pub fn verify_native_ed25519_signature(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8],
) -> Result<(), CertificateError> {
    if message.len() > 4096 {
        return Err(CertificateError::InvalidSignature);
    }
    let key =
        VerificationKey::try_from(*public_key).map_err(|_| CertificateError::InvalidPublicKey)?;
    let signature =
        Signature::try_from(signature).map_err(|_| CertificateError::InvalidSignature)?;
    key.verify(&signature, message)
        .map_err(|_| CertificateError::InvalidSignature)
}
