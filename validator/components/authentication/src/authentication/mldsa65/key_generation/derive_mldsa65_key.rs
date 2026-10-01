// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ml_dsa::{MlDsa65, Seed, SigningKey};

use crate::{CryptoError, MlDsa65SigningKey};

/// Derive a key from a caller-protected, securely generated seed, or an isolated test vector.
/// This API does not generate randomness or decode untrusted expanded private keys.
pub fn derive_mldsa65_key(seed: &[u8]) -> Result<MlDsa65SigningKey, CryptoError> {
    let seed = Seed::try_from(seed).map_err(|_| CryptoError::InvalidSeedLength)?;
    Ok(SigningKey::<MlDsa65>::from_seed(&seed))
}
