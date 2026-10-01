// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::GenesisError;
use ed25519_dalek::VerifyingKey;

/// EVE enrollment requires canonical nonidentity prime-order points; native ZIP-215 verification is separate.
pub fn validate_classical_enrollment_key(bytes: &[u8; 32]) -> Result<(), GenesisError> {
    let key = VerifyingKey::from_bytes(bytes).map_err(|_| GenesisError::InvalidKey)?;
    let point = key.to_edwards();
    if key.is_weak() || !point.is_torsion_free() || point.compress().to_bytes() != *bytes {
        return Err(GenesisError::InvalidKey);
    }
    Ok(())
}
