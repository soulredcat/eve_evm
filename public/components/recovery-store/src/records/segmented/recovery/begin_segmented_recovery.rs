// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    required_segmented_recovery_reservation::required_segmented_recovery_reservation,
    types::{SegmentedRecoveryError, SegmentedRecoveryScan},
    verify_segmented_recovery_anchor::verify_segmented_recovery_anchor,
};
use crate::records::{
    OpaqueRecordRepository, opaque_record_budget, opaque_record_identity,
    segmented::{SegmentedCodecLimits, SegmentedRecoveryAnchor},
};

/// Caller selects an already verified logical anchor. This operation checks its
/// actual local storage membership/body identity and reserves no capacity itself.
pub fn begin_segmented_recovery(
    repository: &OpaqueRecordRepository,
    anchor: SegmentedRecoveryAnchor,
    limits: &SegmentedCodecLimits,
    reserved_bytes: usize,
) -> Result<SegmentedRecoveryScan, SegmentedRecoveryError> {
    let budget = opaque_record_budget(repository);
    if reserved_bytes < required_segmented_recovery_reservation(&budget, limits)? {
        return Err(SegmentedRecoveryError::ReservationTooSmall);
    }
    let mut body = Vec::new();
    body.try_reserve_exact(limits.maximum_logical_bytes)
        .map_err(|_| SegmentedRecoveryError::AllocationFailed)?;
    if body.capacity() != limits.maximum_logical_bytes {
        return Err(SegmentedRecoveryError::AllocationFailed);
    }
    verify_segmented_recovery_anchor(repository, anchor, limits, &mut body)?;
    body.clear();
    Ok(SegmentedRecoveryScan {
        namespace: opaque_record_identity(repository),
        budget,
        limits: *limits,
        anchor,
        physical: anchor.cursor,
        candidate: None,
        pending: None,
        body,
        orphan_segments: 0,
        failed: false,
    })
}
