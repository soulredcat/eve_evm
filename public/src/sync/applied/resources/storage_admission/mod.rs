// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Owner-bound read operations and raw snapshot staging; decoded state keeps separate working leases.
mod create_storage_admission_pool;
mod map_storage_admission_error;
mod map_working_control_error;
mod observe_applied_storage;
mod observe_storage_admission;
mod release_snapshot_staging;
mod release_storage_read;
mod required_storage_admission_control_reservation;
mod reserve_applied_snapshot_staging;
mod reserve_applied_storage_read;
mod reserve_checkpoint_storage;
mod reserve_snapshot_staging;
mod reserve_storage_read;
mod snapshot_staging_drop_adapter;
mod storage_read_drop_adapter;
#[cfg(test)]
mod tests;
mod types;
pub(in crate::sync::applied) use create_storage_admission_pool::create_storage_admission_pool;
pub(in crate::sync::applied) use map_storage_admission_error::map_storage_admission_error;
pub use observe_applied_storage::observe_applied_storage;
pub(in crate::sync::applied) use observe_storage_admission::observe_storage_admission;
pub(in crate::sync::applied) use required_storage_admission_control_reservation::required_storage_admission_control_reservation;
pub use reserve_applied_snapshot_staging::reserve_applied_snapshot_staging;
pub use reserve_applied_storage_read::reserve_applied_storage_read;
pub(in crate::sync::applied) use reserve_checkpoint_storage::reserve_checkpoint_storage;
pub(in crate::sync::applied) use reserve_snapshot_staging::reserve_snapshot_staging;
pub(in crate::sync::applied) use reserve_storage_read::reserve_storage_read;
pub use types::{
    AppliedSnapshotStagingReservation, AppliedStorageObservation, AppliedStorageReadReservation,
    StorageAdmissionError,
};
pub(in crate::sync::applied) use types::{
    AppliedStorageResourcePools, CheckpointStorageReservation, StorageAdmissionPool,
};
