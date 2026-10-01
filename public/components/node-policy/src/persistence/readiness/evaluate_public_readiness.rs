// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    PersistenceObservation, PublicBudget, PublicReadiness, PublicWatermarks,
    validate_public_budget, validate_watermarks,
};

pub fn evaluate_public_readiness(
    budget: PublicBudget,
    watermarks: PublicWatermarks,
    observation: PersistenceObservation,
) -> PublicReadiness {
    if validate_public_budget(budget).is_err() || validate_watermarks(watermarks).is_err() {
        return PublicReadiness::NotReady("invalid configuration or watermarks");
    }
    if observation.storage_failed {
        return PublicReadiness::NotReady("storage failed");
    }
    if !observation.recoverable_tail_available {
        return PublicReadiness::NotReady("missing recoverable tail");
    }
    if observation.queued_bytes > budget.queue_bytes
        || observation.queued_batches > budget.queue_batches
        || observation.oldest_queue_age_ms > budget.queue_age_ms
    {
        return PublicReadiness::NotReady("persistence queue budget exceeded");
    }
    if watermarks.applied.0 < watermarks.durable_recovery.0 {
        return PublicReadiness::NotReady("working state is recovering");
    }
    if watermarks.applied.0 - watermarks.durable_recovery.0 > budget.maximum_durable_lag_blocks {
        return PublicReadiness::NotReady("durable lag exceeded");
    }
    if watermarks.applied.0 - watermarks.authenticated_state.0
        > budget.maximum_authenticated_lag_blocks
    {
        return PublicReadiness::NotReady("state authentication lag exceeded");
    }
    let Some(head) = observation.independently_verified_head else {
        return PublicReadiness::NotReady("head freshness unknown");
    };
    let serve_height = watermarks.applied.0.min(watermarks.authenticated_state.0);
    if head < watermarks.finalized.0
        || head.saturating_sub(serve_height) > budget.maximum_head_lag_blocks
    {
        return PublicReadiness::NotReady("verified head lag exceeded or inconsistent");
    }
    PublicReadiness::Ready {
        serve_height,
        durable_height: watermarks.durable_recovery.0,
    }
}
