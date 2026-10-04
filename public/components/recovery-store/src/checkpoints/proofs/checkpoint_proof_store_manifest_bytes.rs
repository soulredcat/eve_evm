// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CompletedCheckpointProofStore;

pub fn checkpoint_proof_store_manifest_bytes(store: &CompletedCheckpointProofStore) -> &[u8] {
    &store.transfer.manifest
}
