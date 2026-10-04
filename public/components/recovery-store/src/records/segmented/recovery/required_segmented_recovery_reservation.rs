// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{RecoveredSegmentedBundle, SegmentedRecoveryError, SegmentedRecoveryScan};
use crate::records::{
    OpaqueRecordBudget,
    segmented::{SegmentedCodecLimits, validate_segmented_codec_limits},
};

/// Conservative logical-buffer/read/control reservation; not allocator/RSS proof.
/// The caller must hold real leases, including while returned owned bytes survive.
pub fn required_segmented_recovery_reservation(
    budget: &OpaqueRecordBudget,
    limits: &SegmentedCodecLimits,
) -> Result<usize, SegmentedRecoveryError> {
    validate_segmented_codec_limits(limits).map_err(SegmentedRecoveryError::Codec)?;
    if budget.maximum_record_bytes < 88 || budget.maximum_read_bytes < budget.maximum_record_bytes {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let control = core::mem::size_of::<SegmentedRecoveryScan>()
        .checked_add(core::mem::size_of::<RecoveredSegmentedBundle>())
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(SegmentedRecoveryError::ArithmeticOverflow)?;
    budget
        .maximum_read_bytes
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(limits.maximum_logical_bytes))
        .and_then(|bytes| bytes.checked_add(control))
        .ok_or(SegmentedRecoveryError::ArithmeticOverflow)
}
