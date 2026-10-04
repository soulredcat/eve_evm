// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointBaseError, CheckpointBaseLimits,
    validate_checkpoint_base_limits::validate_checkpoint_base_limits,
};
use crate::records::{OpaqueRecordBudget, validate_opaque_record_budget};

/// Actual row, retained canonical target and bounded parent/read/control scratch.
/// The caller holds real leases while the returned owned row survives.
pub fn required_checkpoint_base_membership_reservation(
    budget: &OpaqueRecordBudget,
    limits: &CheckpointBaseLimits,
) -> Result<usize, CheckpointBaseError> {
    validate_checkpoint_base_limits(limits)?;
    validate_opaque_record_budget(budget).map_err(|_| CheckpointBaseError::InvalidLimits)?;
    budget
        .maximum_read_bytes
        .checked_mul(3)
        .and_then(|bytes| bytes.checked_add(limits.maximum_payload_bytes))
        .and_then(|bytes| bytes.checked_add(8_192))
        .ok_or(CheckpointBaseError::ArithmeticOverflow)
}
