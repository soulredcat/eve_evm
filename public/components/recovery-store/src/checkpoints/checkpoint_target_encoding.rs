// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CompletedCheckpointStore, types::HEADER_BYTES};

pub fn checkpoint_target_encoding(store: &CompletedCheckpointStore) -> &[u8] {
    &store.transfer.manifest[HEADER_BYTES..HEADER_BYTES + store.transfer.summary.version_bytes]
}
