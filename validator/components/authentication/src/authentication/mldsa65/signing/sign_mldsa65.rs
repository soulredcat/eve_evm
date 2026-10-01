// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::authentication::mldsa65::validation::validate_mldsa65_input;
use crate::{CryptoError, MlDsa65SigningKey};

/// Use the standardized deterministic variant; durable anti-double-sign safety belongs to callers.
pub fn sign_mldsa65(
    signing_key: &MlDsa65SigningKey,
    message: &[u8],
    context: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    validate_mldsa65_input(message, context)?;
    let signature = signing_key
        .expanded_key()
        .sign_deterministic(message, context)
        .map_err(|_| CryptoError::SigningFailed)?;
    Ok(signature.encode().to_vec())
}
