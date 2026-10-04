// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::verify_startup_marker_membership::verify_startup_marker_membership;
use super::{
    super::{SegmentedError, SegmentedPartPool, SegmentedWorker, pool::acquire_segmented_worker},
    create_segmented_worker_state::create_segmented_worker_state,
    spawn_segmented_worker::spawn_segmented_worker,
};
use eve_storage::records::segmented::SegmentedRecoveryAnchor;
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_bootstrap_cursor, opaque_record_budget,
    opaque_record_cursor, opaque_record_identity,
};
use std::sync::Arc;
pub fn start_segmented_worker(
    repository: OpaqueRecordRepository,
    pool: Arc<SegmentedPartPool>,
    logical_parent: SegmentedRecoveryAnchor,
) -> Result<SegmentedWorker, SegmentedError> {
    let worker_lifetime = acquire_segmented_worker(&pool)?;
    if opaque_record_identity(&repository) != pool.namespace
        || opaque_record_budget(&repository) != pool.repository
    {
        return Err(SegmentedError::ForeignPool);
    }
    let cursor = opaque_record_cursor(&repository).map_err(|_| SegmentedError::StorageFailed)?;
    if logical_parent.cursor.sequence > cursor.sequence
        || logical_parent.state_binding == [0; 32]
        || logical_parent.cursor.content_hash == [0; 32]
        || (logical_parent.height == 0) != (logical_parent.cursor.sequence == 0)
        || (logical_parent.cursor.sequence == cursor.sequence && logical_parent.cursor != cursor)
    {
        return Err(SegmentedError::WrongCursor);
    }
    let state = create_segmented_worker_state(pool, cursor, logical_parent, worker_lifetime)?;
    if logical_parent.height == 0 {
        if logical_parent.cursor != opaque_record_bootstrap_cursor(&repository) {
            return Err(SegmentedError::WrongCursor);
        }
    } else {
        verify_startup_marker_membership(&repository, &state, logical_parent)?;
    }
    spawn_segmented_worker(repository, state)
}
