// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofLimits, CheckpointProofPendingKind, preflight_checkpoint_proof_manifest,
    types::{COMPLETION_MAGIC, HEADER_BYTES, MAGIC},
};

pub(super) fn is_valid_checkpoint_proof_pending(
    bytes: &[u8],
    kind: CheckpointProofPendingKind,
) -> bool {
    if kind == CheckpointProofPendingKind::Completion {
        if bytes.len() < COMPLETION_MAGIC.len() {
            return !COMPLETION_MAGIC.starts_with(bytes);
        }
        if &bytes[..COMPLETION_MAGIC.len()] != COMPLETION_MAGIC {
            return true;
        }
        return bytes.len() == 80;
    }
    if bytes.len() < MAGIC.len() {
        return !MAGIC.starts_with(bytes);
    }
    if &bytes[..MAGIC.len()] != MAGIC {
        return true;
    }
    if bytes.len() < HEADER_BYTES {
        return false;
    }
    // Unsupported future framing cannot be classified as corrupt current-format content.
    if bytes[24..26] != [0, 0] {
        return true;
    }
    let length = usize::from(u16::from_be_bytes(bytes[26..28].try_into().unwrap()));
    if length == 0 || length > 4_096 || HEADER_BYTES + length > bytes.len() {
        return false;
    }
    let version = &bytes[HEADER_BYTES..HEADER_BYTES + length];
    let Ok(target) = eve_state::decode_state_version(version) else {
        return false;
    };
    let snapshot = bytes[48..80].try_into().unwrap();
    let body = bytes[80..112].try_into().unwrap();
    // A foreign manifest exceeding this receiver's smaller resource policy remains valid content.
    // Validation stays borrowed; these format bounds never authorize allocating witness bodies.
    let format_limits = CheckpointProofLimits {
        maximum_witness_bytes: usize::MAX,
        maximum_total_bytes: usize::MAX,
        maximum_files: 10_001,
        maximum_manifest_bytes: 1_048_576,
        maximum_disk_bytes: usize::MAX,
    };
    preflight_checkpoint_proof_manifest(
        bytes,
        target.height,
        version,
        snapshot,
        body,
        &format_limits,
    )
    .is_ok()
}
