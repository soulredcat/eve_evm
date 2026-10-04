// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(in crate::cases) fn verify_checkpoint_markers(
    status: &Value,
    checkpoint: u64,
    minimum_durable: u64,
) -> Result<()> {
    let applied = status["applied_height"]
        .as_u64()
        .context("PUBLIC_CHECKPOINT_APPLIED_HEIGHT")?;
    let durable = status["durable_height"]
        .as_u64()
        .context("PUBLIC_CHECKPOINT_DURABLE_HEIGHT")?;
    let finalized = applied
        .checked_add(1)
        .context("PUBLIC_CHECKPOINT_HEIGHT_OVERFLOW")?;
    ensure!(
        checkpoint > 0
            && checkpoint <= minimum_durable
            && minimum_durable <= durable
            && durable <= applied
            && status["authenticated_height"] == applied
            && status["finalized_height"] == finalized
            && status["checkpoint_height"] == checkpoint
            && status["authenticated_snapshot_height"] == checkpoint
            && status["authenticated_finality"] == true
            && status["verification_mode"] == "AUTHENTICATED_IMPORT",
        "PUBLIC_CHECKPOINT_AUTHENTICATED_DURABLE_MARKERS"
    );
    ensure!(
        status["ready"] == false
            && status["readiness_reason"] == "head freshness unknown"
            && status["peer_count"].is_null()
            && status["lag"].is_null()
            && status["storage_failed"] == false,
        "PUBLIC_CHECKPOINT_FALSE_FRESHNESS_OR_STORAGE_OUTCOME"
    );
    ensure!(
        status["durable_record_sequence"]
            .as_u64()
            .is_some_and(|sequence| sequence > 0),
        "PUBLIC_CHECKPOINT_ACTUAL_DURABLE_CURSOR"
    );
    Ok(())
}
