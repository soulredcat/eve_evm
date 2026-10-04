// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DurablePrefix;
use anyhow::{Context, Result, ensure};
use serde_json::Value;

/// The product status observes an acknowledged complete logical marker. This
/// observation does not manufacture freshness or a stronger physical fault model.
pub(super) fn capture_durable_prefix(status: &Value, minimum_height: u64) -> Result<DurablePrefix> {
    let height = status["durable_height"]
        .as_u64()
        .context("PUBLIC_TAIL_DURABLE_HEIGHT")?;
    let applied = status["applied_height"]
        .as_u64()
        .context("PUBLIC_TAIL_APPLIED_HEIGHT")?;
    let marker_sequence = status["durable_record_sequence"]
        .as_u64()
        .context("PUBLIC_TAIL_DURABLE_MARKER")?;
    let acknowledged_physical_sequence = status["last_acknowledged_physical_sequence"]
        .as_u64()
        .context("PUBLIC_TAIL_PHYSICAL_ACK")?;
    let finalized = applied
        .checked_add(1)
        .context("PUBLIC_TAIL_HEIGHT_OVERFLOW")?;
    ensure!(
        height >= minimum_height
            && height > 0
            && height <= applied
            && status["authenticated_height"] == applied
            && status["finalized_height"] == finalized
            && status["authenticated_finality"] == true
            && marker_sequence > 0
            && acknowledged_physical_sequence >= marker_sequence,
        "PUBLIC_TAIL_FALSE_DURABLE_OR_AUTHENTICATED_PREFIX"
    );
    ensure!(
        status["ready"] == false
            && status["readiness_reason"] == "head freshness unknown"
            && status["peer_count"].is_null()
            && status["lag"].is_null()
            && status["storage_failed"] == false,
        "PUBLIC_TAIL_FALSE_FRESHNESS_OR_STORAGE_OUTCOME"
    );
    Ok(DurablePrefix {
        height,
        marker_sequence,
        acknowledged_physical_sequence,
    })
}
