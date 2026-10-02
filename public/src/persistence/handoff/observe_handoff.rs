// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HandoffError, HandoffObservation, HandoffPool};
use std::time::Duration;

/// Observation includes constructing, queued, in-flight and cloned retained payloads.
pub fn observe_handoff(pool: &HandoffPool) -> Result<HandoffObservation, HandoffError> {
    let accounting = pool
        .accounting
        .lock()
        .map_err(|_| HandoffError::AccountingUnavailable)?;
    Ok(HandoffObservation {
        retained_bytes: accounting.bytes,
        retained_batches: accounting.leases.len() as u64,
        oldest_age: accounting
            .leases
            .values()
            .map(|created| created.elapsed())
            .max()
            .unwrap_or(Duration::ZERO),
    })
}
