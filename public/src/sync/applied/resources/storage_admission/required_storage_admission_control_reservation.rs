// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{StorageAdmissionError, types::StorageAdmissionPool};

/// One canonical exact logical control allocation: pool body plus Arc reference counters.
/// This remains reserved while any reader, IO handle or artifact retains the controller.
pub(in crate::sync::applied) fn required_storage_admission_control_reservation()
-> Result<usize, StorageAdmissionError> {
    std::mem::size_of::<StorageAdmissionPool>()
        .checked_add(2 * std::mem::size_of::<usize>())
        .ok_or(StorageAdmissionError::Overflow)
}
