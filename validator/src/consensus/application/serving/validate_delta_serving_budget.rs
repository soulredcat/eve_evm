// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{DeltaServingBudget, DeltaServingError};
use eve_state::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES,
    MAXIMUM_STATE_DELTA_REQUEST_BYTES,
};

pub(in crate::consensus::application) fn validate_delta_serving_budget(
    budget: DeltaServingBudget,
) -> Result<(), DeltaServingError> {
    if budget.maximum_working_bytes == 0
        || budget.maximum_working_bytes > isize::MAX as usize
        || budget.maximum_request_bytes == 0
        || budget.maximum_request_bytes > MAXIMUM_STATE_DELTA_REQUEST_BYTES
        || budget.maximum_chunk_bytes == 0
        || budget.maximum_chunk_bytes > MAXIMUM_STATE_DELTA_CHUNK_BYTES
        || budget.maximum_delta_bytes == 0
        || budget.maximum_delta_bytes > MAXIMUM_STATE_DELTA_PAYLOAD_BYTES
    {
        return Err(DeltaServingError::ResourceLimit);
    }
    Ok(())
}
