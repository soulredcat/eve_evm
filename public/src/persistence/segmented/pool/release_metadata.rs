// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::MetadataLease;
pub(super) fn release_metadata(lease: &MetadataLease) {
    let mut state = lease
        .pool
        .accounting
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    state.metadata -= lease.bytes;
}
