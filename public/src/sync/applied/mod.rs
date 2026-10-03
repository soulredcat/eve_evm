// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Estimated, bounded local empty-block follower slice; complete B4 remains unfinished.

mod admission;
mod durability;
mod opening;
mod publication;
mod recovery;
mod resources;
mod shutdown;
#[cfg(test)]
mod tests;
mod types;

pub use admission::try_apply_recovery_bytes;
pub use durability::poll_applied_durability;
pub use opening::open_applied_state_service;
pub use publication::{
    applied_anchor, applied_commit, applied_cursors, applied_markers, applied_storage_failed,
    capture_applied_state,
};
pub use resources::{EstimatedWorkingObservation, observe_estimated_working};
pub use shutdown::{finish_applied_state_service, retained_tail_bytes, retained_tail_len};
pub use types::{
    AppliedAdmission, AppliedConfig, AppliedError, AppliedOwner, AppliedPublication, AppliedReader,
    AppliedShutdown, RetainedAppliedTail,
};
