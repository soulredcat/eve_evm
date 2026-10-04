// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointManifestPreflight, CheckpointManifestStats};

/// Same-buffer sizes only; detached statistics authenticate no source or state.
pub fn checkpoint_manifest_stats(
    manifest: &CheckpointManifestPreflight<'_>,
) -> CheckpointManifestStats {
    let summary = manifest.summary;
    CheckpointManifestStats {
        manifest_bytes: manifest.bytes.len(),
        body_bytes: summary.body_bytes,
        chunk_bytes: summary.chunk_bytes,
        chunks: summary.chunks,
        target_version_bytes: summary.version_bytes,
        body_sha256: summary.body_hash,
    }
}
