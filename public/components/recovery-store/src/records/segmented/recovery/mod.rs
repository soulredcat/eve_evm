// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded actual-row recovery. Complete local integrity still needs public proof verification.

mod accept_segmented_recovery;
mod append_segmented_recovery_candidate;
mod begin_segmented_checkpoint_recovery;
mod begin_segmented_recovery;
mod complete_segmented_recovery_candidate;
mod required_segmented_checkpoint_recovery_reservation;
mod required_segmented_recovery_reservation;
mod scan_next_segmented_recovery;
mod segmented_recovery_bundle_anchor;
mod segmented_recovery_bundle_bytes;
mod segmented_recovery_bundle_capacity;
mod segmented_recovery_bundle_references;
mod segmented_recovery_observation;
mod types;
mod verify_segmented_recovery_anchor;

#[cfg(test)]
#[path = "tests/head_read_failure.rs"]
mod head_read_failure;

pub use accept_segmented_recovery::accept_segmented_recovery;
pub use begin_segmented_checkpoint_recovery::begin_segmented_checkpoint_recovery;
pub use begin_segmented_recovery::begin_segmented_recovery;
pub use required_segmented_checkpoint_recovery_reservation::required_segmented_checkpoint_recovery_reservation;
pub use required_segmented_recovery_reservation::required_segmented_recovery_reservation;
pub use scan_next_segmented_recovery::scan_next_segmented_recovery;
pub use segmented_recovery_bundle_anchor::segmented_recovery_bundle_anchor;
pub use segmented_recovery_bundle_bytes::segmented_recovery_bundle_bytes;
pub use segmented_recovery_bundle_capacity::segmented_recovery_bundle_capacity;
pub use segmented_recovery_bundle_references::segmented_recovery_bundle_references;
pub use segmented_recovery_observation::segmented_recovery_observation;
pub use types::{
    RecoveredSegmentedBundle, RejectedSegmentedRecovery, SegmentedRecoveryError,
    SegmentedRecoveryObservation, SegmentedRecoveryScan, SegmentedRecoveryStep,
};
