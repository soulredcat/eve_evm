// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SegmentedRecoveryBounds, SegmentedRecoveryCapacity, SegmentedRecoveryPolicy};
use crate::BudgetError;

/// Checked ceilings count all data segments and one complete marker part.
pub(super) fn derive_segmented_recovery_capacity(
    policy: SegmentedRecoveryPolicy,
    bounds: SegmentedRecoveryBounds,
) -> Result<SegmentedRecoveryCapacity, BudgetError> {
    let overhead = bounds
        .segment_header_bytes
        .checked_add(bounds.segment_footer_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let data_bytes_per_segment = policy
        .maximum_segment_bytes
        .checked_sub(overhead)
        .filter(|bytes| *bytes > 0)
        .ok_or(BudgetError::InconsistentBound)?;
    let quotient = bounds.maximum_logical_bytes / data_bytes_per_segment;
    let maximum_data_segments = quotient
        .checked_add(u64::from(
            !bounds
                .maximum_logical_bytes
                .is_multiple_of(data_bytes_per_segment),
        ))
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let quotient = maximum_data_segments / policy.maximum_segments_per_part;
    let maximum_data_parts = quotient
        .checked_add(u64::from(
            maximum_data_segments % policy.maximum_segments_per_part != 0,
        ))
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_retained_parts = maximum_data_parts
        .checked_add(1)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_retained_payload_bytes = maximum_data_segments
        .checked_mul(overhead)
        .and_then(|bytes| bytes.checked_add(bounds.maximum_logical_bytes))
        .and_then(|bytes| bytes.checked_add(policy.maximum_marker_bytes))
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_part_payload_bytes = policy
        .maximum_segments_per_part
        .checked_mul(policy.maximum_segment_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_physical_record_bytes = policy
        .maximum_segment_bytes
        .checked_add(bounds.opaque_record_header_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_marker_record_bytes = policy
        .maximum_marker_bytes
        .checked_add(bounds.opaque_record_header_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let maximum_physical_records = maximum_data_segments
        .checked_add(1)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let reserved_auxiliary_bytes = policy
        .maximum_metadata_bytes
        .checked_add(policy.maximum_scratch_bytes)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    Ok(SegmentedRecoveryCapacity {
        data_bytes_per_segment,
        maximum_data_segments,
        maximum_data_parts,
        maximum_retained_parts,
        maximum_retained_payload_bytes,
        maximum_part_payload_bytes,
        maximum_physical_record_bytes,
        maximum_marker_record_bytes,
        maximum_physical_records,
        physical_records_per_transaction: 1,
        required_metadata_bytes: bounds.required_metadata_bytes,
        required_scratch_bytes: bounds.required_scratch_bytes,
        reserved_auxiliary_bytes,
    })
}
