// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointLimits;
use super::{CheckpointWitnessWireError, MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES};
use eve_state::{StateBudget, validate_state_budget};

pub(super) fn validate_checkpoint_witness_wire_limits(
    budget: &StateBudget,
    limits: CheckpointLimits,
) -> Result<(), CheckpointWitnessWireError> {
    validate_state_budget(budget).map_err(CheckpointWitnessWireError::State)?;
    if limits.maximum_height_gap == 0
        || limits.maximum_witness_bytes == 0
        || limits.maximum_witness_bytes > MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES
    {
        return Err(CheckpointWitnessWireError::InvalidLimits);
    }
    Ok(())
}
