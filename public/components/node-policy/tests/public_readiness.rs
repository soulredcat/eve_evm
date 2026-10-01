// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_node_policy::{
    AppliedHeight, AuthenticatedStateHeight, BudgetError, CheckpointHeight, DurableRecoveryHeight,
    FinalizedHeight, PersistenceObservation, PublicReadiness, PublicWatermarks,
    StateAuthentication, development_public_budget, evaluate_public_readiness,
    validate_public_budget, validate_watermarks,
};

fn watermarks() -> PublicWatermarks {
    PublicWatermarks {
        finalized: FinalizedHeight(10),
        applied: AppliedHeight(10),
        durable_recovery: DurableRecoveryHeight(8),
        checkpoint: CheckpointHeight(5),
        authenticated_state: AuthenticatedStateHeight(9),
        authentication: StateAuthentication::NextCertifiedHeader { header_height: 10 },
        authenticated_snapshot_height: 5,
        oldest_retained_height: 1,
    }
}

fn observation() -> PersistenceObservation {
    PersistenceObservation {
        queued_bytes: 1,
        queued_batches: 1,
        oldest_queue_age_ms: 1,
        storage_failed: false,
        recoverable_tail_available: true,
        independently_verified_head: Some(10),
    }
}

#[test]
fn serves_only_authenticated_applied_height_with_truthful_durable_watermark() {
    let budget = development_public_budget();
    assert!(validate_public_budget(budget).is_ok());
    assert!(validate_watermarks(watermarks()).is_ok());
    assert_eq!(
        evaluate_public_readiness(budget, watermarks(), observation()),
        PublicReadiness::Ready {
            serve_height: 9,
            durable_height: 8
        }
    );
    let mut replayed = watermarks();
    replayed.authenticated_state = AuthenticatedStateHeight(10);
    replayed.authentication = StateAuthentication::VerifiedReplay;
    assert_eq!(
        evaluate_public_readiness(budget, replayed, observation()),
        PublicReadiness::Ready {
            serve_height: 10,
            durable_height: 8
        }
    );
}

#[test]
fn rejects_wrong_anchor_incomplete_recovery_storage_failure_queue_and_unknown_freshness() {
    let budget = development_public_budget();
    let mut state = watermarks();
    state.authentication = StateAuthentication::NextCertifiedHeader { header_height: 9 };
    assert!(validate_watermarks(state).is_err());
    state = watermarks();
    state.checkpoint = CheckpointHeight(9);
    assert!(validate_watermarks(state).is_err());
    let mut obs = observation();
    obs.storage_failed = true;
    assert_eq!(
        evaluate_public_readiness(budget, watermarks(), obs),
        PublicReadiness::NotReady("storage failed")
    );
    obs = observation();
    obs.recoverable_tail_available = false;
    assert_eq!(
        evaluate_public_readiness(budget, watermarks(), obs),
        PublicReadiness::NotReady("missing recoverable tail")
    );
    obs = observation();
    obs.queued_bytes = budget.queue_bytes + 1;
    assert_eq!(
        evaluate_public_readiness(budget, watermarks(), obs),
        PublicReadiness::NotReady("persistence queue budget exceeded")
    );
    obs = observation();
    obs.independently_verified_head = None;
    assert_eq!(
        evaluate_public_readiness(budget, watermarks(), obs),
        PublicReadiness::NotReady("head freshness unknown")
    );
}

#[test]
fn rejects_zero_inconsistent_memory_capacity_and_overflow_budgets() {
    let mut budget = development_public_budget();
    budget.queue_bytes = 0;
    assert_eq!(
        validate_public_budget(budget),
        Err(BudgetError::InvalidLimit)
    );
    budget = development_public_budget();
    budget.queue_batches = 5;
    assert_eq!(
        validate_public_budget(budget),
        Err(BudgetError::InconsistentBound)
    );
    budget = development_public_budget();
    budget.process_memory_budget_bytes = 1;
    assert_eq!(
        validate_public_budget(budget),
        Err(BudgetError::InconsistentBound)
    );
    budget = development_public_budget();
    budget.write_buffer_bytes = u64::MAX;
    assert_eq!(
        validate_public_budget(budget),
        Err(BudgetError::ArithmeticOverflow)
    );
}
