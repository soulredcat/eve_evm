// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Version 2 retained-part policy, separate from version 1 physical batch semantics.

mod available_segmented_allowance;
mod derive_segmented_recovery_capacity;
mod development_segmented_recovery_policy;
mod types;
mod validate_segmented_recovery_policy;

pub use available_segmented_allowance::available_segmented_allowance;
pub use development_segmented_recovery_policy::development_segmented_recovery_policy;
pub use types::{
    SEGMENTED_RECOVERY_POLICY_VERSION, SegmentedRecoveryBounds, SegmentedRecoveryCapacity,
    SegmentedRecoveryPolicy,
};
pub use validate_segmented_recovery_policy::validate_segmented_recovery_policy;
