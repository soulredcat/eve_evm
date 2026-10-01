// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SimulationError, SimulationRequest};

pub(super) fn infer_simulation_type(request: &SimulationRequest) -> Result<u8, SimulationError> {
    let dynamic = request.max_fee_per_gas.is_some() || request.max_priority_fee_per_gas.is_some();
    let inferred = if dynamic {
        2
    } else if request.access_list.is_some() {
        1
    } else {
        0
    };
    let kind = request.transaction_type.unwrap_or(inferred);
    if !matches!(kind, 0..=2) {
        return Err(SimulationError::InvalidRequest(
            "unsupported simulation transaction type",
        ));
    }
    if request.gas_price.is_some() && dynamic
        || request.max_priority_fee_per_gas.is_some() && request.max_fee_per_gas.is_none()
        || request
            .max_priority_fee_per_gas
            .zip(request.max_fee_per_gas)
            .is_some_and(|(tip, cap)| tip > cap)
        || kind == 0 && (request.access_list.is_some() || dynamic)
        || kind == 1 && dynamic
        || kind == 2 && request.gas_price.is_some()
    {
        return Err(SimulationError::InvalidRequest(
            "incompatible simulation type or fee fields",
        ));
    }
    Ok(kind)
}
