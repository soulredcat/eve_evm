// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::SigningKey;

pub(super) fn seeded_signing_key(public_key: &[u8; 32]) -> SigningKey {
    for seed in 1..=4_u8 {
        let key = SigningKey::from_bytes(&[seed; 32]);
        if key.verifying_key().to_bytes() == *public_key {
            return key;
        }
    }
    panic!("fixture validator must use one of the four deterministic development keys")
}
