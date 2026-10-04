// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use sha2::{Digest, Sha256};

/// Standard transfer integrity, never finality or correct execution.
pub fn hash_state_delta_bytes(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
