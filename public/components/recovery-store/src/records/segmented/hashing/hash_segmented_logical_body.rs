// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use sha2::{Digest, Sha256};

/// Canonical full logical-body identity; local integrity, never finality.
pub fn hash_segmented_logical_body(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
