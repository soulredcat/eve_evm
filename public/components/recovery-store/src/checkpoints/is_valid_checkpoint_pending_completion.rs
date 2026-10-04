// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// True means valid or unrecognized content must be preserved; it grants no completion authority.
pub(super) fn is_valid_checkpoint_pending_completion(bytes: &[u8]) -> bool {
    bytes.len() >= 80
        || bytes.len() < 16 && !b"EVE_CKPT_DONE_V1".starts_with(bytes)
        || bytes.len() >= 16 && &bytes[..16] != b"EVE_CKPT_DONE_V1"
}
