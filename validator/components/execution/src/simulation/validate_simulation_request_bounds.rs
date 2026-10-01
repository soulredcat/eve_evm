// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SimulationError, SimulationLimits, SimulationRequest};

/// Check caller-owned list/data before retaining clones or rebuilding state.
pub(super) fn validate_simulation_request_bounds(
    request: &SimulationRequest,
    limits: &SimulationLimits,
) -> Result<(), SimulationError> {
    if request.data.len() > limits.maximum_calldata_bytes {
        return Err(SimulationError::Limit("simulation calldata"));
    }
    if let Some(list) = &request.access_list {
        if list.0.len() > limits.maximum_access_list_entries || list.0.len() > 256 {
            return Err(SimulationError::Limit("simulation access-list entries"));
        }
        let keys = list
            .0
            .iter()
            .try_fold(0_usize, |sum, item| {
                sum.checked_add(item.storage_keys.len())
            })
            .ok_or(SimulationError::Limit(
                "simulation access-list key overflow",
            ))?;
        if keys > limits.maximum_access_list_storage_keys || keys > 1_024 {
            return Err(SimulationError::Limit(
                "simulation access-list storage keys",
            ));
        }
    }
    Ok(())
}
