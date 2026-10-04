// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CompletedCheckpointStore;

/// Borrow the exact retained manifest under the completed store's actual caller metadata lease.
pub fn checkpoint_store_manifest_bytes(store: &CompletedCheckpointStore) -> &[u8] {
    &store.transfer.manifest
}
