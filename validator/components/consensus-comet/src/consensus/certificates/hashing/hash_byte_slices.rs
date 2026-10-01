// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use sha2::{Digest, Sha256};

/// Exact native RFC6962 field tree: SHA256 leaf/domain prefixes and split points.
pub(in crate::consensus::certificates) fn hash_byte_slices(items: &[Vec<u8>]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    match items.len() {
        0 => {}
        1 => {
            hasher.update([0]);
            hasher.update(&items[0]);
        }
        length => {
            let split = 1_usize << (usize::BITS - 1 - (length - 1).leading_zeros());
            hasher.update([1]);
            hasher.update(hash_byte_slices(&items[..split]));
            hasher.update(hash_byte_slices(&items[split..]));
        }
    }
    hasher.finalize().into()
}
