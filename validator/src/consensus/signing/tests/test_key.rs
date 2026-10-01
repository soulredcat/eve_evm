// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::SigningKey;

/// Deliberately unsafe test-only key material; this module is absent from production builds.
pub(in crate::consensus) fn test_key(index: u8) -> SigningKey {
    SigningKey::from_bytes(&[index; 32])
}
