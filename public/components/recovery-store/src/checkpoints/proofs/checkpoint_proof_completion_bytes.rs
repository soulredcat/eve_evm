// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{COMPLETION_MAGIC, ProofSummary};

pub(super) fn checkpoint_proof_completion_bytes(summary: &ProofSummary) -> [u8; 80] {
    let mut bytes = [0_u8; 80];
    bytes[..16].copy_from_slice(COMPLETION_MAGIC);
    bytes[16..48].copy_from_slice(&summary.id);
    bytes[48..80].copy_from_slice(&summary.stream_hash);
    bytes
}
