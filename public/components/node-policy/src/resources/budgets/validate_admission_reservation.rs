// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{PublicBudget, validate_public_budget};

/// Check global and per-request bounds before allocating payload buffers.
/// The caller must atomically reserve/release these bytes; this pure check does
/// not claim an implemented concurrent admission controller.
pub fn validate_admission_reservation(
    budget: PublicBudget,
    bulk: bool,
    reserved_bytes: u64,
    requested_bytes: u64,
) -> Result<u64, &'static str> {
    validate_public_budget(budget).map_err(|_| "invalid admission budget")?;
    let (request_limit, global_limit) = if bulk {
        (
            budget.per_peer_inflight_bytes,
            budget.global_bulk_inflight_bytes,
        )
    } else {
        (
            budget.ordinary_envelope_bytes,
            budget.global_ordinary_inflight_bytes,
        )
    };
    if requested_bytes == 0 || requested_bytes > request_limit {
        return Err("request byte limit exceeded");
    }
    let next = reserved_bytes
        .checked_add(requested_bytes)
        .ok_or("admission byte overflow")?;
    if next > global_limit {
        return Err("global admission byte limit exceeded");
    }
    Ok(next)
}
