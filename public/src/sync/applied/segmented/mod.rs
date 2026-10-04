// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod admit_segmented_publication;
mod bind_local_state_version;
mod open_segmented_applied_state_service;
mod recover_segmented_applied_prefix;
mod retained_tail_part_bytes;
mod retained_tail_progress;
#[cfg(test)]
pub(crate) mod tests;
mod try_apply_segmented_import;
mod types;
mod validate_segmented_applied_ack;
pub(super) use bind_local_state_version::bind_local_state_version;
pub use open_segmented_applied_state_service::open_segmented_applied_state_service;
pub use retained_tail_part_bytes::retained_tail_part_bytes;
pub use retained_tail_progress::retained_tail_progress;
pub(super) use try_apply_segmented_import::try_apply_segmented_import;
pub use types::{SegmentedAppliedConfig, SegmentedAppliedPosition, SegmentedTailProgress};
pub(super) use validate_segmented_applied_ack::validate_segmented_applied_ack;
