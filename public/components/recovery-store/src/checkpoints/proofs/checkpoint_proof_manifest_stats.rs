// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointProofManifestPreflight, CheckpointProofManifestStats};

pub fn checkpoint_proof_manifest_stats(
    manifest: &CheckpointProofManifestPreflight<'_>,
) -> CheckpointProofManifestStats {
    CheckpointProofManifestStats {
        files: manifest.summary.files,
        height: manifest.summary.height,
        total_bytes: manifest.summary.total_bytes,
        manifest_bytes: manifest.bytes.len(),
        stream_sha256: manifest.summary.stream_hash,
    }
}
