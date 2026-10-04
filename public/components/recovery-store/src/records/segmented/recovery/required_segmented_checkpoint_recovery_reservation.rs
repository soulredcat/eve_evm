// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{required_segmented_recovery_reservation, types::SegmentedRecoveryError};
use crate::records::{
    OpaqueRecordBudget,
    segmented::{
        SegmentedCodecLimits,
        checkpoints::{
            CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits,
            required_checkpoint_base_membership_reservation,
        },
    },
};

/// Existing charged logical scanner plus the actual checkpoint membership reread.
/// This numeric admission acquires no lease; the caller retains real capacity.
pub fn required_segmented_checkpoint_recovery_reservation(
    budget: &OpaqueRecordBudget,
    limits: &SegmentedCodecLimits,
) -> Result<usize, SegmentedRecoveryError> {
    let base_limits = CheckpointBaseLimits {
        maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
    };
    let membership = required_checkpoint_base_membership_reservation(budget, &base_limits)
        .map_err(|error| match error {
            crate::records::segmented::checkpoints::CheckpointBaseError::ArithmeticOverflow => {
                SegmentedRecoveryError::ArithmeticOverflow
            }
            _ => SegmentedRecoveryError::InvalidAnchor,
        })?;
    required_segmented_recovery_reservation(budget, limits)?
        .checked_add(membership)
        .ok_or(SegmentedRecoveryError::ArithmeticOverflow)
}
