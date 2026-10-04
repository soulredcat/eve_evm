// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointStorageReservation, StorageAdmissionPool, map_storage_admission_error,
    reserve_snapshot_staging, reserve_storage_read,
};
use crate::sync::applied::AppliedError;
use std::sync::Arc;
pub(in crate::sync::applied) fn reserve_checkpoint_storage(
    storage: &Arc<StorageAdmissionPool>,
    raw_bytes: usize,
) -> Result<CheckpointStorageReservation, AppliedError> {
    let read = reserve_storage_read(storage).map_err(map_storage_admission_error)?;
    let staging =
        reserve_snapshot_staging(storage, raw_bytes).map_err(map_storage_admission_error)?;
    Ok(CheckpointStorageReservation {
        _read: read,
        _staging: staging,
    })
}
