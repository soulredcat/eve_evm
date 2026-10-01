// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Public-owned development budgets, watermarks, readiness and source policy.

pub mod persistence;
pub mod resources;
pub mod sources;

pub use persistence::{
    AppliedHeight, AuthenticatedStateHeight, CheckpointHeight, DurableRecoveryHeight,
    FinalizedHeight, PersistenceObservation, PublicReadiness, PublicWatermarks,
    StateAuthentication, evaluate_public_readiness, validate_watermarks,
};
pub use resources::{
    BudgetError, PublicBudget, development_public_budget, validate_admission_reservation,
    validate_public_budget,
};
pub use sources::{
    SourceEligibility, SourceObservation, SourcePurpose, SourceVerification, ZoneId,
    evaluate_source, select_preferred_source, should_switch_source,
};
