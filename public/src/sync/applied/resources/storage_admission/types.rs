// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::{Arc, Mutex};

pub(in crate::sync::applied) struct StorageAdmissionPool {
    pub(super) reads_limit: u64,
    pub(super) staging_limit: u64,
    pub(super) accounting: Mutex<StorageAdmissionAccounting>,
    pub(super) _control: super::super::EstimatedWorkingLease,
}
pub(in crate::sync::applied) struct AppliedStorageResourcePools {
    pub(in crate::sync::applied) working: Arc<super::super::EstimatedWorkingPool>,
    pub(in crate::sync::applied) storage: Arc<StorageAdmissionPool>,
}
pub(in crate::sync::applied) struct CheckpointStorageReservation {
    pub(super) _read: AppliedStorageReadReservation,
    pub(super) _staging: AppliedSnapshotStagingReservation,
}
pub(super) struct StorageAdmissionAccounting {
    pub(super) reads: u64,
    pub(super) peak_reads: u64,
    pub(super) staging: u64,
    pub(super) peak_staging: u64,
}
/// Same-owner storage operation slot; no state or finality authority.
pub struct AppliedStorageReadReservation {
    pub(super) pool: Arc<StorageAdmissionPool>,
}
/// Explicit raw buffers/manifest/scratch capacity, separate from decoded working state.
pub struct AppliedSnapshotStagingReservation {
    pub(super) pool: Arc<StorageAdmissionPool>,
    pub(super) bytes: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageAdmissionError {
    InvalidConfiguration,
    AccountingUnavailable,
    ReadCapacity,
    StagingCapacity,
    WorkingCapacity,
    Overflow,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppliedStorageObservation {
    pub active_reads: u64,
    pub peak_reads: u64,
    pub read_limit: u64,
    pub reserved_staging_bytes: u64,
    pub peak_staging_bytes: u64,
    pub staging_limit: u64,
}
