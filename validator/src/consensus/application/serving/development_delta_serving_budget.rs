// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DeltaServingBudget;
use eve_state::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES,
    MAXIMUM_STATE_DELTA_REQUEST_BYTES,
};

pub(in crate::consensus) fn development_delta_serving_budget() -> DeltaServingBudget {
    DeltaServingBudget {
        maximum_working_bytes: 64 * 1_048_576,
        maximum_request_bytes: MAXIMUM_STATE_DELTA_REQUEST_BYTES,
        maximum_chunk_bytes: MAXIMUM_STATE_DELTA_CHUNK_BYTES,
        maximum_delta_bytes: MAXIMUM_STATE_DELTA_PAYLOAD_BYTES,
    }
}
