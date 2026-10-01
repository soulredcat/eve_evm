// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::hex;

use super::DevelopmentSpecError;

pub(super) fn decode_public_key(encoded: &str) -> Result<[u8; 32], DevelopmentSpecError> {
    let key = encoded
        .strip_prefix("0x")
        .filter(|key| key.len() == 64)
        .ok_or(DevelopmentSpecError::InvalidPublicKey)?;
    let decoded: [u8; 32] = hex::decode(key)
        .map_err(|_| DevelopmentSpecError::InvalidPublicKey)?
        .try_into()
        .map_err(|_| DevelopmentSpecError::InvalidPublicKey)?;
    if hex::encode(decoded) != key {
        return Err(DevelopmentSpecError::InvalidPublicKey);
    }
    Ok(decoded)
}
