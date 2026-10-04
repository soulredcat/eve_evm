// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ManifestSummary;

pub(super) fn checkpoint_completion_bytes(summary: &ManifestSummary) -> [u8; 80] {
    let mut bytes = [0_u8; 80];
    bytes[..16].copy_from_slice(b"EVE_CKPT_DONE_V1");
    bytes[16..48].copy_from_slice(&summary.id);
    bytes[48..].copy_from_slice(&summary.body_hash);
    bytes
}
