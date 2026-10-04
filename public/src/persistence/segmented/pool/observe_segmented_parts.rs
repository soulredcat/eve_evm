// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{SegmentedError, SegmentedPartObservation, SegmentedPartPool};
pub fn observe_segmented_parts(
    pool: &SegmentedPartPool,
) -> Result<SegmentedPartObservation, SegmentedError> {
    let state = pool
        .accounting
        .lock()
        .map_err(|_| SegmentedError::AccountingUnavailable)?;
    let oldest_age = state
        .slots
        .iter()
        .flatten()
        .map(|slot| slot.created.elapsed())
        .max()
        .unwrap_or_default();
    Ok(SegmentedPartObservation {
        retained_parts: state.slots.iter().flatten().count(),
        retained_encoded_bytes: state.bytes,
        estimated_metadata_bytes: state.metadata,
        oldest_age,
    })
}
