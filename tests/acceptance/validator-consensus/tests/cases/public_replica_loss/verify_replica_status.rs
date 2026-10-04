// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn verify_replica_status(status: &Value, minimum_durable: u64) -> Result<()> {
    let applied = status["applied_height"]
        .as_u64()
        .context("PUBLIC_REPLICA_APPLIED_HEIGHT")?;
    let durable = status["durable_height"]
        .as_u64()
        .context("PUBLIC_REPLICA_DURABLE_HEIGHT")?;
    let finalized = applied
        .checked_add(1)
        .context("PUBLIC_REPLICA_HEIGHT_OVERFLOW")?;
    let marker = status["durable_record_sequence"]
        .as_u64()
        .context("PUBLIC_REPLICA_DURABLE_MARKER")?;
    let physical = status["last_acknowledged_physical_sequence"]
        .as_u64()
        .context("PUBLIC_REPLICA_PHYSICAL_ACK")?;
    ensure!(
        minimum_durable > 0
            && minimum_durable <= durable
            && durable <= applied
            && applied <= 128
            && status["authenticated_height"] == applied
            && status["finalized_height"] == finalized
            && status["authenticated_finality"] == true
            && status["verification_mode"] == "AUTHENTICATED_IMPORT"
            && marker > 0
            && physical >= marker
            && status["missing_recovery_from"].is_null(),
        "PUBLIC_REPLICA_VERIFIED_DURABLE_PREFIX"
    );
    ensure!(
        status["ready"] == false
            && status["readiness_reason"] == "head freshness unknown"
            && status["peer_count"].is_null()
            && status["lag"].is_null()
            && status["storage_failed"] == false,
        "PUBLIC_REPLICA_FALSE_FRESHNESS_OR_STORAGE_OUTCOME"
    );
    ensure!(
        status["checkpoint_height"] == 0 && status["authenticated_snapshot_height"] == 0,
        "PUBLIC_REPLICA_UNREQUESTED_CHECKPOINT"
    );
    Ok(())
}
