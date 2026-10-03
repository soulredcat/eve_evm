// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Public-node allocation/admission policy.

mod budgets;
pub mod segmented;
pub use segmented::{
    SEGMENTED_RECOVERY_POLICY_VERSION, SegmentedRecoveryBounds, SegmentedRecoveryCapacity,
    SegmentedRecoveryPolicy, available_segmented_allowance, development_segmented_recovery_policy,
    validate_segmented_recovery_policy,
};

pub use budgets::{
    BudgetError, PublicBudget, development_public_budget, validate_admission_reservation,
    validate_public_budget,
};
