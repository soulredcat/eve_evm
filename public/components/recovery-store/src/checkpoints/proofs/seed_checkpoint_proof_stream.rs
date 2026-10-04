// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use sha2::{Digest, Sha256};

pub(super) fn seed_checkpoint_proof_stream(
    snapshot_id: &[u8; 32],
    body_sha256: &[u8; 32],
    height: u64,
    target: &[u8],
) -> Sha256 {
    let mut hash = Sha256::new();
    hash.update(b"EVE_CHECKPOINT_PROOF_STREAM_V1");
    hash.update(snapshot_id);
    hash.update(body_sha256);
    hash.update(height.to_be_bytes());
    hash.update(
        u16::try_from(target.len())
            .expect("sealed target encoding fits u16")
            .to_be_bytes(),
    );
    hash.update(target);
    hash
}
