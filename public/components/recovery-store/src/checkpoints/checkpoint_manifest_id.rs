// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointManifestPreflight;

pub fn checkpoint_manifest_id(manifest: &CheckpointManifestPreflight<'_>) -> [u8; 32] {
    manifest.summary.id
}
