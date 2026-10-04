// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointProofManifestPreflight;

pub fn checkpoint_proof_manifest_id(manifest: &CheckpointProofManifestPreflight<'_>) -> [u8; 32] {
    manifest.summary.id
}
