// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedOwner, AppliedPublication, capture_applied_state};
use std::sync::Arc;

/// Failed base I/O/publication stops all admission while preserving the old usable state.
pub(super) fn fence_checkpoint_activation(owner: &mut AppliedOwner) {
    owner.storage_failed = true;
    let Ok(current) = capture_applied_state(&owner.reader) else {
        return;
    };
    let failed = Arc::new(AppliedPublication {
        generation: Arc::clone(&current.generation),
        markers: current.markers,
        durable_cursor: current.durable_cursor,
        admitted_cursor: current.admitted_cursor,
        storage_failed: true,
        segmented_position: current.segmented_position,
    });
    let Ok(mut publication) = owner.reader.publication.write() else {
        return;
    };
    let old = std::mem::replace(&mut *publication, failed);
    drop(publication);
    drop(old);
}
