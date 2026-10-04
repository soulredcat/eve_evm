// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod acquire_segmented_worker;
mod create_segmented_part_pool;
mod estimate_segmented_metadata;
mod metadata_drop_adapter;
mod observe_segmented_parts;
mod part_drop_adapter;
mod release_metadata;
mod release_part;
mod release_segmented_worker;
mod reserve_metadata;
mod worker_lifetime_drop_adapter;
pub(super) use acquire_segmented_worker::acquire_segmented_worker;
pub use create_segmented_part_pool::create_segmented_part_pool;
pub(super) use estimate_segmented_metadata::estimate_segmented_metadata;
pub use observe_segmented_parts::observe_segmented_parts;
pub(super) use reserve_metadata::reserve_metadata;

mod required_segmented_metadata_reservation;
pub use required_segmented_metadata_reservation::required_segmented_metadata_reservation;
