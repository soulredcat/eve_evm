// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointLimits, MAXIMUM_CHECKPOINT_CHUNK_BYTES, preflight_checkpoint_manifest,
    types::{HEADER_BYTES, MAGIC},
};

/// Borrowed conservative repair classification, independent of receiver admission resource limits.
pub(super) fn is_valid_checkpoint_pending_manifest(bytes: &[u8]) -> bool {
    if bytes.len() < MAGIC.len() {
        return !MAGIC.starts_with(bytes);
    }
    if &bytes[..MAGIC.len()] != MAGIC {
        return true;
    }
    if bytes.len() < HEADER_BYTES {
        return false;
    }
    // An unrecognized future codec is preserved; unsupported is not proof of corrupt content.
    if bytes[24..26] != [0, 0] {
        return true;
    }
    let length = usize::from(u16::from_be_bytes(bytes[26..28].try_into().unwrap()));
    if length == 0 || length > 4_096 || HEADER_BYTES + length > bytes.len() {
        return false;
    }
    let version = &bytes[HEADER_BYTES..HEADER_BYTES + length];
    if eve_state::preflight_state_version(version).is_err() {
        return false;
    }
    let Some(maximum_body_bytes) = MAXIMUM_CHECKPOINT_CHUNK_BYTES.checked_mul(4_096) else {
        return true;
    };
    let mut logical = eve_state::development_state_budget();
    logical.maximum_commit_bytes = maximum_body_bytes;
    let format_limits = CheckpointLimits {
        logical,
        maximum_body_bytes,
        maximum_chunk_bytes: MAXIMUM_CHECKPOINT_CHUNK_BYTES,
        maximum_chunks: 4_096,
        maximum_manifest_bytes: 262_144,
    };
    preflight_checkpoint_manifest(bytes, version, &format_limits).is_ok()
}
