// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Explicit replay/import capabilities with estimated bounded public application.
//! Compact and segmented persistence share one owner; full B4 and peer recovery remain unfinished.

mod admission;
pub mod checkpoints;
mod durability;
mod opening;
mod publication;
mod recovery;
mod resources;
pub(crate) mod segmented;
mod shutdown;
mod state;
#[cfg(test)]
pub(crate) mod tests;
mod types;

pub use admission::{try_apply_recovery_bytes, try_apply_recovery_bytes_matching_target};
pub use durability::poll_applied_durability;
pub use opening::{open_applied_state_service, open_applied_state_service_with_mode};
pub use publication::{
    applied_anchor, applied_commit, applied_cursors, applied_markers, applied_mode,
    applied_owner_reader, applied_owner_state_budget, applied_storage_failed,
    capture_applied_state,
};
pub use publication::{applied_readiness, applied_segmented_position};
pub use resources::storage_admission::{
    AppliedSnapshotStagingReservation, AppliedStorageObservation, AppliedStorageReadReservation,
    StorageAdmissionError, observe_applied_storage, reserve_applied_snapshot_staging,
    reserve_applied_storage_read,
};
pub use resources::{
    AppliedWorkingReservation, EstimatedWorkingObservation, observe_estimated_working,
    reserve_applied_working,
};
pub use segmented::{
    SegmentedAppliedConfig, SegmentedAppliedPosition, SegmentedTailProgress,
    open_segmented_applied_state_service, retained_tail_part_bytes, retained_tail_progress,
};
pub use shutdown::{finish_applied_state_service, retained_tail_bytes, retained_tail_len};
pub use types::{
    AppliedAdmission, AppliedConfig, AppliedError, AppliedMode, AppliedOwner, AppliedPublication,
    AppliedReader, AppliedShutdown, RetainedAppliedTail,
};
