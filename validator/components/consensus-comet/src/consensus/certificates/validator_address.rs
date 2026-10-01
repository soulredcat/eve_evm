// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use sha2::{Digest, Sha256};

/// Native Comet Ed25519 routing address; distinct from the validator's EVM owner.
pub fn validator_address(public_key: &[u8; 32]) -> [u8; 20] {
    let digest = Sha256::digest(public_key);
    let mut address = [0; 20];
    address.copy_from_slice(&digest[..20]);
    address
}
