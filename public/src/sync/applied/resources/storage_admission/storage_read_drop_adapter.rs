// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedStorageReadReservation, release_storage_read::release_storage_read};
impl std::ops::Drop for AppliedStorageReadReservation {
    fn drop(&mut self) {
        release_storage_read(&self.pool);
    }
}
