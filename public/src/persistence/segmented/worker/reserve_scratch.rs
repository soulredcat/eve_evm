// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{
        SegmentedError,
        types::{ScratchLease, WorkerState},
    },
    required_segment_scratch,
};
use std::sync::{Arc, atomic::Ordering};
pub(super) fn reserve_scratch(
    state: &Arc<WorkerState>,
    payload_bytes: usize,
) -> Result<ScratchLease, SegmentedError> {
    let required = required_segment_scratch(payload_bytes, state.pool.repository)?;
    if required > state.pool.policy.maximum_scratch_bytes {
        return Err(SegmentedError::Capacity);
    }
    state
        .scratch
        .compare_exchange(0, required, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| SegmentedError::Capacity)?;
    Ok(ScratchLease {
        state: Arc::clone(state),
    })
}
